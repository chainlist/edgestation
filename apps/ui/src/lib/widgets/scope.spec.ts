import { describe, expect, it, vi } from 'vitest';
import { createScope } from './scope';

describe('guard', () => {
	it('routes sync throws to fail', () => {
		const fail = vi.fn();
		createScope(fail).guard(() => {
			throw new Error('boom');
		})();
		expect(fail).toHaveBeenCalledWith(new Error('boom'));
	});

	it('routes rejected promises to fail', async () => {
		const fail = vi.fn();
		createScope(fail).guard(() => Promise.reject(new Error('later')))();
		await vi.waitFor(() => expect(fail).toHaveBeenCalledWith(new Error('later')));
	});

	it('ignores a promise that rejects once disposed', async () => {
		const fail = vi.fn();
		const scope = createScope(fail);
		let reject!: (error: unknown) => void;
		const late = new Promise((_, r) => (reject = r));
		scope.guard(() => late)();
		scope.dispose();
		reject(new Error('cancelled'));
		await late.catch(() => {});
		expect(fail).not.toHaveBeenCalled();
	});

	it('does nothing once disposed', () => {
		const fn = vi.fn();
		const scope = createScope(vi.fn());
		const guarded = scope.guard(fn);
		scope.dispose();
		guarded();
		expect(fn).not.toHaveBeenCalled();
	});
});

describe('dispose', () => {
	it('runs cleanups in reverse order, even if one throws', () => {
		const order: string[] = [];
		const scope = createScope(vi.fn());
		scope.add(() => order.push('first'));
		scope.add(() => {
			throw new Error('bad cleanup');
		});
		scope.add(() => order.push('last'));
		vi.spyOn(console, 'error').mockImplementation(() => {});
		scope.dispose();
		expect(order).toEqual(['last', 'first']);
	});

	it('skips a cleanup that was unregistered', () => {
		const scope = createScope(vi.fn());
		const cleanup = vi.fn();
		const unregister = scope.add(cleanup);
		unregister();
		scope.dispose();
		expect(cleanup).not.toHaveBeenCalled();
	});

	it('runs a cleanup added after disposal right away', () => {
		const scope = createScope(vi.fn());
		scope.dispose();
		const cleanup = vi.fn();
		scope.add(cleanup);
		expect(cleanup).toHaveBeenCalledOnce();
	});
});
