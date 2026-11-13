import type { WidgetSetting } from './settings.js';

/**
 * A part of `api` that a widget must ask for in `permissions`. `system` also gives
 * `topPrograms`. The other members of `api` need no permission.
 */
export type WidgetPermission = 'system' | 'media' | 'apps' | 'audio' | 'fetch' | 'webview';

/**
 * The contents of the `manifest.json` of a widget, as the host gives it to the widget in
 * `api.manifest`. The file does not contain `id`. The host sets `id` to the folder name of the
 * widget.
 */
export interface WidgetManifest {
	id: string;
	name: string;
	version?: string;
	description?: string;
	author?: string;
	/**
	 * The oldest EdgeStation version that the widget runs on, such as `"0.1.0"`. This is the
	 * version whose `api` the widget uses. Older versions show the error card of the widget and
	 * do not load it.
	 */
	minAppVersion?: string;
	/** The entry module, relative to the widget folder. The default is `main.js`. */
	main?: string;
	/** A stylesheet that the host loads into the shadow root of the widget. */
	styles?: string;
	/**
	 * An image that settings show next to the name of the widget, relative to the widget folder.
	 * Only its shape is important, because the host draws it in the accent color.
	 */
	icon?: string;
	/** The default size in grid cells when the user adds the widget to a page. */
	size?: { w: number; h: number };
	/** The options that the user can set for each placement, in display order. */
	settings?: WidgetSetting[];
	/**
	 * The parts of `api` that the widget uses. The marketplace tells the user what each one lets
	 * the widget do before the install, and before an update that adds one. A call to a part that
	 * is not in this list shows the error card and throws. `webview` also adds a Profile choice to
	 * the settings of the widget.
	 */
	permissions?: WidgetPermission[];
}
