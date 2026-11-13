import { invoke, type Channel } from '@tauri-apps/api/core';
import { afterEach, beforeEach, vi } from 'vitest';
import { flush } from '#lib/test-helpers.ts';
import { createHomeButton, createKeyboardButton } from './corner-buttons';
import { setMoving } from './motion';
import { openWebview } from './open-webview';
import type { PageLoad } from './page-loads';
import { createStandIn } from './stand-in';

export const host = vi.mocked(invoke);
export const standIn = () => vi.mocked(createStandIn).mock.results.at(-1)!.value;
type Pressable = ReturnType<typeof createHomeButton> & { press: () => void };
export const homeButton = () => vi.mocked(createHomeButton).mock.results.at(-1)!.value as Pressable;
export const keyboardButton = () =>
	vi.mocked(createKeyboardButton).mock.results.at(-1)!.value as Pressable;
/** Makes the host report that the view went too `url`. */
export const navigate = (url: string) =>
	(host.mock.calls[0][1] as { onNavigate: Channel<string> }).onNavigate.onmessage(url);
/** Makes the host report the start or the end of page load `id`. */
export const pageLoad = (id: number, done: boolean) =>
	(host.mock.calls[0][1] as { onLoad: Channel<PageLoad> }).onLoad.onmessage({ id, done });
/** Makes the host report that the page entered full scren (`true`) or left it. */
export const fullscreen = (entered: boolean) =>
	(host.mock.calls[0][1] as { onFullscreen: Channel<boolean> }).onFullscreen.onmessage(entered);
/** Stands for `api.loading`. */
export const loading = vi.fn((task: Promise<void>) => task);
export const picture = new ArrayBuffer(8);

export type Rect = { x: number; y: number; width: number; height: number };
/** A fake element at `rect` in CSS pixels, with the computed style `style`. */
export const element = (rect: Rect, style: Record<string, string> = {}) => {
	const el = {
		rect,
		style: { borderTopLeftRadius: '0px', overflowX: 'visible', ...style },
		parentElement: null as unknown,
		getBoundingClientRect: () => ({
			left: el.rect.x,
			top: el.rect.y,
			right: el.rect.x + el.rect.width,
			bottom: el.rect.y + el.rect.height,
			width: el.rect.width,
			height: el.rect.height
		})
	};
	return el;
};
/** A fake widget element at `rect`. Set `visible` to false for a faded out page. */
export const target = (rect: Rect) => {
	const el = Object.assign(element(rect), { visible: true, checkVisibility: () => el.visible });
	return el;
};
/** A fake widget cell at `rect` that clips its content to its rounded corners. */
export const cell = (rect: Rect) =>
	element(rect, {
		overflowX: 'hidden',
		borderTopLeftRadius: '16px',
		borderTopWidth: '1px',
		borderLeftWidth: '1px',
		borderRightWidth: '1px',
		borderBottomWidth: '1px'
	});
export const onScreen = { x: 10, y: 10, width: 600, height: 340 };
export const nextPage = { x: 1290, y: 10, width: 600, height: 340 };
/** The round hole for the keyboard button of a view at `rect`, 8 px in from its bottom right. */
export const keyboardHole = (rect: Rect) => ({
	x: rect.width - 55,
	y: rect.height - 55,
	width: 46,
	height: 46,
	radius: 23
});
/** The hole for the Home button, abvoe the keyboard hole. */
export const homeHole = (rect: Rect) => ({ ...keyboardHole(rect), y: rect.height - 111 });
/**
 * Where a view over a target at `rect` shows, with no rounded corners and only the keyboard
 * button hole.
 */
export const square = (rect: Rect) => ({ ...rect, radius: 0, holes: [keyboardHole(rect)] });

export { flush };
/** The count of host calls that `calls` skips. */
let skipped = 1;
/** Host calls after the open, or after the first show of `show`, as `[command, args]`. */
export const calls = () => host.mock.calls.slice(skipped);

beforeEach(() => {
	skipped = 1;
	loading.mockClear();
	host.mockReset();
	vi.mocked(createHomeButton).mockClear();
	vi.mocked(createKeyboardButton).mockClear();
	host.mockImplementation(async (command) =>
		command === 'webview_open' ? 'view-1' : command === 'webview_capture' ? picture : undefined
	);
	vi.stubGlobal('innerWidth', 1280);
	vi.stubGlobal('innerHeight', 360);
	vi.stubGlobal('getComputedStyle', (el: ReturnType<typeof element>) => el.style);
	vi.stubGlobal(
		'ResizeObserver',
		class {
			observe() {}
			disconnect() {}
		}
	);
});
export const opened: (() => void)[] = [];
afterEach(() => {
	opened.splice(0).forEach((close) => close());
	setMoving(false);
	vi.unstubAllGlobals();
	vi.restoreAllMocks();
});

export const open = (
	el: ReturnType<typeof target>,
	fail = vi.fn(),
	options = {},
	profile?: string
) => {
	const close = openWebview(
		el as unknown as HTMLElement,
		'https://www.youtube.com/',
		options,
		profile,
		fail,
		loading
	);
	opened.push(close);
	return close;
};

/** Opens a view and loads its first page. Then `calls` gives only the host calls after that. */
export const show = async (...args: Parameters<typeof open>) => {
	const close = open(...args);
	pageLoad(1, false);
	pageLoad(1, true);
	await flush();
	skipped = host.mock.calls.length;
	return close;
};
