import type { Cleanup } from '../cleanup.js';

export interface MediaApi {
	/**
	 * The session that Windows thinks is current, which the media keys control. On Linux, the
	 * MPRIS player that plays, else the one that playerctld follows, else a paused one. `null` if
	 * none.
	 */
	current(): Promise<MediaInfo | null>;
	/**
	 * All sessions, in the order of the system, with `current` set on the one that `current()`
	 * gives. On Linux, the MPRIS players that play or are paused. Empty if none.
	 */
	sessions(): Promise<MediaInfo[]>;
	/**
	 * The album art of the track of session `id`, from the last `current()` or `sessions()` call,
	 * as a data URL. Without `id`, the art of the current session. `null` if none.
	 */
	art(id?: string): Promise<string | null>;
	/**
	 * Asks the app of session `id`, or of the current session, to play, pause or skip. Resolves
	 * `false` if it refused or the session is gone.
	 */
	control(action: MediaAction, id?: string): Promise<boolean>;
	/**
	 * Asks the app of session `id`, or of the current session, to go to `position` seconds in the
	 * track. `seek(0)` plays the track from the start. Resolves `false` if it refused or the
	 * session is gone. `can.seek` tells if the app accepts it now.
	 */
	seek(position: number, id?: string): Promise<boolean>;
	/**
	 * Calls `fn` when a session opens, closes or becomes current, or when its track, playback
	 * state or timeline changes. `fn` gets no arguments, and it does not tell what changed or in
	 * which session: read `sessions()` or `current()` in it. The system tells the app about
	 * changes, so nothing polls. A burst of changes calls `fn` at most every 100 ms. The position
	 * of a playing track moves on without a call, so advance it yourself between reads. While the
	 * page of the widget is off screen or the Edge is frozen, it waits and calls `fn` once when the
	 * widget shows again, also if nothing changed. It stops on unmount, and the returned function
	 * stops it earlier.
	 */
	onChange(fn: () => unknown): Cleanup;
}

export type MediaAction = 'play' | 'pause' | 'next' | 'previous';

/**
 * A media session, as an app publishes it to Windows, or as an MPRIS player on Linux. The fields
 * that the app does not set are empty.
 */
export interface MediaInfo {
	/**
	 * Names the session in `art()`, `control()` and `seek()`. On Windows, it is the app id, with `#2`, `#3`,
	 * ... for more sessions of the same app, in the order of the system. On Linux, the bus name of
	 * the player.
	 */
	id: string;
	/** `true` for the session that the media keys control. */
	current: boolean;
	/**
	 * The app that owns the session, as Windows names it (`Spotify.exe`, `MSEdge`, ...). On Linux,
	 * the desktop entry of the player (`spotify`, `firefox`, ...).
	 */
	app: string;
	title: string;
	artist: string;
	album: string;
	playing: boolean;
	/**
	 * The playback speed, 1 at normal speed. While `playing`, the position moves on by `rate`
	 * seconds per second. Neither `onChange` nor the system tells about this, so compute it.
	 */
	rate: number;
	/**
	 * The position in the track at the time of the call, in seconds. `null` if the app gives no
	 * timeline. Some apps clear it, for example Zen and Firefox about once a minute. Then it
	 * continues from the last position, while a widget listens with `onChange` or calls come at
	 * least every 5 seconds.
	 */
	position: number | null;
	duration: number | null;
	/** Changes each time the album art changes. Then call `art()`. `null` if there is no art. */
	art: number | null;
	/** The `control()` actions that the app accepts now, and `seek` if it accepts `seek()`. */
	can: Record<MediaAction | 'seek', boolean>;
}
