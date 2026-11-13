import type { WidgetApi } from './api.js';
import type { Cleanup } from './cleanup.js';

/** The default export of a widget's entry module. */
export interface WidgetModule {
	/** Renders the widget into `el`. It can return a cleanup function, or a promise of one. */
	mount(el: HTMLElement, api: WidgetApi): void | Cleanup | Promise<void | Cleanup>;
}

/** The props of the root component of a widget in the Svelte, React and Vue adapters. */
export interface WidgetProps {
	api: WidgetApi;
}
