import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { createWidgetApi } from './widget-api';
import { shared } from './audio';
import { createScope } from '../scope';
import { createPause } from './timers';
import type { WidgetPermission } from '../types';
import { overlayScrollbars } from '../../scrollbars';
import { openWebview } from '../webview/open-webview';
import { fetch as httpFetch } from '@tauri-apps/plugin-http';
import { invoke } from '@tauri-apps/api/core';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(async () => []) }));
vi.mock('../webview/open-webview', () => ({ openWebview: vi.fn(() => () => {}) }));
vi.mock('@tauri-apps/plugin-http', () => ({ fetch: vi.fn() }));
vi.mock('../../scrollbars', () => ({ overlayScrollbars: vi.fn() }));

const ALL: WidgetPermission[] = ['system', 'media', 'apps', 'audio', 'fetch', 'webview'];

/** `permissions` is the list in the manifest. */
const setup = ({
	permissions = ALL,
	profile
}: { permissions?: WidgetPermission[]; profile?: string } = {}) => {
	const fail = vi.fn();
	const scope = createScope(fail);
	const pause = createPause(false);
	const api = createWidgetApi(
		'instance',
		{ id: 'test', name: 'Test', permissions },
		{},
		profile,
		scope,
		pause,
		vi.fn()
	);
	return { api, scope, pause, fail };
};

beforeEach(() => {
	vi.useFakeTimers();
	vi.mocked(openWebview).mockClear();
});
afterEach(() => vi.useRealTimers());

describe('webview', () => {
	const el = {} as HTMLElement;

	it('opens in the placement profile', () => {
		const { api } = setup({ profile: 'a1b2c3d4' });
		api.webview(el, 'https://example.com/', { mobile: true });
		expect(openWebview).toHaveBeenCalledWith(
			el,
			'https://example.com/',
			{ mobile: true },
			'a1b2c3d4',
			expect.any(Function),
			expect.any(Function)
		);
	});

	it('refuses a widget without the webview permission', () => {
		const { api, fail } = setup({ permissions: [] });
		expect(() => api.webview(el, 'https://example.com/')).toThrow('It tried to open a web page');
		expect(openWebview).not.toHaveBeenCalled();
		expect(fail).toHaveBeenCalledOnce();
	});
});

describe('permissions', () => {
	it('refuses a part that the manifest does not ask for', () => {
		const { api, fail } = setup({ permissions: ['media'] });
		expect(() => api.fetch('https://example.com/')).toThrow();
		expect(httpFetch).not.toHaveBeenCalled();
		const [error] = fail.mock.calls[0] as [Error];
		expect(error.message).toBe(
			'It tried to connect to the internet without asking. It needs an update.'
		);
		expect((error.cause as Error).message).toBe(
			'Add "fetch" to the permissions of test/manifest.json'
		);
	});

	it('gives topPrograms with system', () => {
		void setup({ permissions: ['system'] }).api.topPrograms(3);
		expect(invoke).toHaveBeenLastCalledWith('top_programs', { count: 3 });
		expect(() => setup({ permissions: ['media'] }).api.topPrograms(3)).toThrow(
			'how busy your PC is'
		);
	});

	it('refuses each function of a part, and keeps its keys', () => {
		const { api } = setup({ permissions: [] });
		expect(Object.keys(api.audio)).toEqual(Object.keys(setup().api.audio));
		expect(() => api.audio.icon('a.exe')).toThrow('your sound settings');
		expect(() => api.apps.launch('a')).toThrow('your apps');
	});

	it('does not report a refusal after unmount', () => {
		const { api, scope, fail } = setup({ permissions: [] });
		scope.dispose();
		expect(() => api.system()).toThrow();
		expect(fail).not.toHaveBeenCalled();
	});
});

