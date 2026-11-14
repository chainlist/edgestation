use windows::Win32::Foundation::S_OK;
use windows::Win32::Media::Audio::{AudioSessionStateActive, IMMDevice, ISimpleAudioVolume};
use windows::Win32::Storage::FileSystem::QueryDosDeviceW;
use windows::Win32::UI::Shell::SHLoadIndirectString;
use windows::core::{HSTRING, Interface, Result};

use super::sessions::sessions;
use crate::audio::App;
use crate::os::windows::com::take;
use crate::process::{self, image_path};

/// The apps on `output`, one for each group of sessions.
pub(super) unsafe fn apps(output: &IMMDevice) -> Result<Vec<App>> {
    unsafe {
        let drives = drives();
        let mut apps: Vec<App> = Vec::new();
        let mut system = None;
        for (id, session) in sessions(output)? {
            let volume: ISimpleAudioVolume = session.cast()?;
            let muted = volume.GetMute()?.as_bool();
            let active = session.GetState()? == AudioSessionStateActive;
            if let Some(app) = apps.iter_mut().chain(system.as_mut()).find(|app| app.id == id) {
                app.muted &= muted;
                app.active |= active;
                continue;
            }
            let is_system = session.IsSystemSoundsSession() == S_OK;
            let kernel_path = match image_path(session.GetProcessId()? as usize) {
                Some(path) => Some(path),
                // The proccess has exited, so the session ends soon.
                None if !is_system => continue,
                None => None,
            };
            let given = take(session.GetDisplayName()?)?.and_then(|name| indirect(&name));
            let name = match (given, &kernel_path) {
                (Some(name), _) => name,
                (None, Some(path)) => process::display_name(path),
                (None, None) => String::new(),
            };
            let kernel_path = kernel_path.filter(|_| !is_system);
            let app = App {
                id,
                name,
                path: kernel_path.as_deref().and_then(|path| dos_path(&drives, path)),
                program: kernel_path.as_deref().map(|path| process::file_name(path).to_owned()),
                volume: volume.GetMasterVolume()?,
                muted,
                active,
            };
            if is_system {
                system = Some(app);
            } else {
                apps.push(app);
            }
        }
        apps.sort_by_cached_key(|app| app.name.to_lowercase());
        apps.extend(system);
        Ok(apps)
    }
}

/// The name that an app gave its session, or `None` if it gave none. If the name points to the
/// resources of a file, for example `@%SystemRoot%\System32\AudioSrv.Dll,-202`, the text comes
/// from that file.
fn indirect(name: &str) -> Option<String> {
    let name = name.trim();
    if !name.starts_with('@') {
        return (!name.is_empty()).then(|| name.to_owned());
    }
    let mut text = [0u16; 512];
    unsafe { SHLoadIndirectString(&HSTRING::from(name), &mut text, None).ok()? };
    let length = text.iter().position(|&c| c == 0).unwrap_or(text.len());
    Some(String::from_utf16_lossy(&text[..length])).filter(|text| !text.is_empty())
}

/// Each drive letter with its kernel device, for example `C:` and `\Device\HarddiskVolume3`.
fn drives() -> Vec<(String, String)> {
    let mut drives = Vec::new();
    for letter in 'A'..='Z' {
        let drive = format!("{letter}:");
        let mut target = [0u16; 512];
        let length = unsafe { QueryDosDeviceW(&HSTRING::from(&drive), Some(&mut target)) };
        // The buffer can hold more than one target, each with a NUL at the end. The first
        // target is the current one.
        let first = target[..length as usize].split(|&c| c == 0).next().unwrap_or_default();
        if !first.is_empty() {
            drives.push((drive, String::from_utf16_lossy(first)));
        }
    }
    drives
}

/// Changes kernel path `path` (`\Device\HarddiskVolume3\Program Files\...`) to a path with a
/// drive letter (`C:\Program Files\...`). Returns `None` if no drive letter points to teh device.
fn dos_path(drives: &[(String, String)], path: &str) -> Option<String> {
    drives.iter().find_map(|(drive, device)| {
        let rest = path.strip_prefix(device.as_str())?;
        rest.starts_with('\\').then(|| format!("{drive}{rest}"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puts_a_drive_letter_on_kernel_paths() {
        let drives = [
            ("C:".to_owned(), r"\Device\HarddiskVolume1".to_owned()),
            ("D:".to_owned(), r"\Device\HarddiskVolume10".to_owned()),
        ];
        let path = |path| dos_path(&drives, path);
        assert_eq!(path(r"\Device\HarddiskVolume1\a.exe").as_deref(), Some(r"C:\a.exe"));
        assert_eq!(path(r"\Device\HarddiskVolume10\b.exe").as_deref(), Some(r"D:\b.exe"));
        assert_eq!(path(r"\Device\HarddiskVolume2\c.exe"), None);
    }

    #[test]
    fn reads_names_from_resources() {
        assert_eq!(indirect("  Zen  ").as_deref(), Some("Zen"));
        assert_eq!(indirect(""), None);
        let system = indirect(r"@%SystemRoot%\System32\AudioSrv.Dll,-202");
        assert!(system.is_some_and(|name| !name.starts_with('@')));
    }
}
