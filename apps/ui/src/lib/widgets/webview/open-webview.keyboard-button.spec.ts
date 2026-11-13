import { describe, expect, it, vi } from 'vitest';
import {
	calls,
	flush,
	fullscreen,
	host,
	keyboardButton,
	onScreen,
	open,
	show,
	square,
	target
} from './open-webview.stubs';

vi.mock('@tauri-apps/api/core', async () => (await import('./open-webview.mocks')).coreModule());
vi.mock('./stand-in', async () => (await import('./open-webview.mocks')).standInModule());
vi.mock('./corner-buttons', async () =>
	(await import('./open-webview.mocks')).cornerButtonsModule()
);

describe('the keyboard button of a view', () => {
	it('floats over the bottom right corner of every view, in a hole the view leaves', () => {
		open(target(onScreen));
		expect(keyboardButton().show).toHaveBeenCalledExactlyOnceWith(true);
		expect(host.mock.calls[0][1]).toMatchObject({ bounds: square(onScreen) });
	});

	it('gives the view the keyboard when pressed', async () => {
		open(target(onScreen));
		await flush();
		keyboardButton().press();
		expect(calls()).toEqual([['focus_borrow', { view: 'view-1' }]]);
	});

	it('does nothing before the view is open', () => {
		open(target(onScreen));
		keyboardButton().press();
		expect(calls()).toEqual([]);
	});

	it('hides while the page shows full screen, and its hole closes', async () => {
		await show(target(onScreen));
		fullscreen(true);
		await flush();
		expect(keyboardButton().show).toHaveBeenLastCalledWith(false);
		const whole = { ...square(onScreen), holes: [] };
		expect(calls()).toEqual([['webview_place', { label: 'view-1', bounds: whole }]]);
	});

	it('comes back when the page leaves full screen', async () => {
		await show(target(onScreen));
		fullscreen(true);
		fullscreen(false);
		await flush();
		expect(keyboardButton().show).toHaveBeenLastCalledWith(true);
		expect(calls().at(-1)).toEqual([
			'webview_place',
			{ label: 'view-1', bounds: square(onScreen) }
		]);
	});

	it('goes with the view', () => {
		open(target(onScreen))();
		expect(keyboardButton().remove).toHaveBeenCalledOnce();
	});
});
