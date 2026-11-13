import type { Cleanup } from '../cleanup.js';

/**
 * The sound of the machine, as the Windows volume mixer shows it. On Linux, it comes from
 * PulseAudio, which PipeWire also serves, as the sound settings show it. Nothing listens between
 * calls.
 */
export interface AudioApi {
	/**
	 * The default output and microphone, and the apps on that output. The host reads them when
	 * you call this function.
	 */
	mixer(): Promise<AudioMixer>;
	/** How loud the default output, the microphone and each app are now. */
	levels(): Promise<AudioLevels>;
	/**
	 * Calls `fn` when the mixer changes: a volume, a mute, the default device, or the sound of an
	 * app when it opens, closes, starts or stops. Then read `mixer()`. The system tells the app
	 * about changes, so nothing polls. While the page of the widget is off screen or the Edge is
	 * frozen, it waits and calls `fn` once when the widget shows again. It stops on unmount, and
	 * the returned function stops it earlier.
	 */
	onChange(fn: () => unknown): Cleanup;
	/**
	 * Calls `fn` with the levels 25 times a second while there is sound, and for two seconds
	 * after, so meters can fall. Then it does not call `fn` until there is sound again. New sound
	 * shows within a quarter of a second. It calls `fn` only while the page of the widget shows
	 * and the Edge is not frozen. It stops on unmount, and the returned function stops it earlier.
	 */
	onLevels(fn: (levels: AudioLevels) => unknown): Cleanup;
	/**
	 * Sets the volume of `target`, from 0 to 1. It clamps values outside this range. Resolves to
	 * `false` if `target` is gone: the app closed, or the device is not there anymore.
	 */
	setVolume(target: AudioTarget, volume: number): Promise<boolean>;
	/**
	 * Mutes or unmutes `target`, like `setVolume`. For `'input'`, it mutes the default
	 * microphone. If the default microphone for calls is a different device, it mutes it too.
	 */
	setMuted(target: AudioTarget, muted: boolean): Promise<boolean>;
	/**
	 * The output devices that are plugged in and enabled, in name order. The host reads them when
	 * you call this function.
	 */
	outputs(): Promise<AudioOutput[]>;
	/**
	 * Makes the output `id` the default output, as "Set as default device" in Sound settings
	 * does. Calls keep the device that the user selected for them. Resolves to `false` if the
	 * device is not plugged in anymore. Windows has no public API for this, so it uses the API
	 * of Sound settings. On Linux, the apps that follow the default output move to it.
	 */
	setOutput(id: string): Promise<boolean>;
	/**
	 * The URL of the icon of the executable of an app (`AudioApp.path`), for an `<img>`. The icon
	 * is a square PNG of `size` pixels (16 to 256, default 64).
	 */
	icon(path: string, size?: number): string;
}

/** What `setVolume` and `setMuted` change: the default output, the microphone, or an app `id`. */
export type AudioTarget = 'output' | 'input' | (string & {});

export interface AudioMixer {
	/** The default output, which the volume flyout shows. `null` if there is none. */
	output: AudioDevice | null;
	/**
	 * The default microphone. `null` if there is none. `muted` is `true` only if the default
	 * microphone for calls is also muted.
	 */
	input: AudioDevice | null;
	/** The apps with a sound session on the default output, in name order, System sounds last. */
	apps: AudioApp[];
}

/** A sound device. */
export interface AudioDevice {
	/** The Windows id of the device, or the PulseAudio name on Linux, as `outputs()` lists it. */
	id: string;
	/** The name that Sound settings shows, such as `Speakers (Realtek(R) Audio)`. */
	name: string;
	/** From 0 to 1, as the volume flyout shows it. */
	volume: number;
	muted: boolean;
}

/**
 * The sound of an app: all the sessions that the Windows volume mixer shows as one. On Linux, all
 * the streams of one executable on the default output.
 */
export interface AudioApp {
	/**
	 * Identifies the sound of the app while it runs, for `setVolume` and `levels()`. It does not
	 * stay the same across restarts.
	 */
	id: string;
	/**
	 * The name that the app gives its sound. Else, the name that Task Manager shows, which is the
	 * description of its executable (on Linux, its name in the app menu). Else, the file name of
	 * its executable.
	 */
	name: string;
	/**
	 * The path of its executable, for `icon()`. `null` for System sounds, and on Linux for an app
	 * of another user.
	 */
	path: string | null;
	/**
	 * The file name of its executable, such as `Discord.exe`. It identifies the app across
	 * restarts and updates, so save this value. The folder of the app can change with them.
	 * `null` for System sounds.
	 */
	program: string | null;
	/** From 0 to 1, as a part of the output volume. */
	volume: number;
	muted: boolean;
	/** `true` if the app plays sound now, `false` if it is open and silent. */
	active: boolean;
}

/** The peak levels of the last few milliseconds, from 0 to 1. */
export interface AudioLevels {
	output: number;
	/** The default microphone while an app records from it. 0 at other times. */
	input: number;
	/** By app `id`. */
	apps: Record<string, number>;
}

/** An output device that `setOutput` can switch to. */
export interface AudioOutput {
	id: string;
	name: string;
}
