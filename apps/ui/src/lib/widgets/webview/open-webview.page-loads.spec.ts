import { Channel } from '@tauri-apps/api/core';
import { describe, expect, it, vi } from 'vitest';
import { flush } from '#lib/test-helpers.ts';
import { layoutChanged } from './motion';
import {
	calls,
	host,
	loading,
	nextPage,
	onScreen,
	open,
	pageLoad,
	square,
	target
} from './open-webview.stubs';

vi.mock('@tauri-apps/api/core', async () => (await import('./open-webview.mocks')).coreModule());
vi.mock('./stand-in', async () => (await import('./open-webview.mocks')).standInModule());
vi.mock('./corner-buttons', async () =>
	(await import('./open-webview.mocks')).cornerButtonsModule()
);

describe('the page loads of a view', () => {
	it('start with the view, so the dots show while it opens', () => {
		open(target(onScreen));
		expect(loading).toHaveBeenCalledOnce();
	});

	it('keep it out of its place until the first one ends', async () => {
		open(target(onScreen));
		expect(host).toHaveBeenCalledWith('webview_open', {
			url: 'https://www.youtube.com/',
			bounds: square(onScreen),
			userAgent: null,
			profile: null,
			onNavigate: expect.any(Channel),
			onLoad: expect.any(Channel),
			onFullscreen: expect.any(Channel)
		});
		pageLoad(1, false);
		await flush();
		expect(calls()).toEqual([]);
		pageLoad(1, true);
		await flush();
		expect(calls()).toEqual([['webview_place', { label: 'view-1', bounds: square(onScreen) }]]);
	});

	it('wait for a target on another page to come on screen', () => {
		const el = target(nextPage);
		open(el);
		expect(host).not.toHaveBeenCalled();
		el.rect = onScreen;
		layoutChanged();
		expect(host).toHaveBeenCalledWith(
			'webview_open',
			expect.objectContaining({ bounds: square(onScreen) })
		);
	});

	it('wait for a target on a faded out page to fade in', () => {
		const el = target(onScreen);
		el.visible = false;
		open(el);
		expect(host).not.toHaveBeenCalled();
		el.visible = true;
		layoutChanged();
		expect(host).toHaveBeenCalledOnce();
	});

	it('hide it, with its picture, if its page left the screen before the first one ended', async () => {
		const el = target(onScreen);
		open(el);
		pageLoad(1, false);
		el.rect = nextPage;
		layoutChanged();
		await flush();
		expect(calls()).toEqual([]);
		pageLoad(1, true);
		await flush();
		expect(calls()).toEqual([
			['webview_capture', { label: 'view-1' }],
			['webview_place', { label: 'view-1', bounds: null }]
		]);
	});

	it('show with api.loading from their start to their end', async () => {
		open(target(onScreen));
		pageLoad(1, false);
		expect(loading).toHaveBeenCalledTimes(2);
		const settled = vi.fn();
		void loading.mock.calls[1][0].then(settled);
		pageLoad(1, true);
		await flush();
		expect(settled).toHaveBeenCalled();
	});

	it('end when the view closes, and the loads after are ignored', async () => {
		const close = open(target(onScreen));
		const settled = vi.fn();
		void loading.mock.calls[0][0].then(settled);
		close();
		pageLoad(1, false);
		await flush();
		expect(settled).toHaveBeenCalled();
		expect(loading).toHaveBeenCalledOnce();
	});
});
