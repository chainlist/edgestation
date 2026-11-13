import { describe, expect, it, vi } from 'vitest';
import { flush } from '#lib/test-helpers.ts';
import { trackPageLoads } from './page-loads';

const setup = () => {
	const loading = vi.fn((task: Promise<void>) => task);
	const firstLoaded = vi.fn();
	const loads = trackPageLoads(loading, firstLoaded);
	/** Tells if the task of call `n` to `loading` has settled. */
	const settled = (n: number) => {
		const done = vi.fn();
		void loading.mock.calls[n][0].then(done);
		return flush().then(() => done.mock.calls.length > 0);
	};
	return { loading, firstLoaded, loads, settled };
};

describe('trackPageLoads', () => {
	it('starts the first load at once, before the host reports it', async () => {
		const { loading, settled } = setup();
		expect(loading).toHaveBeenCalledOnce();
		expect(await settled(0)).toBe(false);
	});

	it('ends the first load with the end of the first reported load', async () => {
		const { firstLoaded, loads, settled } = setup();
		loads.report({ id: 7, done: false });
		expect(await settled(1)).toBe(false);
		loads.report({ id: 7, done: true });
		expect(await settled(0)).toBe(true);
		expect(await settled(1)).toBe(true);
		expect(firstLoaded).toHaveBeenCalledOnce();
	});

	it('shows each later load from its start to its end', async () => {
		const { loads, settled } = setup();
		loads.report({ id: 1, done: false });
		loads.report({ id: 1, done: true });
		loads.report({ id: 2, done: false });
		expect(await settled(2)).toBe(false);
		loads.report({ id: 2, done: true });
		expect(await settled(2)).toBe(true);
	});

	it('counts a redirect as the same load', () => {
		const { loading, loads } = setup();
		loads.report({ id: 7, done: false });
		loads.report({ id: 7, done: false });
		expect(loading).toHaveBeenCalledTimes(2);
	});

	it('tells when the first load ends, and only then', () => {
		const { firstLoaded, loads } = setup();
		loads.report({ id: 1, done: false });
		loads.report({ id: 2, done: false });
		expect(firstLoaded).not.toHaveBeenCalled();
		loads.report({ id: 1, done: true });
		loads.report({ id: 2, done: true });
		expect(firstLoaded).toHaveBeenCalledOnce();
	});

	it('ignores the end of a load it did not see start, like the blank first page', () => {
		const { firstLoaded, loads } = setup();
		loads.report({ id: 1, done: true });
		expect(firstLoaded).not.toHaveBeenCalled();
	});

	it('ends the loads still open when the view closes', async () => {
		const { loads, settled } = setup();
		loads.report({ id: 1, done: false });
		loads.end();
		expect(await settled(0)).toBe(true);
		expect(await settled(1)).toBe(true);
	});
});
