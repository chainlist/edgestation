import { createApp, type Component } from 'vue';
import { report } from './report-error.js';
import type { WidgetModule } from './types.js';

/**
 * Makes a widget from a Vue 3 component: `export default vueWidget(Clock)`. The adapter mounts
 * the component as a Vue app on the root of the widget, and unmounts it with the widget. The
 * component gets the `api` as its prop. Declare it with `defineProps<WidgetProps>()`.
 *
 * The error card of the widget shows the errors that Vue catches. These are errors in render,
 * lifecycle hooks, watchers and template event handlers.
 */
export function vueWidget(component: Component): WidgetModule {
	return {
		mount(el, api) {
			const app = createApp(component, { api });
			app.config.errorHandler = (error) => report(api, error);
			app.mount(el);
			return () => app.unmount();
		}
	};
}
