import { describe, expect, it, vi } from 'vitest';
import { setup } from './widget-api.stubs';

vi.mock('@tauri-apps/api/core', async () => (await import('./widget-api.mocks')).coreModule());
vi.mock('../webview/open-webview', async () =>
	(await import('./widget-api.mocks')).openWebviewModule()
);
vi.mock('@tauri-apps/plugin-http', async () => (await import('./widget-api.mocks')).httpModule());
vi.mock('#lib/scrollbars.ts', async () => (await import('./widget-api.mocks')).scrollbarsModule());

describe('every', () => {
	it('stops when its stop function is called', () => {
		const { api } = setup();
		const fn = vi.fn();
		const stop = api.every(1000, fn);
		vi.advanceTimersByTime(1000);
		stop();
		vi.advanceTimersByTime(5000);
		expect(fn).toHaveBeenCalledOnce();
	});

	it('stops on unmount', () => {
		const { api, scope } = setup();
		const fn = vi.fn();
		api.every(1000, fn);
		scope.dispose();
		vi.advanceTimersByTime(5000);
		expect(fn).not.toHaveBeenCalled();
	});

	it('waits while its page is paused', () => {
		const { api, scope, pause } = setup();
		const fn = vi.fn();
		api.every(1000, fn);
		pause.set(true);
		vi.advanceTimersByTime(5000);
		expect(fn).not.toHaveBeenCalled();
		scope.dispose();
	});
});

describe('after', () => {
	it('runs once after the delay', () => {
		const { api } = setup();
		const fn = vi.fn();
		api.after(300, fn);
		vi.advanceTimersByTime(299);
		expect(fn).not.toHaveBeenCalled();
		vi.advanceTimersByTime(5000);
		expect(fn).toHaveBeenCalledOnce();
	});

	it('does not run once cancelled', () => {
		const { api } = setup();
		const fn = vi.fn();
		api.after(300, fn)();
		vi.advanceTimersByTime(5000);
		expect(fn).not.toHaveBeenCalled();
	});

	it('waits while its page is paused', () => {
		const { api, pause } = setup();
		const fn = vi.fn();
		api.after(300, fn);
		pause.set(true);
		vi.advanceTimersByTime(5000);
		expect(fn).not.toHaveBeenCalled();
		pause.set(false);
		vi.advanceTimersByTime(0);
		expect(fn).toHaveBeenCalledOnce();
	});

	it('does not run after unmount', () => {
		const { api, scope } = setup();
		const fn = vi.fn();
		api.after(300, fn);
		scope.dispose();
		vi.advanceTimersByTime(5000);
		expect(fn).not.toHaveBeenCalled();
	});

	it('routes errors to fail', () => {
		const { api, fail } = setup();
		api.after(300, () => {
			throw new Error('boom');
		});
		vi.advanceTimersByTime(300);
		expect(fail).toHaveBeenCalledWith(new Error('boom'));
	});

	it('leaves nothing to clean up once it has run', () => {
		const { api, scope } = setup();
		api.after(300, vi.fn());
		vi.advanceTimersByTime(300);
		const clear = vi.spyOn(globalThis, 'clearTimeout');
		scope.dispose();
		expect(clear).not.toHaveBeenCalled();
	});
});
