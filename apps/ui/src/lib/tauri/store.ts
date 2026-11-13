import { LazyStore } from '@tauri-apps/plugin-store';

/**
 * The app settings file. The plugin keeps one instance of each file for the whole app. Thus a
 * write from the settings window also calls the `onKeyChange` listeners in the Edge window.
 */
export const settingsStore = new LazyStore('settings.json');

/**
 * The data that widgets save with `api.storage`. The keys are `widget/<widget id>/<key>` and
 * `instance/<instance id>/<key>`.
 */
export const widgetDataStore = new LazyStore('widget-data.json');
