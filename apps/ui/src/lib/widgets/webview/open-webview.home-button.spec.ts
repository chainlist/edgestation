import { describe, expect, it, vi } from 'vitest';
import { flush } from '#lib/test-helpers.ts';
import { createHomeButton } from './corner-buttons';
import { openWebview } from './open-webview';
import {
	calls,
	fullscreen,
	homeButton,
	homeHole,
	keyboardHole,
	loading,
	navigate,
	onScreen,
	open,
	opened,
	show,
	square,
	target,
	type Rect
} from './open-webview.stubs';

vi.mock('@tauri-apps/api/core', async () => (await import('./open-webview.mocks')).coreModule());
vi.mock('./stand-in', async () => (await import('./open-webview.mocks')).standInModule());
vi.mock('./corner-buttons', async () =>
	(await import('./open-webview.mocks')).cornerButtonsModule()
);

describe('the Home button of a view', () => {
	const withHome = { homeButton: true };
	/** Where the view shows when it has holes for its keyboard and Home buttons. */
	const notched = (rect: Rect) => ({
		...square(rect),
		holes: [keyboardHole(rect), homeHole(rect)]
	});

	it('shows once the view leaves its page, in a hole the view leaves for it', async () => {
		await show(target(onScreen), vi.fn(), withHome);
		navigate('https://www.youtube.com/');
		navigate('https://www.youtube.com/watch?v=1');
		await flush();
		expect(homeButton().show).toHaveBeenCalledExactlyOnceWith(true);
		expect(calls()).toEqual([['webview_place', { label: 'view-1', bounds: notched(onScreen) }]]);
	});

	it('shows after a link within a single-page site too', async () => {
		open(target(onScreen), vi.fn(), withHome);
		navigate('https://www.youtube.com/');
		navigate('https://www.youtube.com/#comments');
		await flush();
		expect(homeButton().show).toHaveBeenCalledExactlyOnceWith(true);
	});

	it('stays hidden where the page redirects to as it loads, and on the blank page before', async () => {
		open(target(onScreen), vi.fn(), withHome);
		navigate('about:blank');
		navigate('https://m.youtube.com/');
		await flush();
		navigate('https://www.youtube.com/');
		await flush();
		expect(homeButton().show).not.toHaveBeenCalled();
		expect(calls()).toEqual([]);
	});

	it('compares the page as the host writes it', async () => {
		const close = openWebview(
			target(onScreen) as unknown as HTMLElement,
			'https://Example.com',
			withHome,
			undefined,
			vi.fn(),
			loading
		);
		opened.push(close);
		navigate('https://www.example.com/');
		navigate('https://example.com/');
		await flush();
		expect(homeButton().show).not.toHaveBeenCalled();
	});

	it('hides when the view comes back to its page, and its hole closes', async () => {
		await show(target(onScreen), vi.fn(), withHome);
		navigate('https://www.youtube.com/');
		navigate('https://www.youtube.com/watch?v=1');
		await flush();
		navigate('https://www.youtube.com/');
		await flush();
		expect(homeButton().show).toHaveBeenLastCalledWith(false);
		expect(calls()).toEqual([
			['webview_place', { label: 'view-1', bounds: notched(onScreen) }],
			['webview_place', { label: 'view-1', bounds: square(onScreen) }]
		]);
	});

	it('takes the view back to its page when pressed, and hides at once', async () => {
		await show(target(onScreen), vi.fn(), withHome);
		navigate('https://www.youtube.com/');
		navigate('https://www.youtube.com/watch?v=1');
		await flush();
		homeButton().press();
		await flush();
		expect(homeButton().show).toHaveBeenLastCalledWith(false);
		expect(calls().slice(1)).toEqual([
			['webview_place', { label: 'view-1', bounds: square(onScreen) }],
			['webview_navigate', { label: 'view-1', url: 'https://www.youtube.com/' }]
		]);
	});

	it('takes where its page lands after being pressed as its page', async () => {
		open(target(onScreen), vi.fn(), withHome);
		navigate('https://www.youtube.com/');
		navigate('https://www.youtube.com/watch?v=1');
		homeButton().press();
		navigate('https://www.youtube.com/?signed-in');
		await flush();
		expect(homeButton().show).toHaveBeenCalledTimes(2);
		expect(homeButton().show).toHaveBeenLastCalledWith(false);
	});

	it('hides while the page shows full screen, and shows again after', async () => {
		await show(target(onScreen), vi.fn(), withHome);
		navigate('https://www.youtube.com/');
		navigate('https://www.youtube.com/watch?v=1');
		await flush();
		fullscreen(true);
		await flush();
		expect(homeButton().show).toHaveBeenLastCalledWith(false);
		expect(calls().at(-1)?.[1]).toMatchObject({ bounds: { holes: [] } });
		fullscreen(false);
		await flush();
		expect(homeButton().show).toHaveBeenLastCalledWith(true);
		expect(calls().at(-1)).toEqual([
			'webview_place',
			{ label: 'view-1', bounds: notched(onScreen) }
		]);
	});

	it('stays hidden in full screen when the view leaves its page', async () => {
		await show(target(onScreen), vi.fn(), withHome);
		navigate('https://www.youtube.com/');
		fullscreen(true);
		navigate('https://www.youtube.com/watch?v=1');
		await flush();
		expect(homeButton().show).not.toHaveBeenCalledWith(true);
		expect(calls().at(-1)?.[1]).toMatchObject({ bounds: { holes: [] } });
	});

	it('only shows over a widget that asks for it', async () => {
		open(target(onScreen));
		navigate('https://www.youtube.com/');
		navigate('https://www.youtube.com/watch?v=1');
		await flush();
		expect(createHomeButton).not.toHaveBeenCalled();
		expect(calls()).toEqual([]);
	});

	it('is removed with the view, and ignores the pages reported after', async () => {
		const close = open(target(onScreen), vi.fn(), withHome);
		navigate('https://www.youtube.com/');
		close();
		navigate('https://www.youtube.com/watch?v=1');
		await flush();
		expect(homeButton().remove).toHaveBeenCalled();
		expect(homeButton().show).not.toHaveBeenCalled();
	});
});
