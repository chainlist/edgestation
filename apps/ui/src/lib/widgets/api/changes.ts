import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { Cleanup } from '../types';
import { whileShown, type Pause } from './timers';

/**
 * Keeps the listeners of a stream that the app sends (`event`). `command` turns the stream on
 * with the first listener and off with the last. So the app listens to the system only while a
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

type Guard = <A extends unknown[]>(fn: (...args: A) => unknown) => (...args: A) => void;

/**
 * Makes an `onChange` of `api` from `changes`, a stream of change events. The listener of one
 * widget listens only while its page shows and the Edge is not frozen (`pause`). It runs once
 * when the widget shows again, for the changes that came while it waited. `guard` sends its
 * errors to the error card, and `track` stops it when the widget unmounts.
 */
export const watchChanges =
	(
		changes: (listener: () => void) => Cleanup,
		pause: Pause,
		guard: Guard,
		track: (stop: Cleanup) => Cleanup
	) =>
	(fn: () => unknown): Cleanup => {
		const run = guard(fn);
		return track(
			whileShown(pause, (resumed) => {
				// Show the changes that came while it wated.
				if (resumed) run();
				// `fn` gets no arguments. The widget reads the new values itself.
				return changes(() => run());
			})
		);
	};
