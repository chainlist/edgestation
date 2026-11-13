import type { Cleanup } from './types';

/** The lifetime of one mounted widget. It collects the cleanups and sends the errors to `fail`. */
export interface Scope {
	readonly disposed: boolean;
	/**
	 * Registers a cleanup. If the scope is already disposed, it runs the cleanup immediately.
	 * Returns a function that unregisters the cleanup and does not run it.
	 */
	add(fn: Cleanup): () => void;
	/**
	 * Wraps `fn` so that sync throws and rejected promises go to `fail`. After dispose, the wraper
	 * does nothing, and it ignores a promise that rejects later.
	 */
	guard<A extends unknown[]>(fn: (...args: A) => unknown): (...args: A) => void;
	dispose(): void;
}

const runSafely = (fn: Cleanup) => {
	try {
		fn();
	} catch (error) {
		console.error('Widget cleanup failed', error);
	}
};

export function createScope(fail: (error: unknown) => void): Scope {
	const cleanups: Cleanup[] = [];
	let disposed = false;

	return {
		get disposed() {
			return disposed;
		},
		add(fn) {
			if (disposed) runSafely(fn);
			else cleanups.push(fn);
			return () => {
				const index = cleanups.indexOf(fn);
				if (index !== -1) cleanups.splice(index, 1);
			};
		},
		guard:
			(fn) =>
			(...args) => {
				if (disposed) return;
				try {
					const result = fn(...args);
					// Ignore a late rejection, for example from a request that unmount cancelled.
					// Else the error card shows on the widget that mounts next in this place.
					if (result instanceof Promise)
						result.catch((error) => {
							if (!disposed) fail(error);
						});
				} catch (error) {
					fail(error);
				}
			},
		dispose() {
			disposed = true;
			cleanups.splice(0).reverse().forEach(runSafely);
		}
	};
}
