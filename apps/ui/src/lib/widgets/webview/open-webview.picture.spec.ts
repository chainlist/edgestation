import { describe, expect, it, vi } from 'vitest';
import { flush } from '#lib/test-helpers.ts';
import { aboutToMove, layoutChanged, setMoving, viewsPlaced } from './motion';
import {
	calls,
	host,
	nextPage,
	onScreen,
	open,
	picture,
	show,
	square,
	standIn,
	target
} from './open-webview.stubs';

vi.mock('@tauri-apps/api/core', async () => (await import('./open-webview.mocks')).coreModule());
vi.mock('./stand-in', async () => (await import('./open-webview.mocks')).standInModule());
vi.mock('./corner-buttons', async () =>
	(await import('./open-webview.mocks')).cornerButtonsModule()
);

describe('openWebview', () => {
	it('hides behind its picture when its page fades out, and shows when it fades in', async () => {
		const el = target(onScreen);
		await show(el);
		el.visible = false;
		layoutChanged();
		await flush();
		el.visible = true;
		layoutChanged();
		await flush();
		expect(calls()).toEqual([
			['webview_capture', { label: 'view-1' }],
			['webview_place', { label: 'view-1', bounds: null }],
			['webview_place', { label: 'view-1', bounds: square(onScreen) }]
		]);
	});

	it('hides behind its picture while the pages move, then shows at the new place', async () => {
		const el = target(onScreen);
		await show(el);
		setMoving(true);
		el.rect = { ...onScreen, x: 400 };
		setMoving(false);
		await flush();
		expect(calls()).toEqual([
			['webview_capture', { label: 'view-1' }],
			['webview_place', { label: 'view-1', bounds: null }],
			['webview_place', { label: 'view-1', bounds: square({ ...onScreen, x: 400 }) }]
		]);
		expect(standIn().show).toHaveBeenCalledExactlyOnceWith(picture);
	});

	it('hides behind its picture when its page leaves the screen', async () => {
		const el = target(onScreen);
		await show(el);
		el.rect = nextPage;
		layoutChanged();
		await flush();
		expect(calls()).toEqual([
			['webview_capture', { label: 'view-1' }],
			['webview_place', { label: 'view-1', bounds: null }]
		]);
	});

	it('takes its picture when the pages may move, and hides with it at once', async () => {
		await show(target(onScreen));
		aboutToMove();
		await flush();
		expect(calls()).toEqual([['webview_capture', { label: 'view-1' }]]);
		setMoving(true);
		await flush();
		expect(calls().slice(1)).toEqual([['webview_place', { label: 'view-1', bounds: null }]]);
	});

	it('takes a new picture once the last one is old', async () => {
		const now = vi.spyOn(performance, 'now').mockReturnValue(0);
		await show(target(onScreen));
		aboutToMove();
		await flush();
		now.mockReturnValue(5000);
		setMoving(true);
		await flush();
		expect(calls()).toEqual([
			['webview_capture', { label: 'view-1' }],
			['webview_capture', { label: 'view-1' }],
			['webview_place', { label: 'view-1', bounds: null }]
		]);
	});

	it('takes no picture while hidden', async () => {
		open(target(nextPage));
		aboutToMove();
		setMoving(true);
		await flush();
		expect(calls()).toEqual([]);
	});

	it('still hides, without an error, when its picture fails', async () => {
		vi.spyOn(console, 'warn').mockImplementation(() => {});
		host.mockImplementation(async (command) => {
			if (command === 'webview_capture') throw 'no picture';
			return command === 'webview_open' ? 'view-1' : undefined;
		});
		const fail = vi.fn();
		await show(target(onScreen), fail);
		setMoving(true);
		await flush();
		expect(calls().at(-1)).toEqual(['webview_place', { label: 'view-1', bounds: null }]);
		expect(fail).not.toHaveBeenCalled();
	});

	it('tells when every view is hidden', async () => {
		let pictured!: () => void;
		host.mockImplementation((command) =>
			command === 'webview_capture'
				? new Promise((resolve) => (pictured = () => resolve(picture)))
				: Promise.resolve(command === 'webview_open' ? 'view-1' : undefined)
		);
		await show(target(onScreen));
		setMoving(true);
		const hidden = vi.fn();
		void viewsPlaced().then(hidden);
		await flush();
		expect(hidden).not.toHaveBeenCalled();
		pictured();
		await flush();
		expect(hidden).toHaveBeenCalled();
		expect(calls().at(-1)).toEqual(['webview_place', { label: 'view-1', bounds: null }]);
	});

	it('removes its picture when closed', () => {
		open(target(onScreen))();
		expect(standIn().remove).toHaveBeenCalled();
	});

	it('leaves the host alone when nothing moved', async () => {
		open(target(onScreen));
		layoutChanged();
		await flush();
		expect(calls()).toEqual([]);
	});
});
