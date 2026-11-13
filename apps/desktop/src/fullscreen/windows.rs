//! The Windows side. Win event hooks on the main thread report the window that comes to the
//! foreground, and the moves, resizes, hides and closes of the windows of its thread.

use std::cell::RefCell;

use tauri::AppHandle;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MONITOR_DEFAULTTONULL, MONITORINFO, MonitorFromWindow,
};
use windows::Win32::System::Threading::GetCurrentProcessId;
use windows::Win32::UI::Accessibility::{HWINEVENTHOOK, SetWinEventHook, UnhookWinEvent};
use windows::Win32::UI::WindowsAndMessaging::{
    CHILDID_SELF, EVENT_OBJECT_DESTROY, EVENT_OBJECT_LOCATIONCHANGE, EVENT_SYSTEM_FOREGROUND,
    GWL_STYLE, GetForegroundWindow, GetShellWindow, GetWindowLongW, GetWindowRect,
    GetWindowThreadProcessId, IsIconic, IsWindowVisible, IsZoomed, OBJID_WINDOW,
    WINEVENT_OUTOFCONTEXT, WS_CAPTION,
};

struct Watch {
    app: AppHandle,
    /// Fires when any window comes to the foreground.
    foreground: HWINEVENTHOOK,
    /// The last window of another app that came to the foregound. It stays when a window
    /// of this app gets the focus.
    target: Option<HWND>,
    /// The thread id of the target, and a hook on the windows of that thread. The hook fires
    /// when one of them is destroyed, shown, hidden, moved or resized. The foreground hook
    /// does not see these changes. One example is a window that goes full screen in place,
    /// with F11 or a game that changes mode. Another is a window that closes while the Edge
    /// has the focus.
    changes: Option<(u32, HWINEVENTHOOK)>,
    /// The last state sent to the Edge window.
    sent: Option<bool>,
}

// Windows calls the hooks on the main thread, and only that thread adds and removes them.
thread_local! {
    static WATCH: RefCell<Option<Watch>> = const { RefCell::new(None) };
}

pub fn set(app: AppHandle, enabled: bool) -> Result<(), String> {
    let handle = app.clone();
    // Windows sends the events to the thread that asked for them. That thread must run a
    // message loop, so use the main thread.
    app.run_on_main_thread(move || if enabled { start(handle) } else { stop() })
        .map_err(|error| error.to_string())
}

fn start(app: AppHandle) {
    WATCH.with_borrow_mut(|watch| {
        let watch = watch.get_or_insert_with(|| Watch {
            app,
            foreground: hook(EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_FOREGROUND, 0),
            target: None,
            changes: None,
            sent: None,
        });
        // A new page in the Edge window needs the current state, also when it did not change.
        watch.sent = None;
    });
    focus(unsafe { GetForegroundWindow() });
}

fn stop() {
    if let Some(watch) = WATCH.take() {
        unhook(watch.foreground);
        if let Some((_, changes)) = watch.changes {
            unhook(changes);
        }
    }
}

fn hook(first: u32, last: u32, thread: u32) -> HWINEVENTHOOK {
    let hook = unsafe {
        SetWinEventHook(first, last, None, Some(on_event), 0, thread, WINEVENT_OUTOFCONTEXT)
    };
    if hook.is_invalid() {
        eprintln!("could not watch window events {first:#x} to {last:#x}");
    }
    hook
}

fn unhook(hook: HWINEVENTHOOK) {
    unsafe {
        let _ = UnhookWinEvent(hook);
    }
}

unsafe extern "system" fn on_event(
    _: HWINEVENTHOOK,
    event: u32,
    window: HWND,
    object: i32,
    child: i32,
    _: u32,
    _: u32,
) {
    if event == EVENT_SYSTEM_FOREGROUND {
        focus(window);
    // The thread events also include child windows, the focus and the caret. Only the
    // target window itslef counts.
    } else if object == OBJID_WINDOW.0 && child == CHILDID_SELF as i32 && is_target(window) {
        check();
    }
}

fn is_target(window: HWND) -> bool {
    WATCH.with_borrow(|watch| watch.as_ref().is_some_and(|watch| watch.target == Some(window)))
}

/// `window` came to the foreground. It becomes the target, unless it is a window of this app.
fn focus(window: HWND) {
    let process = owner(window);
    if process != 0 && process != unsafe { GetCurrentProcessId() } {
        WATCH.with_borrow_mut(|watch| {
            if let Some(watch) = watch {
                watch.follow(window);
            }
        });
    }
    check();
}

/// Tells the Edge window wether the target covers its monitor. It sends only a changed state.
fn check() {
    let update = WATCH.with_borrow_mut(|watch| {
        let watch = watch.as_mut()?;
        let fullscreen = watch.target.is_some_and(covers_monitor);
        if watch.sent == Some(fullscreen) {
            return None;
        }
        watch.sent = Some(fullscreen);
        Some((watch.app.clone(), fullscreen))
    });
    if let Some((app, fullscreen)) = update {
        super::send(&app, fullscreen);
    }
}

impl Watch {
    /// Makes `window` the target, and moves the change hook to its thread.
    fn follow(&mut self, window: HWND) {
        self.target = Some(window);
        let thread = unsafe { GetWindowThreadProcessId(window, None) };
        if self.changes.is_some_and(|(hooked, _)| hooked == thread) {
            return;
        }
        if let Some((_, changes)) = self.changes.take() {
            unhook(changes);
        }
        let changes = hook(EVENT_OBJECT_DESTROY, EVENT_OBJECT_LOCATIONCHANGE, thread);
        self.changes = Some((thread, changes));
    }
}

/// The id of the process that owns `window`, or 0 when the window is gone.
fn owner(window: HWND) -> u32 {
    let mut process = 0;
    unsafe { GetWindowThreadProcessId(window, Some(&raw mut process)) };
    process
}

/// True if `window` covers its full monitor. The shell windows (desktop, Alt+Tab, Task View)
/// never count. A maximized window with a title bar over an auto-hidden taskbar also does
/// not count.
fn covers_monitor(window: HWND) -> bool {
    let process = owner(window);
    if process == 0 || process == owner(unsafe { GetShellWindow() }) {
        return false;
    }
    unsafe {
        if !IsWindowVisible(window).as_bool() || IsIconic(window).as_bool() {
            return false;
        }
        let caption = WS_CAPTION.0;
        if IsZoomed(window).as_bool()
            && GetWindowLongW(window, GWL_STYLE) as u32 & caption == caption
        {
            return false;
        }
        let monitor = MonitorFromWindow(window, MONITOR_DEFAULTTONULL);
        let mut info =
            MONITORINFO { cbSize: size_of::<MONITORINFO>() as u32, ..Default::default() };
        let mut rect = RECT::default();
        if monitor.is_invalid()
            || !GetMonitorInfoW(monitor, &mut info).as_bool()
            || GetWindowRect(window, &mut rect).is_err()
        {
            return false;
        }
        let screen = info.rcMonitor;
        rect.left <= screen.left
            && rect.top <= screen.top
            && rect.right >= screen.right
            && rect.bottom >= screen.bottom
    }
}
