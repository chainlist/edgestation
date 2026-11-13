import { OverlayScrollbars } from 'overlayscrollbars';

/**
 * Replaces the native scrollbars of `viewport`, a scrolling element, with overlay scrollbars
 * (see `scrollbars.css`). They show when it scrolls or when the pointer moves over it, but the
 * scroll stays native. The parent shows the scrollbars and must hold nothing else. The parent
 * takes the place of `viewport` in the layout, and `viewport` fills it. Returns a function that
 * restores the native scrollbars.
 *
 * Also a Svelte attachment: `<div class="flex-1"><main {@attach overlayScrollbars}>`.
 */
export function overlayScrollbars(viewport: HTMLElement): () => void {
	const target = viewport.parentElement;
	if (!target) throw new Error('Overlay scrollbars need the scrolling element to have a parent');

	// Give the viewport to the library, so that it does not make its own and move the children of
	// the element. Svelte continues to insert into the element.
	const instance = OverlayScrollbars(
		{ target, elements: { viewport } },
		{ scrollbars: { theme: 'os-theme-edgestation', autoHide: 'move', autoHideDelay: 800 } }
	);
	return () => instance.destroy();
}
