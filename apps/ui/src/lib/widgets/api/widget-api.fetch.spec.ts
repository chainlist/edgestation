import { describe, expect, it, vi } from 'vitest';
import { fetch as httpFetch } from '@tauri-apps/plugin-http';
import { setup } from './widget-api.stubs';

vi.mock('@tauri-apps/api/core', async () => (await import('./widget-api.mocks')).coreModule());
vi.mock('../webview/open-webview', async () =>
	(await import('./widget-api.mocks')).openWebviewModule()
);
vi.mock('@tauri-apps/plugin-http', async () => (await import('./widget-api.mocks')).httpModule());
vi.mock('#lib/scrollbars.ts', async () => (await import('./widget-api.mocks')).scrollbarsModule());

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
