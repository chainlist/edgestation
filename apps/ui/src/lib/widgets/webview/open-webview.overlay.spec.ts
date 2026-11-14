import { describe, expect, it, vi } from 'vitest';
import { flush } from '#lib/test-helpers.ts';
import { overlay } from './motion';
import {
	calls,
	element,
	keyboardHole,
	nextPage,
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

describe('a page element laid over the widgets', () => {
	/** The Paused badge, in the top right corner of the window. */
	const badge = () =>
		element(
			{ x: 500, y: 0, width: 130, height: 36 },
			{
				borderTopRightRadius: '0px',
				borderBottomRightRadius: '0px',
				borderBottomLeftRadius: '12px'
			}
		) as unknown as Element;

	it('makes the views it covers leave a hole for it, rounded like it', async () => {
		await show(target(onScreen));
		const stop = overlay(badge());
		await flush();
		stop();
		await flush();
		const hole = { x: 490, y: -10, width: 130, height: 36, radius: 12 };
		expect(calls()).toEqual([
			[
				'webview_place',
				{ label: 'view-1', bounds: { ...square(onScreen), holes: [keyboardHole(onScreen), hole] } }
			],
			['webview_place', { label: 'view-1', bounds: square(onScreen) }]
		]);
	});

	it('leaves the views it misses whole', async () => {
		open(target(nextPage));
		const stop = overlay(badge());
		await flush();
		stop();
		expect(calls()).toEqual([]);
	});
});
