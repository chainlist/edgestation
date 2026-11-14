import { invoke } from '@tauri-apps/api/core';
import { fetch as httpFetch } from '@tauri-apps/plugin-http';
import { overlayScrollbars } from '#lib/scrollbars.ts';
import type { Scope } from '../scope';
import type {
	AppInfo,
	AppsApi,
	Cleanup,
	ProgramUsage,
	SystemInfo,
	WebviewOptions,
	WidgetApi,
	WidgetManifest,
	WidgetPermission,
	WidgetSettingValues
} from '../types';
import { PERMISSIONS } from '../permissions';
import { appIconUrl, widgetUrl } from '../url';
import { openWebview } from '../webview/open-webview';
import { createAudioApi } from './audio';
import { createLoading, type LoadingLook } from './loading';
import { createMediaApi } from './media';
import { createStorageApi } from './storage';
import { interval, timeout, type Pause } from './timers';

const apps: AppsApi = Object.freeze({
	list: () => invoke<AppInfo[]>('apps_list'),
	launch: (id: string) => invoke<void>('apps_launch', { id }),
	icon: (id: string, size = 64) => appIconUrl(id, size)
});

/**
 * `profile` is the browser profile id of the placement, `undefined` for Default. The card draws
 * what `showLoading` gets.
 */
export function createWidgetApi(
	instanceId: string,
	manifest: WidgetManifest,
	settings: WidgetSettingValues,
	profile: string | undefined,
	scope: Scope,
	pause: Pause,
	showLoading: (look: LoadingLook) => void
): WidgetApi {
	/** Registers `stop` to run on unmount. The returned function runs it now and unregisters it. */
	const track = (stop: Cleanup): Cleanup => {
		const forget = scope.add(stop);
		return () => {
			forget();
			stop();
		};
	};
	const loading = createLoading(scope, showLoading);
	// Throw the error again in a guard. It shows the error card if the widget is still there, also
	// when a plain event listener made the call.
	const report = scope.guard((error: unknown) => {
		throw error;
	});
	/**
	 * Returns `part` if the manifest lists `permission`. Else it returns a copy whose functions
	 * show the error card and throw.
	 */
	const gate = <T extends object>(permission: WidgetPermission, part: T): T => {
		if (manifest.permissions?.includes(permission)) return part;
		const denied = () => {
			// The card shows the message to the user. The cause tells the author what to fix, in the
			// console.
			const fix = new Error(
				`Add "${permission}" to the permissions of ${manifest.id}/manifest.json`
			);
			const error = new Error(
				`It tried to ${PERMISSIONS[permission].tried} without asking. It needs an update.`,
				{ cause: fix }
			);
			report(error);
			throw error;
		};
		if (typeof part === 'function') return denied as T;
		return Object.freeze(Object.fromEntries(Object.keys(part).map((key) => [key, denied]))) as T;
	};

	return Object.freeze({
		instanceId,
		manifest,
		settings: Object.freeze(settings),
		system: gate('system', () => invoke<SystemInfo>('system_info')),
		topPrograms: gate('system', (count: number) =>
			invoke<ProgramUsage[]>('system_top_programs', { count })
		),
		media: gate('media', createMediaApi(pause, scope.guard, track)),
		apps: gate('apps', apps),
		audio: gate('audio', createAudioApi(pause, scope.guard, track)),
		fetch: gate('fetch', (input: RequestInfo | URL, init: RequestInit = {}) => {
			// Unmount cancels the request until it has an answer. Then the cleanup goes away, so
			// polls do not collect cleanups.
			const unmount = new AbortController();
			const forget = scope.add(() => unmount.abort());
			const own = init.signal ?? (input instanceof Request ? input.signal : undefined);
			const signal = own ? AbortSignal.any([own, unmount.signal]) : unmount.signal;
			return httpFetch(input, { ...init, signal }).finally(forget);
		}),
		storage: createStorageApi(manifest.id, instanceId),
		asset: (path: string) => widgetUrl(manifest.id, path),
		guard: scope.guard,
		loading,
		every(ms: number, fn: () => unknown) {
			return track(interval(pause, ms, scope.guard(fn)));
		},
		after(ms: number, fn: () => unknown) {
			const run = scope.guard(fn);
			const cancel = track(
				timeout(pause, ms, () => {
					cancel(); // This only unregisters, because the timer has allready fired.
					run();
				})
			);
			return cancel;
		},
		webview: gate('webview', (target: HTMLElement, url: string, options: WebviewOptions = {}) =>
			track(openWebview(target, url, options, profile, report, loading))
		),
		scrollbars(viewport: HTMLElement) {
			return track(overlayScrollbars(viewport));
		},
		onDispose(fn: Cleanup) {
			scope.add(fn);
		}
	});
}
