import { invoke } from '@tauri-apps/api/core';
import type {
	AudioApi,
	AudioLevels,
	AudioMixer,
	AudioOutput,
	AudioTarget,
	Cleanup
} from '../types';
import { clamp } from '#lib/clamp.ts';
import type { Scope } from '../scope';
import { fileIconUrl } from '../url';
import { stream, watchChanges } from './changes';
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

/** Mixer reads closer than this share one, because all mixer widgets read after a change. */
const MIXER_SHARE_MS = 250;
const LEVELS_SHARE_MS = 30;

/** The mixer read for the whole window: the widget api and the `audioApps` setting source. */
export const mixer = shared(() => invoke<AudioMixer>('audio_mixer'), MIXER_SHARE_MS);
const levels = shared(() => invoke<AudioLevels>('audio_levels'), LEVELS_SHARE_MS);
/** After a change from here, the next mixer read shows it and does not share an older read. */
const change = (done: Promise<boolean>) => done.finally(mixer.forget);

/** When Windows tells the app that the mixer changed, the next mixer read reads again. */
const changes = stream<null>('audio:changed', 'audio_watch', mixer.forget);
const levelStream = stream<AudioLevels>('audio:levels', 'audio_meters');

// The readers are wrapped, so that widgets do not get `forget`.
const readers = Object.freeze({
	mixer: () => mixer(),
	levels: () => levels(),
	setVolume: (target: AudioTarget, volume: number) =>
		change(invoke<boolean>('audio_set_volume', { target, volume: clamp(volume, 0, 1) })),
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
	guard: Scope['guard'],
	track: (stop: Cleanup) => Cleanup
): AudioApi {
	return Object.freeze({
		...readers,
		onChange: watchChanges(changes, pause, guard, track),
		onLevels(fn: (levels: AudioLevels) => unknown) {
			const run = guard(fn);
			return track(whileShown(pause, () => levelStream(run)));
		}
	});
}
