import { describe, expect, it, vi } from 'vitest';
import { flush } from '../../test-helpers';
import { createScope } from '../scope';
import { createLoading } from './loading';

const setup = () => {
	const show = vi.fn();
	const scope = createScope(vi.fn());
	return { show, scope, loading: createLoading(scope, show) };
};

describe('loading', () => {
	it('shows the dots until the first load ends', async () => {
		const { show, loading } = setup();
		const task = Promise.withResolvers<void>();
		loading(task.promise);
		expect(show).toHaveBeenLastCalledWith('dots');
		task.resolve();
		await flush();
		expect(show).toHaveBeenLastCalledWith(undefined);
	});

	it('shows the bar for the loads after the first', async () => {
		const { show, loading } = setup();
		await loading(Promise.resolve());
		await flush();
		loading(new Promise(() => {}));
		expect(show).toHaveBeenLastCalledWith('bar');
	});

	it('shows one indicator until the last of overlapping loads ends', async () => {
		const { show, loading } = setup();
		const first = Promise.withResolvers<void>();
		const second = Promise.withResolvers<void>();
		loading(first.promise);
		loading(second.promise);
		first.resolve();
		await flush();
		expect(show.mock.calls).toEqual([['dots']]);
		second.resolve();
		await flush();
		expect(show.mock.calls).toEqual([['dots'], [undefined]]);
	});

	it('returns the task, and ends the first load when it fails', async () => {
		const { show, loading } = setup();
		const task = Promise.reject(new Error('offline'));
		expect(loading(task)).toBe(task);
		await expect(task).rejects.toThrow('offline');
		await flush();
		loading(new Promise(() => {}));
		expect(show.mock.calls).toEqual([['dots'], [undefined], ['bar']]);
	});

	it('accepts a value that is not a promise', async () => {
		const { show, loading } = setup();
		loading(42 as unknown as Promise<number>);
		await flush();
		expect(show).toHaveBeenLastCalledWith(undefined);
	});

	it('shows nothing on unmount, and ignores loads after it', async () => {
		const { show, scope, loading } = setup();
		const task = Promise.withResolvers<void>();
		loading(task.promise);
		scope.dispose();
		expect(show).toHaveBeenLastCalledWith(undefined);
		loading(new Promise(() => {}));
		task.resolve();
		await flush();
		expect(show.mock.calls).toEqual([['dots'], [undefined]]);
	});
});
