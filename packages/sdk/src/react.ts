import { createElement, type ComponentType } from 'react';
import { createRoot } from 'react-dom/client';
import { report } from './report-error.js';
import type { WidgetModule, WidgetProps } from './types.js';

/**
 * Makes a widget from a React component: `export default reactWidget(Clock)`. The adapter
 * renders the component in a React root on the root of the widget, with the `api` as its prop.
 * It unmounts the React root when the widget unmounts.
 *
 * With React 19, the widget shows its error card when the component throws an error in render
 * and none of your error boundaries catches it.
 */
export function reactWidget(component: ComponentType<WidgetProps>): WidgetModule {
	return {
		mount(el, api) {
			const root = createRoot(el, { onUncaughtError: (error) => report(api, error) });
			root.render(createElement(component, { api }));
			return () => root.unmount();
		}
	};
}
