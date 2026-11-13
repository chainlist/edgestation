//! The Linux side. It uses the desktop entries of the app menu (XDG), through GIO, as the menus
//! of GNOME and KDE do. Icons come from the icon theme of the desktop.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::time::{Duration, Instant};

use gtk::gdk_pixbuf::{Colorspace, Pixbuf};
use gtk::gio::prelude::*;
use gtk::gio::{self, AppLaunchContext, DesktopAppInfo};
use gtk::glib;
use tauri::{Runtime, Window};

use super::App;

/// How long the entries stay known for the names and icons of programs. An app that the user
/// installs shows after this time.
const INDEX_FOR: Duration = Duration::from_secs(30);

/// Lists every app that the app menu shows, sorted by name.
pub fn list() -> Result<Vec<App>, String> {
    let mut apps: Vec<App> = gio::AppInfo::all()
        .into_iter()
        .filter(|app| app.should_show())
        .filter_map(|app| Some(App { id: app.id()?.into(), name: app.name().into() }))
        .collect();
    apps.sort_by_cached_key(|app| app.name.to_lowercase());
    apps.dedup_by(|a, b| a.id == b.id);
    Ok(apps)
}

/// Starts app `id`, as the app menu does. There is no hint for the screen: the window manager
/// places the window.
/// Only Windows takes the screen for the window of an app that it starts.
pub fn away_from<R: Runtime>(_window: &Window<R>) -> Option<isize> {
    None
}

pub fn launch(id: &str, _screen: Option<isize>) -> Result<(), String> {
    let app = DesktopAppInfo::new(id).ok_or_else(|| format!("no app {id}"))?;
    app.launch_uris(&[], None::<&AppLaunchContext>).map_err(|error| error.to_string())
}

/// The icon of app `id` as a square PNG of `size` pixels, or `None` if the app is gone.
pub fn icon(id: &str, size: u32) -> Option<Vec<u8>> {
    let icon = DesktopAppInfo::new(id)?.icon()?;
    render(&IconSource::of(&icon)?, size)
}

/// The icon of the app that runs the executable at `path`, like `icon`. This is the icon of its
/// entry in the app menu. A program without an entry can have an icon of the same name in the
/// theme.
pub fn executable_icon(path: &str, size: u32) -> Option<Vec<u8>> {
    let path = Path::new(path);
    if !path.is_absolute() {
        return None;
    }
    let index = index();
    match find(&index, path, std::fs::canonicalize(path).ok().as_deref()) {
        Some(entry) => render(entry.icon.as_ref()?, size),
        None => {
            let name = path.file_name()?.to_str()?.to_lowercase();
            render(&IconSource::Names(vec![name]), size)
        }
    }
}

/// The name in the app menu of the app that runs the executable at `path`, for example `Firefox`
/// for `/usr/lib/firefox/firefox`.
pub fn program_name(path: &str) -> Option<String> {
    let path = Path::new(path);
    let index = index();
    find(&index, path, std::fs::canonicalize(path).ok().as_deref()).map(|entry| entry.name.clone())
}

/// What the app keeps of a desktop entry. GIO objects stay on the thread that made them, so this
/// copies the parts that the app needs.
#[derive(Debug, Default)]
struct Entry {
    name: String,
    /// True if the app menu shows the entry.
    shown: bool,
    icon: Option<IconSource>,
    /// The program that `Exec` starts, found on the PATH, with its links resolved.
    program: Option<PathBuf>,
    /// The file names in lower case that the executable of the app can have: the program that
    /// `Exec` starts, the last part of the id, and the window class.
    names: Vec<String>,
}

#[derive(Clone, Debug)]
enum IconSource {
    /// Names in the icon theme, the best first.
    Names(Vec<String>),
    File(PathBuf),
}

impl IconSource {
    fn of(icon: &gio::Icon) -> Option<Self> {
        if let Some(themed) = icon.downcast_ref::<gio::ThemedIcon>() {
            let names = themed.names().iter().map(|name| name.to_string()).collect();
            return Some(Self::Names(names));
        }
        let file = icon.downcast_ref::<gio::FileIcon>()?.file().path()?;
        Some(Self::File(file))
    }
}

