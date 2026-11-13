import { Channel, invoke } from '@tauri-apps/api/core';
import { lendFocusToView } from '#lib/tauri/focus.ts';
import type { Cleanup, WebviewOptions } from '../types';
import { createHomeButton, createKeyboardButton } from './corner-buttons';
import { buttonHole, type Hole } from './holes';
import { mobileUserAgent } from './mobile-user-agent';
import { isMoving, overlayElements, trackView } from './motion';
import { trackPageLoads, type Loading, type PageLoad } from './page-loads';
import { createStandIn } from './stand-in';

/** How long a picture, taken while the view shows, can replace the view after it hides. */
const FRESH_MS = 1000;

/**
 * A box in CSS pixels from the top left of the window. `radius` rounds its corners, and `holes`
 * are cut out of it.
 */
interface Bounds {
	x: number;
	y: number;
	width: number;
	height: number;
	radius: number;
	holes: Hole[];
}

/**
 * Returns `url` in the form that the host reprots after the load. For example,
 * `https://Example.com` becomes `https://example.com/`.
 */
const normalized = (url: string) => {
	try {
		return new URL(url).href;
	} catch {
		return url;
	}
};

/** A computed length in CSS pixels. Other units give 0, for example a percentage radius. */
const px = (value: string) => (value.endsWith('px') ? parseFloat(value) : 0);

/**
 * Returns the element that holds `el` on the page. This is its parent, or the shadow root host
 * if `el` is at the top of a shadow root. Widgets render in one. Returns `null` at the top of
 * the page.
 */
const holder = (el: Element): Element | null =>
	el.parentElement ?? (el.parentNode as ShadowRoot | null)?.host ?? null;

/**
 * Returns the corner radius of a view over `target`, in CSS pixels. The view draws above the
 * page, so the rounded corners of the widget cell cannot clip it. It takes the inner radius of
 * the nearest rounded box that clips `target`, less the inset of `target` from that box corner.
 * If the radius of `target` is larger, it takes that. All four corners use the radius of the
 * corner that is nearest to a box corner.
 */
const cornerRadius = (target: HTMLElement, rect: DOMRect) => {
	const own = px(getComputedStyle(target).borderTopLeftRadius);
	for (let el = holder(target); el; el = holder(el)) {
		const style = getComputedStyle(el);
		const outer = px(style.borderTopLeftRadius);
		if (style.overflowX === 'visible' || outer <= 0) continue;
		const cell = el.getBoundingClientRect();
		// The inset of `target` from each side of the box, inside its border.
		const left = rect.left - cell.left - px(style.borderLeftWidth);
		const top = rect.top - cell.top - px(style.borderTopWidth);
		const right = cell.right - px(style.borderRightWidth) - rect.right;
		const bottom = cell.bottom - px(style.borderBottomWidth) - rect.bottom;
		const inset = Math.min(
			Math.max(left, top),
			Math.max(right, top),
			Math.max(left, bottom),
			Math.max(right, bottom)
		);
		const border = Math.max(px(style.borderTopWidth), px(style.borderLeftWidth));
		return Math.max(own, outer - border - Math.max(inset, 0));
	}
	return own;
};

/**
 * Returns the holes that a view at `rect` leaves where page elements over the widgets
 * (`overlay`) cover it. The holes get the largest corner radius of each elment.
 */
function overlayHoles(rect: DOMRect): Hole[] {
	const holes: Hole[] = [];
	for (const el of overlayElements()) {
		const o = el.getBoundingClientRect();
		if (o.left >= rect.right || o.right <= rect.left) continue;
		if (o.top >= rect.bottom || o.bottom <= rect.top) continue;
		const style = getComputedStyle(el);
		const radius = Math.max(
			px(style.borderTopLeftRadius),
			px(style.borderTopRightRadius),
			px(style.borderBottomRightRadius),
			px(style.borderBottomLeftRadius)
		);
		const { left, top, width, height } = o;
		holes.push({ x: left - rect.left, y: top - rect.top, width, height, radius });
	}
	return holes;
}

/** Where a view over `el` shows. It has holes for its first `buttons` buttons and for overlays. */
const box = (el: HTMLElement, buttons: number): Bounds => {
	const rect = el.getBoundingClientRect();
	const { left, top, width, height } = rect;
	const holes = Array.from({ length: buttons }, (_, slot) => buttonHole(width, height, slot));
	holes.push(...overlayHoles(rect));
	return { x: left, y: top, width, height, radius: cornerRadius(el, rect), holes };
};

/**
 * Tells if a native view can show over `target` at `b`. The view draws above the page, so it
 * cannot slide or fade with the page, and the page edges cannot cut it. So it shows only while
 * the pages do not move, `target` is visible and `b` is fully in the window. The check allows
 * 1 px of error for rounded values. Off-screen pages are outside the window. Faded out pages on
 * screen are hidden.
 */
const fits = (target: HTMLElement, b: Bounds) =>
	!isMoving() &&
	target.checkVisibility({ visibilityProperty: true }) &&
	b.width > 0 &&
	b.height > 0 &&
	b.x >= -1 &&
	b.y >= -1 &&
	b.x + b.width <= innerWidth + 1 &&
	b.y + b.height <= innerHeight + 1;

/**
 * Opens web page `url` in a native view over `target`, in browser profile `profile`
 * (`undefined` for Default). The view stays their when the Edge layout changes. Errors from the
 * host go to `fail`. Its page loads go to `loading` (`api.loading`). Returns the function that
 * closes the view.
 */
