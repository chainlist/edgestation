import type { Cleanup } from '../types';

/**
 * Tells if the pages slide (a swipe, or the snap after it) or a page dot is pressed. It also
 * tells when the Edge layout changes. Native views (`api.webview`) draw above the page and
 * cannot slide with it. While the page moves, they hide behind a picture of themselves, and the
 * picture slides in their place. After each change, they measure their place again.
 */
let moving = false;

/** A native view, as the carousel drives it. */
export interface NativeView {
	/** Measures its place again, then shows or hides. Settles when the host has done it. */
	place(): Promise<unknown> | void;
	/** Takes its picture now if it shows, so that it can hide immediately when the pages move. */
	picture(): void;
}
const views = new Set<NativeView>();
let placed: Promise<unknown> = Promise.resolve();

export const isMoving = () => moving;

export function setMoving(value: boolean) {
	if (value === moving) return;
	moving = value;
	layoutChanged();
}

/** Tells native views that widgets may have moved or changed page. */
export function layoutChanged() {
	placed = Promise.all([...views].map((view) => view.place()));
}

/** Resolves when each native view is were the last change put it. Views hide while pages move. */
export function viewsPlaced(): Promise<void> {
	return placed.then(() => {});
}

/** Two fingers are down, so the pages can move soon. Visible native views take their picture now. */
export function aboutToMove() {
	for (const view of views) view.picture();
}

export function trackView(view: NativeView): Cleanup {
	views.add(view);
	return () => views.delete(view);
}

/** Page elements that draw over the widgets, such as the Paused badge. */
const overlays = new Set<Element>();

/**
 * Lays page element `el` over native views until the returned function runs. The views leave a
 * hole where it covers them.
 */
export function overlay(el: Element): Cleanup {
	overlays.add(el);
	layoutChanged();
	return () => {
		overlays.delete(el);
		layoutChanged();
	};
}

export const overlayElements = (): Iterable<Element> => overlays;
