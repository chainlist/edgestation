/**
 * The apps in the All apps list of the Start menu, which is the Apps folder of the shell. On
 * Linux, the apps that the app menu shows: their desktop entries.
 */
export interface AppsApi {
	/** All apps in name order. The host reads them when you call this function. */
	list(): Promise<AppInfo[]>;
	/**
	 * Starts the app `id` as the Start menu does. On Windows, it asks for the app window on the
	 * main screen, not on the Edge. An app that keeps its last position can still open there. On
	 * Linux, the window manager places the window. Rejects if the app is gone.
	 */
	launch(id: string): Promise<void>;
	/**
	 * The URL of the icon of the app `id`, for an `<img>`. The icon is a square PNG of `size`
	 * pixels (16 to 256, default 64). Ask for the size that it shows at, in device pixels. Icons
	 * at a larger size are sharper.
	 */
	icon(id: string, size?: number): string;
}

/** An app as the Start menu lists it. */
export interface AppInfo {
	/**
	 * The value that launches the app and draws its icon. It is the name of the app in the Apps
	 * folder of the shell. This is the AppUserModelID of the app or, for a program without one,
	 * a path to the program. On Linux, it is the id of the desktop entry, such as
	 * `firefox.desktop`. It stays the same across restarts, so widgets can save it.
	 */
	id: string;
	/** The name that the Start menu or the app menu shows. */
	name: string;
}