export function openWebview(
	target: HTMLElement,
	url: string,
	options: WebviewOptions,
	profile: string | undefined,
	fail: (error: unknown) => void,
	loading: Loading
): Cleanup {
	let label: string | undefined;
	let closed = false;

	// The Edge does not take the keyboard on a touch, and the page cannot tell when a text field
	// in the view gets a touch. So the keyboard button gives the keyboard to the view.
	const keyboard = createKeyboardButton(target, () => label && lendFocusToView(label));
	keyboard.show(true);

	/** The bounds where it shows, as JSON. `null` while it hides, `undefined` before the first. */
	let shown: string | null | undefined;
	// Until its first page loads, it draws outside the window (see `webview_open`). Thus the card
	// shows the dots, and the view comes in painted, not blank.
	let loaded = false;
	const loads = trackPageLoads(loading, () => {
		loaded = true;
		void update();
	});
	const onLoad = new Channel<PageLoad>((load) => closed || loads.report(load));

	// While the view is away from `url`, its Home button shows above the keyboard button. The
	// view is at home on `url` and on the page where its last load of `url` landed (`landing`,
	// `null` until it lands). This covers redirects such as `https://example.com/` to
	// `https://www.example.com/`.
	const home = options.homeButton ? createHomeButton(target, () => goHome()) : null;
	let away = false;
	const start = normalized(url);
	let landing: string | null = null;
	const onNavigate = new Channel<string>((current) => {
		// It starts blank, then loads `url`.
		if (!home || closed || current === 'about:blank') return;
		landing ??= current;
		setAway(current !== start && current !== landing);
	});

	// While the page shows an element full screen, for example a video, the buttons hide.
	// Otherwise they cover the controls of the page (see `keep_fullscreen` in the host).
	let fullscreen = false;
	const onFullscreen = new Channel<boolean>((value) => closed || setFullscreen(value));

	/** Settles when the view is open, or when it failled to open. `undefined` before the open. */
	let opened: Promise<void> | undefined;
	let queue: Promise<unknown> = Promise.resolve();
	// It opens when its target shows on screen for the first time. Thus the views on pages that
	// nobody looks at cost nothing.
	const open = (bounds: Bounds) => {
		queue = opened = invoke<string>('webview_open', {
			url,
			bounds,
			userAgent: options.mobile ? mobileUserAgent() : null,
			profile: profile ?? null,
			onNavigate,
			onLoad,
			onFullscreen
		}).then((value) => void (label = value), fail);
	};
	// Send one host call at a time, so that a hide cannot overtake the show before it. Skip the
	// call if the view failed to open or is closed. Settles when the call is done.
	const send = (call: (label: string) => Promise<unknown>) =>
		(queue = queue
			.then(() => !closed && label && call(label))
			.catch((error) => closed || fail(error)));

	// The view takes its picture when it hides, or earlier if the pages can move soon.
	const standIn = createStandIn(target);
	/** The time of the picture under the view. `null` after the view shows again. */
	let pictured: number | null = null;
	let picturing = false;
	const fresh = () => pictured !== null && performance.now() - pictured < FRESH_MS;
	const picture = async (label: string) => {
		try {
			await standIn.show(await invoke<ArrayBuffer>('webview_capture', { label }));
			pictured = performance.now();
		} catch (error) {
			// Without a picture, the content of the widget shows while the view hides.
			console.warn('No picture of the web view:', error);
		}
	};

	const update = () => {
		const current = box(target, fullscreen ? 0 : home && away ? 2 : 1);
		standIn.place();
		keyboard.place();
		home?.place();
		if (!opened) {
			if (fits(target, current)) open(current);
			return;
		}
		if (!loaded) return;
		const bounds = fits(target, current) ? current : null;
		const next = bounds && JSON.stringify(bounds);
		if (next === shown) return;
		// After the first load, a view that cannot show hides too, so it stops to draw.
		const hiding = shown !== null && bounds === null;
		shown = next;
		return send(async (label) => {
			if (hiding && !fresh()) await picture(label);
			if (closed) return;
			// A view that shows again needs a new pictrue the next time it hides.
			if (bounds) pictured = null;
			await invoke('webview_place', { label, bounds });
		});
	};
	const stopTracking = trackView({
		place: update,
		picture() {
			if (!shown || picturing || fresh()) return;
			picturing = true;
			void send(picture).finally(() => (picturing = false));
		}
	});
	const resize = new ResizeObserver(update);
	resize.observe(target);

	const setAway = (value: boolean) => {
		if (value === away) return;
		away = value;
		home?.show(value && !fullscreen);
		void update();
	};
	const setFullscreen = (value: boolean) => {
		if (value === fullscreen) return;
		fullscreen = value;
		keyboard.show(!value);
		home?.show(away && !value);
		void update();
	};
	const goHome = () => {
		// The page where this load lands is also home.
		landing = null;
		setAway(false);
		void send((label) => invoke('webview_navigate', { label, url }));
	};
	void update();

	return () => {
		// This runs two times on unmount, from the widget and from its scope.
		if (closed) return;
		closed = true;
		loads.end();
		stopTracking();
		resize.disconnect();
		standIn.remove();
		keyboard.remove();
		home?.remove();
		// Close after the open, but not after the queued calls. If the host never answers a
		// picture call, the view stays on the Edge after its widget is gone.
		void opened?.then(() => label && invoke('webview_close', { label })).catch(fail);
	};
}
