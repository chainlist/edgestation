import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type {
	AudioApi,
	AudioLevels,
	AudioMixer,
	AudioOutput,
	AudioTarget,
	Cleanup
} from '../types';
import { fileIconUrl } from '../url';
import { whileShown, type Pause } from './timers';

/**
 * Wraps `read` so that calls less than `ms` apart share one call and one result. Use it for
 * values that many widgets read. `forget` makes the next call read again.
 */
export function shared<T>(read: () => Promise<T>, ms: number) {
	let last: { at: number; result: Promise<T> } | undefined;
	const get = () => {
		const now = performance.now();
		if (!last || now - last.at >= ms) {
			const result = read();
			last = { at: now, result };
			// Do not share a failed read. The next call tries again.
			result.catch(() => {
				if (last?.result === result) last = undefined;
			});
		}
		return last.result;
	};
	return Object.assign(get, { forget: () => void (last = undefined) });
}

/**
 * Keeps the listeners of a stream that the app sends (`event`). `command` turns the stream on
 * with the first listener and off with the last. So the app listens to Windows only while a
 * widget listens. `each` runs on each message, before the listeners.
 */
export function stream<T>(event: string, command: string, each?: (payload: T) => void) {
	const listeners = new Set<(payload: T) => void>();
	let unlisten: Promise<UnlistenFn> | undefined;
	return (listener: (payload: T) => void): Cleanup => {
		listeners.add(listener);
		if (listeners.size === 1) {
			const listening = listen<T>(event, ({ payload }) => {
				each?.(payload);
				for (const fn of [...listeners]) fn(payload);
			});
			unlisten = listening;
			// Listen first, becuase then no message gets lost. Turn the stream on only if the
			// listeners did not leave in the meantime.
			void listening.then(() => {
				if (unlisten === listening) void invoke(command, { on: true });
			});
		}
		return () => {
			if (!listeners.delete(listener) || listeners.size) return;
			void invoke(command, { on: false });
			void unlisten?.then((stop) => stop());
			unlisten = undefined;
		};
	};
}

/** Mixer reads closer than this share one, because all mixer widgets read after a change. */
const MIXER_SHARE_MS = 250;
const LEVELS_SHARE_MS = 30;

const mixer = shared(() => invoke<AudioMixer>('audio_mixer'), MIXER_SHARE_MS);
const levels = shared(() => invoke<AudioLevels>('audio_levels'), LEVELS_SHARE_MS);
/** After a change from here, the next mixer read shows it and does not share an older read. */
const change = (done: Promise<boolean>) => done.finally(mixer.forget);

/** When Windows tells the app that the mixer changed, the next mixer read reads again. */
const changes = stream<null>('audio:changed', 'audio_watch', mixer.forget);
const levelStream = stream<AudioLevels>('audio:levels', 'audio_meters');

const readers = Object.freeze({
	mixer,
	levels,
	setVolume: (target: AudioTarget, volume: number) =>
		change(
			invoke<boolean>('audio_set_volume', { target, volume: Math.min(1, Math.max(0, volume)) })
		),
	setMuted: (target: AudioTarget, muted: boolean) =>
		change(invoke<boolean>('audio_set_muted', { target, muted })),
	outputs: () => invoke<AudioOutput[]>('audio_outputs'),
	setOutput: (id: string) => change(invoke<boolean>('audio_set_output', { id })),
	icon: (path: string, size = 64) => fileIconUrl(path, size)
});

/**
 * The `api.audio` of one widget. Its listeners listen only while its page shows and the Edge is
 * not frozen (`pause`). `guard` sends their errors to its error card. `track` stops them when the
 * widget unmounts.
 */
export function createAudioApi(
	pause: Pause,
	guard: <A extends unknown[]>(fn: (...args: A) => unknown) => (...args: A) => void,
	track: (stop: Cleanup) => Cleanup
): AudioApi {
	return Object.freeze({
		...readers,
		onChange(fn: () => unknown) {
			const run = guard(fn);
			return track(
				whileShown(pause, (resumed) => {
					// Show the changes that came while it wated.
					if (resumed) run();
					return changes(run);
				})
			);
		},
		onLevels(fn: (levels: AudioLevels) => unknown) {
			const run = guard(fn);
			return track(whileShown(pause, () => levelStream(run)));
		}
	});
}
