export interface MediaApi {
	/**
	 * The session that Windows thinks is current, which the media keys control. On Linux, the
	 * MPRIS player that plays, else the one that playerctld follows, else a paused one. `null` if
	 * none.
	 */
	current(): Promise<MediaInfo | null>;
	/** The album art of the track from the last `current()` call, as a data URL. `null` if none. */
	art(): Promise<string | null>;
	/** Asks the app of the current session to play, pause or skip. Resolves `false` if it refused. */
	control(action: MediaAction): Promise<boolean>;
}

export type MediaAction = 'play' | 'pause' | 'next' | 'previous';

/**
 * A media session, as an app publishes it to Windows, or as an MPRIS player on Linux. The fields
 * that the app does not set are empty.
 */
export interface MediaInfo {
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
	 * The position in the track at the time of the call, in seconds. `null` if the app gives no
	 * timeline. Some apps clear it, for example Zen and Firefox about once a minute. Then it
	 * continues from the last position, if calls come at least every 5 seconds.
	 */
	position: number | null;
	duration: number | null;
	/** Changes each time the album art changes. Then call `art()`. `null` if there is no art. */
	art: number | null;
	/** The `control()` actions that the app accepts now. */
	can: Record<MediaAction, boolean>;
}
