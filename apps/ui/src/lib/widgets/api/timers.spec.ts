import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { createPause, interval, setFrozen, timeout } from './timers';
import type { Cleanup } from '../types';

/** The timers are module-wide. Clear the timers of each test, so that none stays for the next. */
const clears: Cleanup[] = [];
const keep = (clear: Cleanup) => (clears.push(clear), clear);

beforeEach(() => vi.useFakeTimers());
afterEach(() => {
	clears.splice(0).forEach((clear) => clear());
	setFrozen(false);
	vi.useRealTimers();
});

describe('interval', () => {
	it('ticks on its interval until cleared', () => {
		const fn = vi.fn();
		const clear = keep(interval(createPause(false), 1000, fn));
		vi.advanceTimersByTime(2500);
		expect(fn).toHaveBeenCalledTimes(2);
		clear();
		vi.advanceTimersByTime(5000);
		expect(fn).toHaveBeenCalledTimes(2);
	});

	it('stops while frozen, then ticks right away and resumes', () => {
		const fn = vi.fn();
		keep(interval(createPause(false), 1000, fn));
		setFrozen(true);
		vi.advanceTimersByTime(5000);
		expect(fn).not.toHaveBeenCalled();
		setFrozen(false);
		expect(fn).toHaveBeenCalledOnce();
		vi.advanceTimersByTime(1000);
		expect(fn).toHaveBeenCalledTimes(2);
	});

	it('waits for the thaw when started while frozen', () => {
		setFrozen(true);
		const fn = vi.fn();
		keep(interval(createPause(false), 1000, fn));
		vi.advanceTimersByTime(5000);
		expect(fn).not.toHaveBeenCalled();
		setFrozen(false);
		expect(fn).toHaveBeenCalledOnce();
	});

	it('stays cleared across a freeze', () => {
		const fn = vi.fn();
		const clear = interval(createPause(false), 1000, fn);
		setFrozen(true);
		clear();
		setFrozen(false);
		vi.advanceTimersByTime(5000);
		expect(fn).not.toHaveBeenCalled();
	});

	it('stops while its page is paused, then ticks right away and resumes', () => {
		const pause = createPause(false);
		const fn = vi.fn();
		keep(interval(pause, 1000, fn));
		pause.set(true);
		vi.advanceTimersByTime(5000);
		expect(fn).not.toHaveBeenCalled();
		pause.set(false);
		expect(fn).toHaveBeenCalledOnce();
		vi.advanceTimersByTime(1000);
		expect(fn).toHaveBeenCalledTimes(2);
	});

	it('waits for both its page and the thaw', () => {
		const pause = createPause(true);
		setFrozen(true);
		const fn = vi.fn();
		keep(interval(pause, 1000, fn));
		pause.set(false);
		vi.advanceTimersByTime(5000);
		expect(fn).not.toHaveBeenCalled();
		setFrozen(false);
		expect(fn).toHaveBeenCalledOnce();
	});

	it('only resumes the timers of the page shown', () => {
		const [shown, hidden] = [createPause(true), createPause(true)];
		const [onShown, onHidden] = [vi.fn(), vi.fn()];
		keep(interval(shown, 1000, onShown));
		keep(interval(hidden, 1000, onHidden));
		shown.set(false);
		vi.advanceTimersByTime(1000);
		expect(onShown).toHaveBeenCalledTimes(2);
		expect(onHidden).not.toHaveBeenCalled();
	});
});

describe('timeout', () => {
	it('fires once after the delay', () => {
		const fn = vi.fn();
		keep(timeout(createPause(false), 300, fn));
		vi.advanceTimersByTime(299);
		expect(fn).not.toHaveBeenCalled();
		vi.advanceTimersByTime(5000);
		expect(fn).toHaveBeenCalledOnce();
	});

	it('waits while its page is paused, then fires if it came due', () => {
		const pause = createPause(false);
		const fn = vi.fn();
		keep(timeout(pause, 300, fn));
		pause.set(true);
		vi.advanceTimersByTime(5000);
		expect(fn).not.toHaveBeenCalled();
		pause.set(false);
		vi.advanceTimersByTime(0);
		expect(fn).toHaveBeenCalledOnce();
	});

	it('keeps its due time across a shorter pause', () => {
		const pause = createPause(false);
		const fn = vi.fn();
		keep(timeout(pause, 1000, fn));
		vi.advanceTimersByTime(200);
		pause.set(true);
		vi.advanceTimersByTime(300);
		pause.set(false);
		vi.advanceTimersByTime(499);
		expect(fn).not.toHaveBeenCalled();
		vi.advanceTimersByTime(1);
		expect(fn).toHaveBeenCalledOnce();
	});

	it('still fires while frozen', () => {
		const fn = vi.fn();
		keep(timeout(createPause(false), 300, fn));
		setFrozen(true);
		vi.advanceTimersByTime(300);
		expect(fn).toHaveBeenCalledOnce();
	});

	it('stays cancelled across a pause', () => {
		const pause = createPause(false);
		const fn = vi.fn();
		const clear = timeout(pause, 300, fn);
		pause.set(true);
		clear();
		pause.set(false);
		vi.advanceTimersByTime(5000);
		expect(fn).not.toHaveBeenCalled();
	});
});
