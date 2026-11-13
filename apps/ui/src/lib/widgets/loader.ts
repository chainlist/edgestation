import { getVersion } from '@tauri-apps/api/app';
import type { WidgetManifest, WidgetModule } from './types';
import { widgetUrl } from './url';
import { versionProblem } from './version';

export interface LoadedWidget {
	manifest: WidgetManifest;
	module: WidgetModule;
}

const cache = new Map<string, Promise<LoadedWidget>>();

/**
 * Loads a widget one time per window, also when it has many placements. A failed load does not
 * stay in the cache, so "Retry" on the error card fetches the files again.
 */
export function loadWidget(id: string): Promise<LoadedWidget> {
	let loading = cache.get(id);
	if (!loading) {
		loading = load(id);
		cache.set(id, loading);
		loading.catch(() => cache.delete(id));
	}
	return loading;
}

export const forgetWidget = (id: string) => void cache.delete(id);

async function load(id: string): Promise<LoadedWidget> {
	const response = await fetch(widgetUrl(id, 'manifest.json'), { cache: 'no-store' });
	if (!response.ok) throw new Error(`Widget "${id}" is not installed`);
	const manifest: WidgetManifest = { ...(await response.json()), id };
	if (manifest.minAppVersion !== undefined) {
		const problem = versionProblem(await getVersion(), String(manifest.minAppVersion));
		if (problem) throw new Error(`Widget "${id}" can't load: ${problem}`);
	}

	// The browser keeps failed module imports by URL. A new query lets a retry import agian.
	const url = `${widgetUrl(id, manifest.main ?? 'main.js')}?t=${Date.now()}`;
	const { default: module } = await import(/* @vite-ignore */ url);
	if (typeof module?.mount !== 'function') {
		throw new Error(`Widget "${id}" must default-export { mount(el, api) }`);
	}
	return { manifest, module };
}
