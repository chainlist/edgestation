/** What the page loads of a view need from `api.loading`. */
export type Loading = (load: Promise<void>) => unknown;

/** The start or the end of a page load in a native view, as the host reports it. */
export interface PageLoad {
	/** The same for the start and the end of one load. */
	id: number;
	done: boolean;
}

/**
 * Shows the page loads of a native view with `loading` (`api.loading`). The first load starts
 * now, before the host reports it, because the view takes a moment to open. `firstLoaded` runs
 * when a reported load ends for the first time, with or without success.
 */
export function trackPageLoads(loading: Loading, firstLoaded: () => void) {
	const open = new Map<number, () => void>();
	let first: (() => void) | undefined;
	loading(new Promise<void>((resolve) => (first = resolve)));

	return {
		report({ id, done }: PageLoad) {
			if (!done) {
				// A redirect starts the same load again.
				if (open.has(id)) return;
				const load = Promise.withResolvers<void>();
				open.set(id, load.resolve);
				loading(load.promise);
				return;
			}
			const end = open.get(id);
			if (!end) return;
			open.delete(id);
			end();
			if (first) {
				first();
				first = undefined;
				firstLoaded();
			}
		},
		/** Ends the loads that are still open, because the view closes. */
		end() {
			first?.();
			first = undefined;
			for (const end of open.values()) end();
			open.clear();
		}
	};
}
