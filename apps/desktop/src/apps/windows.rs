//! The Windows side. It uses the Apps folder through the COM interfaces of the shell. The Start
//! menu lists, starts and draws its apps the same way.

use std::collections::HashSet;
use std::mem::size_of;
use std::thread;
use std::time::Duration;

use tauri::{Runtime, Window};
use windows::Win32::Foundation::{HANDLE, POINT, SIZE};
use windows::Win32::Graphics::Gdi::{
    BI_RGB, BITMAP, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, DeleteObject, GetDC, GetDIBits,
    GetObjectW, HBITMAP, MONITOR_DEFAULTTONEAREST, MONITOR_DEFAULTTOPRIMARY, MonitorFromPoint,
    MonitorFromWindow, ReleaseDC,
};
use windows::Win32::UI::Shell::{
    BHID_EnumItems, FOLDERID_AppsFolder, IEnumShellItems, ILFree, IShellItem,
    IShellItemImageFactory, KF_FLAG_DEFAULT, SEE_MASK_FLAG_NO_UI, SEE_MASK_HMONITOR,
    SEE_MASK_INVOKEIDLIST, SEE_MASK_NOASYNC, SHCreateItemFromParsingName, SHELLEXECUTEINFOW,
    SHGetIDListFromObject, SHGetKnownFolderItem, SIGDN, SIGDN_NORMALDISPLAY,
    SIGDN_PARENTRELATIVEPARSING, SIIGBF_ICONONLY, ShellExecuteExW,
};
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
use windows::core::{HRESULT, HSTRING, Result};

use super::App;
use crate::os::windows::com::{self, Apartment, take};

/// "The data necessary to complete this operation is not yet available". The SDK defines it only
/// with the URL monikers.
const E_PENDING: HRESULT = HRESULT(0x8000_000A_u32 as i32);
/// How many times, and at what intervall, to ask again for an icon that the shell cannot draw yet.
const PENDING_TRIES: u32 = 40;
const PENDING_WAIT: Duration = Duration::from_millis(25);

/// Lists every app of the Apps folder (All apps in the Start menu), sorted by name.
pub fn list() -> std::result::Result<Vec<App>, String> {
    with_com(|| -> Result<Vec<App>> {
        unsafe {
            let folder: IShellItem =
                SHGetKnownFolderItem(&FOLDERID_AppsFolder, KF_FLAG_DEFAULT, None)?;
            let items: IEnumShellItems = folder.BindToHandler(None, &BHID_EnumItems)?;
            let mut seen = HashSet::new();
            let mut apps = Vec::new();
            loop {
                let mut next = [None];
                items.Next(&mut next, None)?;
                let Some(item) = next[0].take() else { break };
                let id = name_of(&item, SIGDN_PARENTRELATIVEPARSING);
                let name = name_of(&item, SIGDN_NORMALDISPLAY);
                if let (Some(id), Some(name)) = (id, name)
                    && seen.insert(id.clone())
                {
                    apps.push(App { id, name });
                }
            }
            apps.sort_by_cached_key(|app| app.name.to_lowercase());
            Ok(apps)
        }
    })
    .map_err(|error| error.to_string())
}

/// The screen for launched apps. This is the main screen, unless the Edge (the screen of
/// `window`) is the main screen. Then Windows decides. Returns the handle as `isize`, because
/// that type can cross threads.
pub fn away_from<R: Runtime>(window: &Window<R>) -> Option<isize> {
    let edge = unsafe { MonitorFromWindow(window.hwnd().ok()?, MONITOR_DEFAULTTONEAREST) };
    let main = unsafe { MonitorFromPoint(POINT::default(), MONITOR_DEFAULTTOPRIMARY) };
    (main != edge).then_some(main.0 as isize)
}

