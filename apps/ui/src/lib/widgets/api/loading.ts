import type { Scope } from '../scope';

/** What the card of a widget shows for `api.loading`. `undefined` shows nothing. */
export type LoadingLook = 'dots' | 'bar' | undefined;

/**
 * Makes `api.loading` for the widget of `scope`. It calls `show` when the look changes. Until
 * the first load ends the look is `dots`, then `bar`. On unmount it shows nothing, and a load
 * that settles after it does not change the look.
 */
export function createLoading(scope: Scope, show: (look: LoadingLook) => void) {
	let pending = 0;
	let loaded = false;
	scope.add(() => show(undefined));

	return <T>(task: Promise<T>): Promise<T> => {
		if (scope.disposed) return task;
		if (pending++ === 0) show(loaded ? 'bar' : 'dots');
		const done = () => {
			if (scope.disposed || --pending > 0) return;
			loaded = true;
			show(undefined);
		};
		// The caller still gets the rejection, because it awaits `task` and not this chain. A plain
		// JavaScript widget can pass a value that is not a promise.
		Promise.resolve(task).then(done, done);
		return task;
	};
}
