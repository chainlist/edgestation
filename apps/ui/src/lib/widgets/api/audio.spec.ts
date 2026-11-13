import { invoke } from '@tauri-apps/api/core';
import { listen, type EventCallback } from '@tauri-apps/api/event';
import { beforeEach, describe, expect, it, vi, type Mock } from 'vitest';
import { flush } from '#lib/test-helpers.ts';
import { createAudioApi, stream } from './audio';
import { createPause } from './timers';

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn(async () => true) }));
vi.mock('@tauri-apps/api/event', () => ({ listen: vi.fn() }));

/** The handler of the latest `listen`, and the mock of the function that stops it. */
let send: EventCallback<unknown>;
let unlisten: Mock<() => void>;

beforeEach(() => {
	vi.mocked(invoke).mockClear();
	unlisten = vi.fn<() => void>();
	vi.mocked(listen).mockImplementation(async (_, handler) => {
		send = handler as EventCallback<unknown>;
		return unlisten;
	});
});

const message = (payload: unknown) => send({ event: 'x', id: 1, payload });

describe('stream', () => {
	it('turns on once listening, with the first listener, and off with the last', async () => {
		const each = vi.fn();
		const listenTo = stream<number>('ev', 'cmd', each);
		const a = vi.fn();
		const b = vi.fn();
		const stopA = listenTo(a);
		const stopB = listenTo(b);
		await flush();
		expect(listen).toHaveBeenCalledOnce();
		expect(invoke).toHaveBeenCalledExactlyOnceWith('cmd', { on: true });

		message(7);
		expect([each, a, b].map((fn) => fn.mock.lastCall)).toEqual([[7], [7], [7]]);

		stopA();
		expect(invoke).toHaveBeenCalledTimes(1);
		stopB();
		await flush();
		expect(invoke).toHaveBeenLastCalledWith('cmd', { on: false });
		expect(unlisten).toHaveBeenCalledOnce();
	});

	it('stays off when its listeners left before it listened', async () => {
		const stop = stream('ev', 'cmd')(vi.fn());
		stop();
		await flush();
		expect(invoke).toHaveBeenCalledExactlyOnceWith('cmd', { on: false });
	});
});

describe('onChange', () => {
	const guard =
		<A extends unknown[]>(fn: (...args: A) => unknown) =>
		(...args: A) =>
			void fn(...args);

	it('listens only while its page shows, and catches up when it shows again', async () => {
		const pause = createPause(false);
		const fn = vi.fn();
		createAudioApi(pause, guard, (stop) => stop).onChange(fn);
		await flush();
		expect(invoke).toHaveBeenLastCalledWith('audio_watch', { on: true });
		message(null);
		expect(fn).toHaveBeenCalledOnce();

		pause.set(true);
		await flush();
		expect(invoke).toHaveBeenLastCalledWith('audio_watch', { on: false });

		pause.set(false);
		expect(fn).toHaveBeenCalledTimes(2);
	});
});
