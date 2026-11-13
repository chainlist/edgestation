import { beforeEach, describe, expect, it, vi } from 'vitest';
import { createStorageApi } from './storage';

// The tests use an in-memory map in place of the store file.
const data = vi.hoisted(() => new Map<string, unknown>());
vi.mock('#lib/tauri/store.ts', () => ({
	widgetDataStore: {
		get: async (key: string) => data.get(key),
		set: async (key: string, value: unknown) => void data.set(key, value),
		delete: async (key: string) => data.delete(key)
	}
}));

beforeEach(() => data.clear());

describe('storage', () => {
	it('reads back what was saved', async () => {
		const { instance } = createStorageApi('todo', 'a');
		await instance.set('tasks', [{ text: 'milk', done: false }]);
		expect(await instance.get('tasks')).toEqual([{ text: 'milk', done: false }]);
	});

	it('returns undefined for a key never saved', async () => {
		expect(await createStorageApi('todo', 'a').instance.get('missing')).toBeUndefined();
	});

	it('deletes a key and says whether it existed', async () => {
		const { instance } = createStorageApi('todo', 'a');
		await instance.set('note', 'x');
		expect(await instance.delete('note')).toBe(true);
		expect(await instance.get('note')).toBeUndefined();
		expect(await instance.delete('note')).toBe(false);
	});

	it('keeps placements apart in instance storage', async () => {
		await createStorageApi('todo', 'a').instance.set('note', 'first');
		await createStorageApi('todo', 'b').instance.set('note', 'second');
		expect(await createStorageApi('todo', 'a').instance.get('note')).toBe('first');
		expect(await createStorageApi('todo', 'b').instance.get('note')).toBe('second');
	});

	it('shares widget storage between placements of the same widget only', async () => {
		await createStorageApi('todo', 'a').widget.set('theme', 'dark');
		expect(await createStorageApi('todo', 'b').widget.get('theme')).toBe('dark');
		expect(await createStorageApi('clock', 'c').widget.get('theme')).toBeUndefined();
	});

	it('keeps the two scopes apart when a placement id equals a widget id', async () => {
		// The clock placment on the default page has the id 'clock'.
		const { widget, instance } = createStorageApi('clock', 'clock');
		await widget.set('format', '24h');
		await instance.set('format', '12h');
		expect(await widget.get('format')).toBe('24h');
		expect(await instance.get('format')).toBe('12h');
	});
});
