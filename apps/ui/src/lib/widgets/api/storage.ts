import { widgetDataStore as store } from '#lib/tauri/store.ts';
import type { StorageApi, WidgetStorage } from '../types';

// Ids never contain '/', so one prefix cannot reach into the keys of another.
const scoped = (prefix: string): WidgetStorage =>
	Object.freeze({
		get: <T>(key: string) => store.get<T>(prefix + key),
		set: (key: string, value: unknown) => store.set(prefix + key, value),
		delete: (key: string) => store.delete(prefix + key)
	});

export const createStorageApi = (widgetId: string, instanceId: string): StorageApi =>
	Object.freeze({
		widget: scoped(`widget/${widgetId}/`),
		instance: scoped(`instance/${instanceId}/`)
	});
