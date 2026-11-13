use std::fs;
use std::io::{self, Cursor, Read};
use std::path::Path;

use crate::widgets::installed::read_manifest;

/// Unpacks `zip` into `dest`. It refuses paths outside `dest`, symbolic links, and archives that
/// unpack to more than `limit` bytes.
pub(super) fn unpack(zip: &[u8], dest: &Path, limit: u64) -> Result<(), String> {
    let bad = |e: &dyn std::fmt::Display| format!("widget.zip can't be read: {e}");
    let mut archive = zip::ZipArchive::new(Cursor::new(zip)).map_err(|e| bad(&e))?;
    let mut total = 0u64;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| bad(&e))?;
        let name = String::from_utf8_lossy(file.name_raw()).into_owned();
        let Some(rel) = file.enclosed_name() else {
            return Err(format!(
                "widget.zip has a path outside the widget folder: {name}"
            ));
        };
        if file.is_symlink() {
            return Err(format!("widget.zip has a symbolic link: {name}"));
        }
        let path = dest.join(rel);
        if file.is_dir() {
            fs::create_dir_all(&path).map_err(|e| bad(&e))?;
            continue;
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| bad(&e))?;
        }
        let mut out = fs::File::create(&path).map_err(|e| bad(&e))?;
        // Count the bytes during the unpack, becuase the sizes in an archive can be false.
        total +=
            io::copy(&mut (&mut file).take(limit - total + 1), &mut out).map_err(|e| bad(&e))?;
        if total > limit {
            return Err(format!(
                "widget.zip unpacks to more than {} MB",
                (limit >> 20).max(1)
            ));
        }
    }
    Ok(())
}

/// Checks that a widget folder has a `manifest.json` at its root, and the module that the
/// manifest names.
pub(super) fn check_widget(dir: &Path) -> Result<(), String> {
    let manifest =
        read_manifest(dir).map_err(|e| format!("widget.zip has no valid manifest.json: {e}"))?;
    let main = manifest["main"].as_str().unwrap_or("main.js");
    if !dir.join(main).is_file() {
        return Err(format!(
            "widget.zip has no {main}, which its manifest.json names"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::marketplace::temp_dir;
    use super::super::install::MAX_UNPACKED;
    use std::io::Write;
    use zip::write::SimpleFileOptions;

    fn zip_of(files: &[(&str, &[u8])]) -> Vec<u8> {
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
        for (name, body) in files {
            writer.start_file(*name, options).unwrap();
            writer.write_all(body).unwrap();
        }
        writer.finish().unwrap().into_inner()
    }

    #[test]
    fn unpacks_a_widget() {
        let dir = temp_dir("unpack");
        let zip = zip_of(&[
            ("manifest.json", br#"{"name":"W","main":"dist/main.js"}"#),
            ("dist/main.js", b"export default {}"),
        ]);
        unpack(&zip, &dir.join("w"), MAX_UNPACKED).unwrap();
        check_widget(&dir.join("w")).unwrap();
        assert!(dir.join("w/dist/main.js").is_file());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn refuses_paths_outside_the_folder() {
        let dir = temp_dir("escape");
        let zip = zip_of(&[("../evil.js", b"x")]);
        assert!(unpack(&zip, &dir.join("w"), MAX_UNPACKED).is_err());
        assert!(!dir.join("evil.js").exists());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn refuses_archives_that_unpack_too_large() {
        let dir = temp_dir("large");
        let zip = zip_of(&[("a.bin", &[0; 600]), ("b.bin", &[0; 600])]);
        assert!(unpack(&zip, &dir.join("w"), 1000).is_err());
        assert!(unpack(&zip, &dir.join("ok"), 1200).is_ok());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn refuses_a_widget_without_its_module() {
        let dir = temp_dir("nomain");
        let zip = zip_of(&[("manifest.json", br#"{"name":"W"}"#)]);
        unpack(&zip, &dir.join("w"), MAX_UNPACKED).unwrap();
        assert!(check_widget(&dir.join("w")).is_err());
        fs::remove_dir_all(dir).unwrap();
    }
}