static INDEX: Mutex<Option<(Instant, Arc<Vec<Entry>>)>> = Mutex::new(None);

/// All desktop entries. The app reads them again after `INDEX_FOR`.
fn index() -> Arc<Vec<Entry>> {
    let mut kept = INDEX.lock().unwrap_or_else(PoisonError::into_inner);
    match &*kept {
        Some((at, entries)) if at.elapsed() < INDEX_FOR => entries.clone(),
        _ => {
            let entries = Arc::new(gio::AppInfo::all().iter().filter_map(entry).collect());
            *kept = Some((Instant::now(), Arc::clone(&entries)));
            entries
        }
    }
}

fn entry(app: &gio::AppInfo) -> Option<Entry> {
    let desktop = app.downcast_ref::<DesktopAppInfo>()?;
    let mut names: Vec<String> = Vec::new();
    let mut program = None;
    let command = app.commandline().and_then(|line| line.to_str().map(str::to_owned));
    let argv = command.and_then(|line| glib::shell_parse_argv(line).ok()).unwrap_or_default();
    let argv: Vec<_> = argv.iter().filter_map(|arg| arg.to_str()).collect();
    if let Some((first, flatpak_command)) = started_program(&argv) {
        let found = if Path::new(first).is_absolute() {
            Some(PathBuf::from(first))
        } else {
            glib::find_program_in_path(first)
        };
        program = found.and_then(|path| std::fs::canonicalize(path).ok());
        names.extend(Path::new(first).file_name().and_then(|name| name.to_str()).map(Into::into));
        names.extend(flatpak_command.map(Into::into));
    }
    if let Some(id) = app.id() {
        names.push(id_name(&id).into());
    }
    names.extend(desktop.startup_wm_class().map(Into::into));
    let mut names: Vec<String> = names.iter().map(|name| name.to_lowercase()).collect();
    names.retain(|name| !name.is_empty());
    names.dedup();
    Some(Entry {
        name: app.name().into(),
        shown: app.should_show(),
        icon: app.icon().as_ref().and_then(IconSource::of),
        program,
        names,
    })
}

/// The program that the arguments of `Exec` start, after `env` and its variables. For a Flatpak
/// app, also the command that it runs in its sandbox.
fn started_program<'a>(argv: &[&'a str]) -> Option<(&'a str, Option<&'a str>)> {
    let mut args = argv.iter().copied().peekable();
    if args.peek().is_some_and(|arg| Path::new(arg).file_name().is_some_and(|f| f == "env")) {
        args.next();
        while args.peek().is_some_and(|arg| arg.contains('=') && !arg.starts_with('-')) {
            args.next();
        }
    }
    let first = args.next()?;
    let flatpak = Path::new(first).file_name().is_some_and(|f| f == "flatpak");
    let command = flatpak
        .then(|| args.find_map(|arg| arg.strip_prefix("--command=")))
        .flatten()
        .and_then(|command| Path::new(command).file_name()?.to_str());
    Some((first, command))
}

/// The last part of desktop file id `id`, for example `nautilus` for `org.gnome.Nautilus.desktop`.
fn id_name(id: &str) -> &str {
    let id = id.strip_suffix(".desktop").unwrap_or(id);
    id.rsplit('.').next().unwrap_or(id)
}

/// The entry of the app that runs executable `exe`. This is the entry that starts the same file
/// (`canonical`, with its links resolved), or else an entry with the name of the file. Entries
/// that the menu shows come first.
fn find<'a>(entries: &'a [Entry], exe: &Path, canonical: Option<&Path>) -> Option<&'a Entry> {
    let best = |matches: &dyn Fn(&Entry) -> bool| {
        let mut found = entries.iter().filter(|entry| matches(entry));
        let first = found.next()?;
        Some(if first.shown { first } else { found.find(|entry| entry.shown).unwrap_or(first) })
    };
    let file = exe.file_name()?.to_str()?.to_lowercase();
    canonical
        .and_then(|canonical| best(&|entry| entry.program.as_deref() == Some(canonical)))
        .or_else(|| best(&|entry| entry.names.contains(&file)))
}

/// The name of the icon theme of the desktop, or the default theme if it is not known.
fn theme() -> &'static str {
    static THEME: OnceLock<String> = OnceLock::new();
    THEME.get_or_init(|| freedesktop_icons::default_theme_gtk().unwrap_or_else(|| "hicolor".into()))
}

