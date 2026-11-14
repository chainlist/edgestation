// Each module that calls the system has a side for each platform (see Platforms in
// ARCHITECTURE.md). Another platform needs its own side in each of them.
#[cfg(not(any(windows, target_os = "linux")))]
compile_error!("EdgeStation builds for Windows and Linux only");

mod app_windows;
mod apps;
mod audio;
mod blocking;
mod changes;
mod focus;
mod fullscreen;
mod http;
mod identify;
mod media;
mod monitors;
mod os;
mod process;
mod system;
mod tray;
mod widgets;

use tauri::Manager;

pub fn run() {
    os::prepare();
    tauri::Builder::default()
        // A second launch opens the settings of the running app. Linux trays give no clicks,
        // and some desktops show no tray at all, so this is the way in there. It goes first, so
        // the second app exits before the other plugins start.
        .plugin(tauri_plugin_single_instance::init(|app, _, _| app_windows::show_settings(app)))
        .plugin(tauri_plugin_store::Builder::new().build())
        // The Edge window registers its shortcuts from the saved settings.
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // Open at startup. The settings window turns it on and off, and Windows keeps the the
        // state.
        .plugin(tauri_plugin_autostart::Builder::new().build())
        // `api.fetch` sends the widget requests from here, so CORS does not apply.
        .plugin(tauri_plugin_http::init())
        // Gets newer releases from GitHub. CI signs them. plugins.updater in tauri.conf.json
        // sets the source. The Edge looks for one at launch and asks before it installs it
        // (UpdatePrompt.svelte).
        .plugin(tauri_plugin_updater::Builder::new().build())
        // Only the host opens files and folders. The plugin script asks for a permission that
        // remote pages do not have, so links in widget web views do not open when it is on.
        .plugin(tauri_plugin_opener::Builder::new().open_js_links_on_click(false).build())
        .register_asynchronous_uri_scheme_protocol("widget", widgets::protocol)
        .register_asynchronous_uri_scheme_protocol("appicon", apps::icon_protocol)
        .on_page_load(|page, payload| {
            widgets::webview::control::close_on_reload(page, payload);
            audio::stop_on_reload(page, payload);
            media::stop_on_reload(page, payload);
        })
        .invoke_handler(tauri::generate_handler![
            apps::apps_list,
            apps::apps_launch,
            audio::audio_mixer,
            audio::audio_levels,
            audio::audio_set_volume,
            audio::audio_set_muted,
            audio::audio_outputs,
            audio::audio_set_output,
            audio::audio_watch,
            audio::audio_meters,
            focus::focus_borrow,
            focus::focus_return,
            app_windows::edge_visible,
            app_windows::edge_fullscreen,
            fullscreen::fullscreen_watch,
            identify::identify_prepare,
            identify::identify_screens,
            identify::identify_release,
            media::media_info,
            media::media_sessions,
            media::media_art,
            media::media_control,
            media::media_seek,
            media::media_watch,
            monitors::monitor_names,
            system::system_info,
            system::system_top_programs,
            widgets::installed::widgets_list,
            widgets::installed::widgets_open_dir,
            widgets::marketplace::listing::marketplace_list,
            widgets::marketplace::install::marketplace_install,
            widgets::marketplace::install::marketplace_uninstall,
            widgets::marketplace::update::marketplace_update,
            widgets::marketplace::listing::marketplace_open_link,
            widgets::webview::open::webview_open,
            widgets::webview::control::webview_place,
            widgets::webview::control::webview_navigate,
            widgets::webview::capture::webview_capture,
            widgets::webview::control::webview_close,
            widgets::webview::profiles::webview_profile_delete
        ])
        .setup(|app| {
            // The page shows this window when it is on the Edge, maybe before the page is drawn.
            if let Some(edge) = app.get_webview_window(changes::EDGE) {
                edge.set_background_color(Some(app_windows::page_background(app.handle())))?;
            }
            Ok(tray::create(app.handle())?)
        })
        .run(tauri::generate_context!())
        .expect("error while running EdgeStation");
}
