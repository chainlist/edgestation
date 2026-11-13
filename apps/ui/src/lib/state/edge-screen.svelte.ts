import { createSubscriber } from 'svelte/reactivity';
import { EDGE_ASPECT, XENEON_EDGE } from '#lib/layout/grid.ts';
import { watchEdgeSize } from '#lib/tauri/edge-window.ts';

let aspect = $state(EDGE_ASPECT);
let shortSide = $state(Math.min(XENEON_EDGE.width, XENEON_EDGE.height));

// Watch the Edge window only while something on the screen reads its shape.
const subscribe = createSubscriber(() =>
	watchEdgeSize(({ width, height }) => {
		// A minimized window has no size, so keep the last size.
		if (width > 0 && height > 0) {
			aspect = width / height;
			shortSide = Math.min(width, height);
		}
	})
);

/** The shape of the screen that shows the Edge pages. */
export const edgeScreen = {
	/** The width of the Edge window divided by its height. */
	get aspect() {
		subscribe();
		return aspect;
	},
	/**
	 * The shorter side of the Edge window in CSS pixels. One `vmin` or `cqmin` on the Edge is a
	 * hundreth of it. Until the window size is known, it is 720, the value of a Xeneon Edge at
	 * 100 % scaling.
	 */
	get shortSide() {
		subscribe();
		return shortSide;
	}
};
