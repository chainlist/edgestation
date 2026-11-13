import type { AppsApi } from './api/apps.js';
import type { AudioApi } from './api/audio.js';
import type { MediaApi } from './api/media.js';
import type { StorageApi } from './api/storage.js';
import type { ProgramUsage, SystemInfo } from './api/system.js';
import type { WebviewOptions } from './api/webview.js';
import type { Cleanup } from './cleanup.js';
import type { WidgetManifest } from './manifest.js';
import type { WidgetSettingValues } from './settings.js';

/**
 * What a widget gets from the host. It is small on purpose, because the full API is not
 * designed yet. When the widget unmounts, the host releases all that the widget registered here.
 * The members marked "Needs" work only if the `permissions` of the manifest list that
 * permission. Else a call shows the error card and throws.
 */
export interface WidgetApi {
	/** The id of this placement. Two copies of a widget get different ids. */
	readonly instanceId: string;
	readonly manifest: WidgetManifest;
	/**
	 * The settings of this placement. It has each key that the manifest declares, with its
	 * default if the user did not set it. The host mounts the widget again when the user
	 * changes a setting.
	 */
	readonly settings: Readonly<WidgetSettingValues>;
	/**
	 * The performance of the machine, read when you call it. Nothing monitors between calls.
	 * Needs `system`.
	 */
	system(): Promise<SystemInfo>;
	/**
	 * The programs that used the most CPU since the previous call from any widget, busiest
	 * first. It gives `count` programs at most, or fewer if fewer programs used the CPU. The
	 * host reads them when you call it. Needs `system`.
	 */
	topPrograms(count: number): Promise<ProgramUsage[]>;
	/**
	 * The media that plays on the machine, read when you call it, and its playback controls.
	 * Needs `media`.
	 */
	readonly media: MediaApi;
	/**
	 * The apps of the Start menu. It lists them when you call it, starts them and gives their
	 * icons. Needs `apps`.
	 */
	readonly apps: AppsApi;
	/**
	 * The sound of the machine, as the volume mixer shows it. This is the default output and
	 * microphone, the apps that play sound, their volumes and how loud they are. The host reads
	 * and changes these values when you call it. Needs `audio`.
	 */
	readonly audio: AudioApi;
	/**
	 * The web `fetch`, but the app sends the request, not the Edge page. Thus CORS does not
	 * apply, and any `http` or `https` URL answers as it would answer a server. You cannot set
	 * the headers that browsers forbid (`Cookie`, `Origin`, `Referer`, ...). The app keeps the
	 * cookies that sites set across restarts, and all widgets share them. If a request has no
	 * answer when the widget unmounts, the app cancels it. Needs `fetch`.
	 */
	fetch(input: RequestInfo | URL, init?: RequestInit): Promise<Response>;
	/**
	 * Data that stays across remounts and restarts. All placements of the widget share it, or
	 * this placement owns it. The host never deletes data in it automatically.
	 */
	readonly storage: StorageApi;
	/** The URL of a file in the widget folder (images, data, ...). */
	asset(path: string): string;
	/** Wraps a callback. If the callback throws an error, the error card of this widget shows it. */
	guard<A extends unknown[]>(fn: (...args: A) => unknown): (...args: A) => void;
	/**
	 * Tells the host that the widget loads data until `task` settles, and returns `task`. Until
	 * the first load ends, three dots show in the middle of the card and the widget stays hidden.
	 * Then the widget fades in. Each later load shows a bar at the top of the card. Loads that
	 * run at the same time show one indicator. Put all of the first load in one call, else the
	 * widget shows before it is ready. The page loads of `webview` count as loads too.
	 */
	loading<T>(task: Promise<T>): Promise<T>;
	/**
	 * A guarded `setInterval` that the host clears when the widget unmounts. It stops while the
	 * page of the widget is off screen or the Edge is frozen. The Edge is frozen when another
	 * app is full screen. When it starts again, it ticks immediately. Returns a function that
	 * stops it earlier.
	 */
	every(ms: number, fn: () => unknown): Cleanup;
	/**
	 * A guarded `setTimeout` that the host clears if the widget unmounts first. It waits while
	 * the page of the widget is off screen. If its time came in that period, it fires when the
	 * page is back. Unlike `every`, it still fires while the Edge is frozen. Returns a function
	 * that cancels it.
	 */
	after(ms: number, fn: () => unknown): Cleanup;
	/**
	 * Shows the web page `url` in a native browser view that lies over `target` and gets the
	 * touches there, for sites that refuse an iframe. The view is above the Edge, so it hides
	 * while pages slide and while `target` is not fully on screen. At these times, after its
	 * first show, a picture of it covers `target` (an `<img>` in the root of the widget). Its page
	 * loads show on the card as `loading` shows them. It opens the first time `target` is fully on
	 * screen, so a view on a page that nobody visits costs nothing. It stays hidden until its
	 * first page loads, so the dots show in place of a blank page. It uses the browser profile
	 * from the placement settings, Default if the user selects no other, and sign-ins in it stay
	 * across restarts. Needs `webview`. The host closes it when the widget unmounts, and the
	 * returned function closes it earlier.
	 */
	webview(target: HTMLElement, url: string, options?: WebviewOptions): Cleanup;
	/**
	 * Replaces the native scrollbars of `viewport` with the overlay scrollbars of the app.
	 * `viewport` is an element of the widget that scrolls, and the overlay scrollbars show while
	 * it scrolls. The app draws them in the parent of `viewport`, so put `viewport` in an element
	 * of its own. This parent takes the place of `viewport` in the layout, and `viewport` fills
	 * it. The host removes them when the widget unmounts, and the returned function removes them
	 * earlier.
	 */
	scrollbars(viewport: HTMLElement): Cleanup;
	/** Runs `fn` when the widget unmounts. */
	onDispose(fn: Cleanup): void;
}
