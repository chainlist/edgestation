//! The Windows side.

/// The kernel path of the executable of a process, for example `\Device\HarddiskVolume3\...`.
/// The query uses the process id, so it also works for processes that the app cannot open, for
/// example an game with an anti-cheat.
pub fn image_path(pid: usize) -> Option<String> {
    use windows::Wdk::System::SystemInformation::{
        NtQuerySystemInformation, SYSTEM_INFORMATION_CLASS,
    };
    use windows::Win32::Foundation::UNICODE_STRING;
    use windows::core::PWSTR;

    /// `SystemProcessIdInformation` and its `SYSTEM_PROCESS_ID_INFORMATION`. The SDK does not
    /// have them.
    const CLASS: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(88);
    #[repr(C)]
    struct Query {
        process_id: usize,
        image_name: UNICODE_STRING,
    }

    let mut path = [0u16; 1024];
    let mut query = Query {
        process_id: pid,
        image_name: UNICODE_STRING {
            Length: 0,
            MaximumLength: size_of_val(&path) as u16,
            Buffer: PWSTR(path.as_mut_ptr()),
        },
    };
    let status = unsafe {
        let size = size_of::<Query>() as u32;
        NtQuerySystemInformation(CLASS, (&raw mut query).cast(), size, std::ptr::null_mut())
    };
    let length = query.image_name.Length as usize / 2;
    (status.is_ok() && length > 0).then(|| String::from_utf16_lossy(&path[..length]))
}

/// The description in the version information of the executable at kernel path `path`, in its
/// first language. Task Manager shows this name.
pub fn program_name(path: &str) -> Option<String> {
    use windows::Win32::Storage::FileSystem::{
        GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW,
    };
    use windows::core::HSTRING;

    // File functions can open a kernel path with this prefix.
    let path = HSTRING::from(format!(r"\\?\GLOBALROOT{path}"));
    unsafe {
        let size = GetFileVersionInfoSizeW(&path, None);
        let mut info = vec![0u8; size as usize];
        GetFileVersionInfoW(&path, None, size, info.as_mut_ptr().cast()).ok()?;
        let value = |name: String| {
            let (mut data, mut length) = (std::ptr::null_mut(), 0);
            VerQueryValueW(info.as_ptr().cast(), &HSTRING::from(name), &mut data, &mut length)
                .as_bool()
                .then_some((data, length as usize))
        };
        let (languages, bytes) = value(r"\VarFileInfo\Translation".into())?;
        if bytes < 4 {
            return None;
        }
        let [language, codepage] = *languages.cast::<[u16; 2]>();
        let (text, chars) =
            value(format!(r"\StringFileInfo\{language:04x}{codepage:04x}\FileDescription"))?;
        let text = String::from_utf16_lossy(std::slice::from_raw_parts(text.cast(), chars));
        let text = text.trim_matches(|c: char| c == '\0' || c.is_whitespace());
        (!text.is_empty()).then(|| text.to_owned())
    }
}