/// Starts app `id` with its default action, as the Start menu does, on `screen` if it is set.
/// This is only a hint. Apps that remeber their position can use that position.
pub fn launch(id: &str, screen: Option<isize>) -> std::result::Result<(), String> {
    with_com(|| -> Result<()> {
        unsafe {
            let item: IShellItem = SHCreateItemFromParsingName(&item_path(id), None)?;
            let pidl = SHGetIDListFromObject(&item)?;
            let mut info = SHELLEXECUTEINFOW {
                cbSize: size_of::<SHELLEXECUTEINFOW>() as u32,
                // No error dialog, because it opens on the Edge. The widget shows the failure.
                fMask: SEE_MASK_INVOKEIDLIST | SEE_MASK_NOASYNC | SEE_MASK_FLAG_NO_UI,
                lpIDList: pidl.cast(),
                nShow: SW_SHOWNORMAL.0,
                ..Default::default()
            };
            if let Some(screen) = screen {
                info.fMask |= SEE_MASK_HMONITOR;
                info.Anonymous.hMonitor = HANDLE(screen as _);
            }
            let launched = ShellExecuteExW(&mut info);
            ILFree(Some(pidl));
            launched
        }
    })
    .map_err(|error| error.to_string())
}

/// The icon of app `id` as a square PNG of `size` pixels, or `None` if the app is gone.
pub fn icon(id: &str, size: u32) -> Option<Vec<u8>> {
    image(&item_path(id), size)
}

/// The icon of the executable at `path`, like `icon`, only on a local drive. For other paths
/// (`\\server\share\...`), Windows goes to the netwrok.
pub fn executable_icon(path: &str, size: u32) -> Option<Vec<u8>> {
    let local = matches!(path.as_bytes(), [drive, b':', b'\\', ..] if drive.is_ascii_alphabetic());
    let executable = path.to_ascii_lowercase().ends_with(".exe");
    (local && executable).then(|| image(&HSTRING::from(path), size)).flatten()
}

/// The icon of the shell item named `name`, like `icon`.
fn image(name: &HSTRING, size: u32) -> Option<Vec<u8>> {
    with_com(|| unsafe {
        let factory: IShellItemImageFactory = SHCreateItemFromParsingName(name, None).ok()?;
        let side = SIZE { cx: size as i32, cy: size as i32 };
        // While another thread draws an icon, the shell can anwser that this icon is not ready.
        // It does not wait. When the Edge opens, several widgets ask for icons at the same time.
        let mut tries = 0;
        let bitmap = loop {
            match factory.GetImage(side, SIIGBF_ICONONLY) {
                Err(error) if error.code() == E_PENDING && tries < PENDING_TRIES => {
                    tries += 1;
                    thread::sleep(PENDING_WAIT);
                }
                result => break result.ok()?,
            }
        };
        let png = encode(bitmap);
        let _ = DeleteObject(bitmap.into());
        png
    })
}

/// Runs `f` with COM set up on this thread for the call, in the single-threaded apartment that
/// the shell needs. Release every COM object inside `f`.
fn with_com<T>(f: impl FnOnce() -> T) -> T {
    com::with(Apartment::SingleThreaded, f)
}

fn item_path(id: &str) -> HSTRING {
    HSTRING::from(format!("shell:AppsFolder\\{id}"))
}

unsafe fn name_of(item: &IShellItem, kind: SIGDN) -> Option<String> {
    unsafe { take(item.GetDisplayName(kind).ok()?).ok().flatten() }
}

