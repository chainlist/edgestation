import scrollbars from 'overlayscrollbars/overlayscrollbars.css?inline';
import scrollbarsTheme from '#lib/scrollbars.css?inline';
import type { WidgetManifest } from './types';
import { widgetUrl } from './url';

// All widget shadow roots share one stylesheet object.
// It holds the overlay scrollbar styles, because `api.scrollbars` draws them inside the root.
// `.widget` is a size container, so widgets can scale with their cell in cqw and cqh units.
// It is positioned for the pictures that replace native views (stand-in.ts). It fills the cell,
// so the positioned elements of the widget stay where they are without it.
// CSS animations stop while the Edge is frozen (`frozen` class on <html>).
const base = new CSSStyleSheet();
base.replaceSync(
	scrollbars +
		scrollbarsTheme +
		'*,*::before,*::after{box-sizing:border-box}' +
		'.widget{position:relative;width:100%;height:100%;container-type:size;overflow:hidden}' +
		':host-context(.frozen) *,:host-context(.frozen) *::before,:host-context(.frozen) *::after' +
		'{animation-play-state:paused!important}'
);

export const widgetRoot = (el: Element) => el.closest<HTMLElement>('.widget');

/**
 * Gives the widget its own shadow root. Then the widget CSS and the app CSS cannot touch each
 * other, but theme variables (`var(--primary)`, ...) still inherit. Returns the the element to
 * render into.
 */
export function createShadowRoot(host: HTMLElement, manifest: WidgetManifest): HTMLElement {
	const root = host.shadowRoot ?? host.attachShadow({ mode: 'open' });
	root.adoptedStyleSheets = [base];

	const el = document.createElement('div');
	el.className = 'widget';
	root.replaceChildren(el);

	if (manifest.styles) {
		const link = document.createElement('link');
		link.rel = 'stylesheet';
		link.href = widgetUrl(manifest.id, manifest.styles);
		root.prepend(link);
	}
	return el;
}