/// Draws `icon` as a square PNG of `size` pixels. An icon that is not square is centered.
fn render(icon: &IconSource, size: u32) -> Option<Vec<u8>> {
    let path = match icon {
        IconSource::File(path) => path.clone(),
        IconSource::Names(names) => names.iter().find_map(|name| {
            // Old entries can name the file of the icon, with its extension.
            let name = name.strip_suffix(".png").or(name.strip_suffix(".svg")).unwrap_or(name);
            let size = u16::try_from(size).ok()?;
            freedesktop_icons::lookup(name).with_size(size).with_theme(theme()).with_cache().find()
        })?,
    };
    let side = i32::try_from(size).ok()?;
    let drawn = Pixbuf::from_file_at_scale(&path, side, side, true).ok()?;
    let square = Pixbuf::new(Colorspace::Rgb, true, 8, side, side)?;
    square.fill(0);
    let (width, height) = (drawn.width().min(side), drawn.height().min(side));
    drawn.copy_area(0, 0, width, height, &square, (side - width) / 2, (side - height) / 2);
    square.save_to_bufferv("png", &[]).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(names: &[&str], program: Option<&str>, shown: bool) -> Entry {
        Entry {
            name: names[0].to_owned(),
            shown,
            program: program.map(PathBuf::from),
            names: names.iter().map(|name| name.to_string()).collect(),
            ..Entry::default()
        }
    }

    #[test]
    fn finds_the_program_after_env_and_in_flatpak() {
        assert_eq!(started_program(&["firefox", "%u"]), Some(("firefox", None)));
        let snap = ["env", "BAMF_DESKTOP_FILE_HINT=/a.desktop", "/snap/bin/firefox", "%u"];
        assert_eq!(started_program(&snap), Some(("/snap/bin/firefox", None)));
        let flatpak = [
            "/usr/bin/flatpak",
            "run",
            "--branch=stable",
            "--command=spotify",
            "com.spotify.Client",
        ];
        assert_eq!(started_program(&flatpak), Some(("/usr/bin/flatpak", Some("spotify"))));
        assert_eq!(started_program(&[]), None);
    }

    #[test]
    fn takes_the_last_part_of_an_id() {
        assert_eq!(id_name("org.gnome.Nautilus.desktop"), "Nautilus");
        assert_eq!(id_name("firefox.desktop"), "firefox");
        assert_eq!(id_name("code"), "code");
    }

    #[test]
    fn matches_an_executable_by_path_then_by_name() {
        let entries = [
            entry(&["helper"], Some("/usr/lib/a/helper"), false),
            entry(&["discord"], Some("/usr/share/discord/Discord"), true),
            entry(&["firefox"], Some("/usr/lib/firefox/firefox.sh"), true),
        ];
        let exe = Path::new("/usr/share/discord/Discord");
        assert_eq!(find(&entries, exe, Some(exe)).unwrap().name, "discord");
        let exe = Path::new("/usr/lib/firefox/firefox");
        assert_eq!(find(&entries, exe, Some(exe)).unwrap().name, "firefox");
        assert!(find(&entries, Path::new("/usr/bin/bash"), None).is_none());
    }

    /// Reads the real app menu. Run it with `--ignored --nocapture` to see the apps.
    #[test]
    #[ignore]
    fn reads_the_app_menu() {
        let apps = list().unwrap();
        for app in &apps {
            let drawn = icon(&app.id, 48).is_some_and(|png| png.starts_with(b"\x89PNG"));
            println!("{} | {} | icon: {drawn}", app.id, app.name);
        }
        for exe in ["/usr/bin/mpv", "/usr/bin/bash"] {
            let drawn = executable_icon(exe, 32).is_some();
            println!("{exe} | {:?} | icon: {drawn}", program_name(exe));
        }
    }

    #[test]
    fn prefers_entries_that_the_menu_shows() {
        let entries = [entry(&["spotify", "hidden"], None, false), entry(&["spotify"], None, true)];
        let found = find(&entries, Path::new("/opt/spotify/spotify"), None).unwrap();
        assert!(found.shown);
    }
}
