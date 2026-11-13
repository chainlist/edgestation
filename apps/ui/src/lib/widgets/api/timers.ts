import type { Cleanup } from '../types';

/**
 * Tells if the widgets of one page are paused because the page is off screen. Their
 * `api.every` and `api.after` timers wait while it is paused.
 */
export interface Pause {
	readonly paused: boolean;
	set(paused: boolean): void;
}

interface Timer {
	pause: Pause;
	/** `every` timers also wait while the Edge is frozen. `after` timers do not. */
	freezes: boolean;
	running: boolean;
	/** Starts the native timer. `resumed` is true if the timer waited before. */
	start(resumed: boolean): void;
	stop(): void;
}

const timers = new Set<Timer>();
let frozen = false;

function sync(timer: Timer, resumed = true) {
	const run = !timer.pause.paused && !(timer.freezes && frozen);
	if (run === timer.running) return;
	timer.running = run;
	if (run) timer.start(resumed);
	else timer.stop();
}

function add(timer: Timer): Cleanup {
	timers.add(timer);
	sync(timer, false);
	return () => {
		timer.stop();
		timers.delete(timer);
	};
}

export function createPause(paused: boolean): Pause {
	const pause = {
		paused,
		set(value: boolean) {
			if (value === pause.paused) return;
			pause.paused = value;
			for (const timer of timers) if (timer.pause === pause) sync(timer);
		}
	};
	return pause;
}

/**
 * A `setInterval` that waits while its page is paused or the Edge is frozen. After the wait, it
 * ticks immediatly and then continues. Returns the function that clears it.
 */
export function interval(pause: Pause, ms: number, fn: () => void): Cleanup {
	let handle: ReturnType<typeof setInterval> | undefined;
	return add({
		pause,
		freezes: true,
		running: false,
		start(resumed) {
			// Start the interval before the catch-up tick. Then a tick that clears the timer
			// clears this interval.
			handle = setInterval(fn, ms);
			if (resumed) fn();
		},
		stop: () => clearInterval(handle)
	});
}

/**
 * A `setTimeout` that waits while its page is paused. If it comes due in that time, it fires
 * when the page shows again. The freeze does not stop it. Returns the function that cancels it.
 */
export function timeout(pause: Pause, ms: number, fn: () => void): Cleanup {
	const due = performance.now() + ms;
	let handle: ReturnType<typeof setTimeout> | undefined;
	const timer: Timer = {
		pause,
		freezes: false,
		running: false,
		start() {
			handle = setTimeout(() => {
				timers.delete(timer);
				fn();
			}, due - performance.now());
		},
		stop: () => clearTimeout(handle)
	};
	return add(timer);
}

/**
 * Runs `start` when the page shows and the Edge is not frozen, like an `every` timer. Runs the
 * function that `start` returns when one of the two stops. `resumed` is true if it waited
 * before. Returns the function that ends it.
 */
export function whileShown(pause: Pause, start: (resumed: boolean) => Cleanup): Cleanup {
	let stop: Cleanup | undefined;
	return add({
		pause,
		freezes: true,
		running: false,
		start(resumed) {
			stop = start(resumed);
		},
		stop() {
			stop?.();
			stop = undefined;
		}
	});
}

/**
 * Freezes or resumes the `every` timers of all widgets. The freeze is on while another app is
 * full screen. Then nothing polls the machine or draws again. On resume, each timer on an shown
 * page ticks immediately, then on its interval.
 */
export function setFrozen(value: boolean) {
	if (value === frozen) return;
	frozen = value;
	for (const timer of timers) sync(timer);
}
