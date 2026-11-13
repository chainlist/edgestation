import { KEYBOARD_BUTTON } from '#lib/tauri/focus.ts';
import { widgetRoot } from '../shadow';
import { BUTTON_SIZE as SIZE, buttonSpot } from './holes';

/**
 * A round button of a native view (`api.webview`). It floats over the bottom right corner of
 * the view target, in the widget. The keyboard button is in the corner, and the Home button is
 * above it. The view draws above the page, so it leaves a round hole under each visible button.
 */
export interface CornerButton {
	show(visible: boolean): void;
	/** Moves to the target again if the target moved or changed size in the widget. */
	place(): void;
	remove(): void;
}

// The style is inline, so that the `button` rules of a widget cannot move or change it. The
// accent color makes it easy to see on all sites, dark or ligth. It is above the picture that
// replaces the view, which sits just under it.
const STYLE =
	'position:absolute;display:none;align-items:center;justify-content:center;' +
	`width:${SIZE}px;height:${SIZE}px;margin:0;padding:0;border:0;border-radius:50%;` +
	'background:var(--primary);color:var(--primary-foreground);cursor:pointer;' +
	'-webkit-tap-highlight-color:transparent;z-index:2147483647';

const icon = (paths: string) =>
	'<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" ' +
	`stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${paths}</svg>`;

// Lucide's `house`.
const HOUSE = icon(
	'<path d="M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8"/>' +
		'<path d="M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>'
);

// Lucide's `keyboard`.
const KEYBOARD = icon(
	'<path d="M10 8h.01"/><path d="M12 12h.01"/><path d="M14 8h.01"/><path d="M16 12h.01"/>' +
		'<path d="M18 8h.01"/><path d="M6 8h.01"/><path d="M7 16h10"/><path d="M8 12h.01"/>' +
		'<rect width="20" height="16" x="2" y="4" rx="2"/>'
);

function createCornerButton(
	target: HTMLElement,
	slot: number,
	title: string,
	glyph: string,
	press: () => void
): CornerButton & { el: HTMLButtonElement } {
	const button = document.createElement('button');
	button.type = 'button';
	button.title = title;
	button.ariaLabel = title;
	button.style.cssText = STYLE;
	button.innerHTML = glyph;
	button.addEventListener('click', press);

	/** Puts the button over its hole in the view, relative to the widget root that holds it. */
	const place = () => {
		const root = button.parentElement;
		if (!root) return;
		const t = target.getBoundingClientRect();
		const r = root.getBoundingClientRect();
		const { x, y } = buttonSpot(t.width, t.height, slot);
		button.style.left = `${t.left - r.left + x}px`;
		button.style.top = `${t.top - r.top + y}px`;
	};

	return {
		el: button,
		show(visible) {
			if (visible && !button.parentElement) widgetRoot(target)?.append(button);
			button.style.display = visible ? 'flex' : 'none';
			place();
		},
		place,
		remove() {
			button.remove();
		}
	};
}

/** A Home button for the view over `target`, above its keyboard button. */
export const createHomeButton = (target: HTMLElement, press: () => void): CornerButton =>
	createCornerButton(target, 1, 'Back to the start page', HOUSE, press);

/**
 * A keyboard button for the view over `target`. It calls `press` on a press. A touch on it does
 * not give the keybord back to the app in front, but a touch on the rest of the Edge does.
 */
export function createKeyboardButton(target: HTMLElement, press: () => void): CornerButton {
	const button = createCornerButton(target, 0, 'Type in this page', KEYBOARD, press);
	button.el.setAttribute(KEYBOARD_BUTTON, '');
	return button;
}