describe('scrollbars', () => {
	const el = {} as HTMLElement;

	it('removes them on unmount', () => {
		const remove = vi.fn();
		vi.mocked(overlayScrollbars).mockReturnValueOnce(remove);
		const { api, scope } = setup();
		api.scrollbars(el);
		expect(overlayScrollbars).toHaveBeenCalledWith(el);
		scope.dispose();
		expect(remove).toHaveBeenCalledOnce();
	});

	it('removes them only once when removed before unmount', () => {
		const remove = vi.fn();
		vi.mocked(overlayScrollbars).mockReturnValueOnce(remove);
		const { api, scope } = setup();
		api.scrollbars(el)();
		scope.dispose();
		expect(remove).toHaveBeenCalledOnce();
	});
});

describe('apps.icon', () => {
	it('puts the size and the whole id in the path', () => {
		const { api } = setup();
		const id = '{6D809377-6AF0-444B-8957-A3773F02200E}\\Tailscale\\tailscale-ipn.exe';
		const url = new URL(api.apps.icon(id, 96));
		const [, size, encoded] = url.pathname.split('/');
		expect(size).toBe('96');
		expect(decodeURIComponent(encoded)).toBe(id);
	});

	it('keeps the size within what the host draws', () => {
		const { api } = setup();
		expect(api.apps.icon('a', 1000)).toMatch(/\/256\/a$/);
		expect(api.apps.icon('a', 4)).toMatch(/\/16\/a$/);
		expect(api.apps.icon('a')).toMatch(/\/64\/a$/);
	});
});

describe('shared', () => {
	it('shares one read between calls less than its time apart', async () => {
		const read = vi.fn(async () => read.mock.calls.length);
		const get = shared(read, 30);
		expect(await Promise.all([get(), get()])).toEqual([1, 1]);
		vi.advanceTimersByTime(30);
		expect(await get()).toBe(2);
	});

	it('reads again after forget, and after a failed read', async () => {
		const read = vi.fn().mockRejectedValueOnce(new Error('gone')).mockResolvedValue('ok');
		const get = shared(read, 1000);
		await expect(get()).rejects.toThrow('gone');
		expect(await get()).toBe('ok');
		get.forget();
		await get();
		expect(read).toHaveBeenCalledTimes(3);
	});
});

describe('audio.icon', () => {
	it('asks for the file route with the size and the whole path', () => {
		const { api } = setup();
		const path = 'C:\\Program Files\\Zen Twilight\\zen.exe';
		const url = new URL(api.audio.icon(path, 1000));
		const [, route, size, encoded] = url.pathname.split('/');
		expect([route, size]).toEqual(['file', '256']);
		expect(decodeURIComponent(encoded)).toBe(path);
	});
});

describe('fetch', () => {
	const url = 'https://example.com/';
	const sentSignal = () => vi.mocked(httpFetch).mock.lastCall![1]!.signal!;
	const unanswered = () => vi.mocked(httpFetch).mockReturnValueOnce(new Promise(() => {}));

	it('sends the request through the http plugin', async () => {
		const response = new Response('ok');
		vi.mocked(httpFetch).mockResolvedValueOnce(response);
		const { api } = setup();
		await expect(api.fetch(url, { method: 'POST', body: 'x' })).resolves.toBe(response);
		expect(httpFetch).toHaveBeenLastCalledWith(
			url,
			expect.objectContaining({ method: 'POST', body: 'x' })
		);
	});

	it('cancels an unanswered request on unmount', () => {
		unanswered();
		const { api, scope } = setup();
		void api.fetch(url);
		scope.dispose();
		expect(sentSignal().aborted).toBe(true);
	});

	it('forgets a request once answered', async () => {
		vi.mocked(httpFetch).mockResolvedValueOnce(new Response());
		const { api, scope } = setup();
		await api.fetch(url);
		scope.dispose();
		expect(sentSignal().aborted).toBe(false);
	});

	it("still follows the widget's own signal", () => {
		unanswered();
		const { api } = setup();
		const own = new AbortController();
		void api.fetch(url, { signal: own.signal });
		own.abort();
		expect(sentSignal().aborted).toBe(true);
	});

	it('follows the signal of a Request', () => {
		unanswered();
		const { api } = setup();
		const own = new AbortController();
		void api.fetch(new Request(url, { signal: own.signal }));
		own.abort();
		expect(sentSignal().aborted).toBe(true);
	});
});

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
