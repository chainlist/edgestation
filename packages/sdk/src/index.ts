import type { WidgetModule } from './types.js';

export type * from './types.js';

/**
 * Gives the types to the default export of a widget. Editors then know `el` and `api` without
 * annotations, also in plain JavaScript:
 *
 * ```js
 * export default defineWidget({
 * 	mount(el, api) {
 * 		el.textContent = api.manifest.name;
 * 	}
 * });
 * ```
 *
 * Returns `widget` with no change. The host only needs the object.
 */
export function defineWidget(widget: WidgetModule): WidgetModule {
	return widget;
}
