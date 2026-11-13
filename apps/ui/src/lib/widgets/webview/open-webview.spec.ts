import { describe, expect, it, vi } from 'vitest';
import { aboutToMove, layoutChanged, setMoving, viewsPlaced } from './motion';
import {
	calls,
	cell,
	element,
	flush,
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
	it('rounds its corners to the inside of the cell that clips it', () => {
		const el = target(onScreen);
		el.parentElement = cell({ x: 9, y: 9, width: 602, height: 342 });
		open(el);
		expect(host.mock.calls[0][1]).toMatchObject({ bounds: { ...onScreen, radius: 15 } });
	});

	it('finds its cell from inside the shadow root its widget renders in', () => {
		const el = Object.assign(target(onScreen), { parentNode: { host: {} } });
		const mount = element(onScreen);
		el.parentNode.host = mount;
		mount.parentElement = cell({ x: 9, y: 9, width: 602, height: 342 });
		open(el);
		expect(host.mock.calls[0][1]).toMatchObject({ bounds: { radius: 15 } });
	});

	it('rounds its corners less the further in from the cell corners it sits', () => {
		const el = target(onScreen);
		// 5 px in from each side, inside the border of the cell.
		el.parentElement = cell({ x: 4, y: 4, width: 612, height: 352 });
		open(el);
		expect(host.mock.calls[0][1]).toMatchObject({ bounds: { radius: 10 } });
	});

	it('rounds all its corners like the one nearest a cell corner', () => {
		const el = target(onScreen);
		// 40 px down from the top, under a title, and against the other sides.
		el.parentElement = cell({ x: 9, y: -31, width: 602, height: 382 });
		open(el);
		expect(host.mock.calls[0][1]).toMatchObject({ bounds: { radius: 15 } });
	});

	it('rounds its corners anew when its cell changes', async () => {
		const el = target(onScreen);
		const parent = cell({ x: 9, y: 9, width: 602, height: 342 });
		el.parentElement = parent;
		await show(el);
		parent.style.borderTopLeftRadius = '9px';
		layoutChanged();
		await flush();
		expect(calls()).toEqual([
			['webview_place', { label: 'view-1', bounds: { ...square(onScreen), radius: 8 } }]
		]);
	});

	it('keeps the radius of its target when no cell rounds it', () => {
		const el = target(onScreen);
		el.style.borderTopLeftRadius = '8px';
		el.parentElement = element({ x: 0, y: 0, width: 1280, height: 360 });
		open(el);
		expect(host.mock.calls[0][1]).toMatchObject({ bounds: { radius: 8 } });
	});

	it('opens in the given profile', () => {
		open(target(onScreen), vi.fn(), {}, 'a1b2c3d4');
		expect(host.mock.calls[0][1]).toMatchObject({ profile: 'a1b2c3d4' });
	});

	it('opens as Chrome on a phone, at the app engine version, when mobile', () => {
		vi.stubGlobal('navigator', {
			userAgent:
				'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36 Edg/142.0.0.0'
		});
		open(target(onScreen), vi.fn(), { mobile: true });
		expect(host.mock.calls[0][1]).toMatchObject({
			userAgent: {
				userAgent:
					'Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Mobile Safari/537.36',
				// The client hints must also say phone, because Google sites trust them frist.
				userAgentMetadata: {
					brands: expect.arrayContaining([{ brand: 'Google Chrome', version: '142' }]),
					fullVersionList: expect.arrayContaining([
						{ brand: 'Google Chrome', version: '142.0.0.0' }
					]),
					formFactors: ['Mobile'],
					platform: 'Android',
					mobile: true
				}
			}
		});
	});

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

	it('closes once open, even when closed first', async () => {
		let opened!: (label: string) => void;
		host.mockImplementationOnce(() => new Promise((resolve) => (opened = resolve)));
		const close = open(target(onScreen));
		close();
		await flush();
		expect(calls()).toEqual([]);
		opened('view-1');
		await flush();
		expect(calls()).toEqual([['webview_close', { label: 'view-1' }]]);
	});

	it('closes at once, even while its picture is still being taken', async () => {
		host.mockImplementation((command) =>
			command === 'webview_capture'
				? new Promise(() => {}) // The host never answers.
				: Promise.resolve(command === 'webview_open' ? 'view-1' : undefined)
		);
		const close = await show(target(onScreen));
		aboutToMove();
		setMoving(true);
		await flush();
		close();
		await flush();
		expect(calls()).toEqual([
			['webview_capture', { label: 'view-1' }],
			['webview_close', { label: 'view-1' }]
		]);
	});

	it('skips the calls still queued once closed', async () => {
		let pictured!: () => void;
		host.mockImplementation((command) =>
			command === 'webview_capture'
				? new Promise((resolve) => (pictured = () => resolve(picture)))
				: Promise.resolve(command === 'webview_open' ? 'view-1' : undefined)
		);
		const fail = vi.fn();
		const close = await show(target(onScreen), fail);
		setMoving(true);
		await flush();
		close();
		pictured();
		await flush();
		expect(calls()).toEqual([
			['webview_capture', { label: 'view-1' }],
			['webview_close', { label: 'view-1' }]
		]);
		expect(fail).not.toHaveBeenCalled();
	});

	it('closes only once when closed twice', async () => {
		const close = open(target(onScreen));
		await flush();
		close();
		close();
		await flush();
		expect(calls()).toEqual([['webview_close', { label: 'view-1' }]]);
	});

	it('reports a failed open once and calls the host no more', async () => {
		host.mockImplementationOnce(async () => {
			throw 'not a web page';
		});
		const fail = vi.fn();
		const el = target(onScreen);
		const close = open(el, fail);
		el.rect = nextPage;
		layoutChanged();
		close();
		await flush();
		expect(fail).toHaveBeenCalledExactlyOnceWith('not a web page');
		expect(calls()).toEqual([]);
	});
});