/// Encodes `bitmap`, a 32-bit image from the shell, as PNG.
unsafe fn encode(bitmap: HBITMAP) -> Option<Vec<u8>> {
    unsafe {
        let mut info = BITMAP::default();
        let read = GetObjectW(
            bitmap.into(),
            size_of::<BITMAP>() as i32,
            Some((&raw mut info).cast()),
        );
        let (width, height) = (info.bmWidth, info.bmHeight);
        if read == 0 || width <= 0 || height <= 0 {
            return None;
        }
        let mut format = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width,
                // A negative height gives the rows from top to bottom, as in PNG.
                biHeight: -height,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut pixels = vec![0u8; width as usize * height as usize * 4];
        let dc = GetDC(None);
        let rows = GetDIBits(
            dc,
            bitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr().cast()),
            &mut format,
            DIB_RGB_COLORS,
        );
        ReleaseDC(None, dc);
        if rows != height {
            return None;
        }
        to_rgba(&mut pixels);

        let mut png = Vec::new();
        let mut encoder = png::Encoder::new(&mut png, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().ok()?;
        writer.write_image_data(&pixels).ok()?;
        writer.finish().ok()?;
        Some(png)
    }
}

/// Changes the BGRA pixels of the shell into RGBA. Their alpha is straight (not premultiplied),
/// as PNG needs. An image without alpha is opaque. Old icons have this, with each alpha at 0.
fn to_rgba(pixels: &mut [u8]) {
    let opaque = pixels.as_chunks::<4>().0.iter().all(|pixel| pixel[3] == 0);
    for pixel in pixels.as_chunks_mut::<4>().0 {
        pixel.swap(0, 2);
        if opaque {
            pixel[3] = 255;
        }
    }
}

#[cfg(test)]
mod tests {
    use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
    use tauri::http::StatusCode;

    use super::{executable_icon, list, to_rgba};
    use crate::apps::icon_response;

    #[test]
    fn swaps_blue_and_red_and_keeps_alpha() {
        let mut pixels = [1, 2, 3, 0, 10, 20, 30, 128];
        to_rgba(&mut pixels);
        assert_eq!(pixels, [3, 2, 1, 0, 30, 20, 10, 128]);
    }

    #[test]
    fn makes_an_image_without_alpha_opaque() {
        let mut pixels = [1, 2, 3, 0, 10, 20, 30, 0];
        to_rgba(&mut pixels);
        assert_eq!(pixels, [3, 2, 1, 255, 30, 20, 10, 255]);
    }


    #[test]
    fn serves_an_installed_app_icon_as_png() {
        let app = &list().unwrap()[0];
        let id = utf8_percent_encode(&app.id, NON_ALPHANUMERIC);
        let response = icon_response(&format!("/48/{id}"));
        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.body().starts_with(b"\x89PNG"));
    }

    #[test]
    fn serves_an_executable_icon_as_png() {
        let path = std::env::var("SystemRoot").unwrap() + r"\explorer.exe";
        let path = utf8_percent_encode(&path, NON_ALPHANUMERIC);
        let response = icon_response(&format!("/file/48/{path}"));
        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.body().starts_with(b"\x89PNG"));
    }

    /// The shell answers "not ready yet" to some icons when many are asked for at the same time.
    #[test]
    fn draws_icons_asked_for_at_once() {
        let root = std::env::var("SystemRoot").unwrap();
        let names = [
            "explorer",
            "notepad",
            "regedit",
            r"System32\cmd",
            r"System32\taskmgr",
            r"System32\mstsc",
            r"System32\charmap",
        ];
        let threads: Vec<_> = (0..4)
            .flat_map(|_| names)
            .map(|name| {
                let path = format!(r"{root}\{name}.exe");
                std::thread::spawn(move || (executable_icon(&path, 48).is_some(), path))
            })
            .collect();
        for thread in threads {
            let (drawn, path) = thread.join().unwrap();
            assert!(drawn, "{path}");
        }
    }

    #[test]
    fn answers_not_found_for_unknown_apps_and_bad_files() {
        for path in [
            "/48/No.Such.App%21App",
            // Only executables, only on a local drive.
            "/file/48/C%3A%5CWindows%5Cwin.ini",
            "/file/48/%5C%5Cserver%5Cshare%5Ca.exe",
            "/file/48/C%3A%5CNo%5CSuch.exe",
        ] {
            assert_eq!(
                icon_response(path).status(),
                StatusCode::NOT_FOUND,
                "{path}"
            );
        }
    }
}
