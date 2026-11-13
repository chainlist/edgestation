import type { LoadingLook } from './api/loading';
import type { Pause } from './api/timers';
import { createWidgetApi } from './api/widget-api';
import { loadWidget } from './loader';
import { createScope } from './scope';
import { resolveSettings } from './settings';
import { createShadowRoot } from './shadow';
import type { Cleanup, WidgetSettingValues } from './types';

/**
 * Loads widget `widgetId` and mounts it on `host` with the saved `settings` of the placement
 * and the browser `profile`. The widget timers wait while `pause` of its page is set. All
 * errors go to `fail`, sync and async. `showLoading` gets the look of `api.loading`. Returns the
 * unmount function. It releases all that the widget registered.
 */
export function mountWidget(
	host: HTMLElement,
	widgetId: string,
	instanceId: string,
	settings: WidgetSettingValues | undefined,
	profile: string | undefined,
	pause: Pause,
	fail: (error: unknown) => void,
	showLoading: (look: LoadingLook) => void
): Cleanup {
	const scope = createScope(fail);

	loadWidget(widgetId)
		.then(({ manifest, module }) => {
			if (scope.disposed) return;
			const el = createShadowRoot(host, manifest);
			const values = resolveSettings(manifest.settings, settings);
			const api = createWidgetApi(instanceId, manifest, values, profile, scope, pause, showLoading);
			return module.mount(el, api);
		})
		.then((cleanup) => {
			if (typeof cleanup === 'function') scope.add(cleanup);
		})
		.catch((error) => {
			if (!scope.disposed) fail(error);
		});

	return () => {
		scope.dispose();
		host.shadowRoot?.replaceChildren();
	};
}
