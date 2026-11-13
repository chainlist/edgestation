import { mount, unmount, type Component } from 'svelte';
import type { WidgetModule, WidgetProps } from './types.js';

/**
 * Makes a widget from a Svelte 5 component: `export default svelteWidget(Clock)`. The adapter
 * mounts the component in the root of the widget with the `api` as its prop, and unmounts it
 * with the widget.
 *
 * The `<style>` blocks of the component get into the shadow root of the widget only if you
 * compile it with injected CSS. For this, use `svelte({ emitCss: false })` in the Vite config.
 * An error that the component throws when Svelte draws it does not show the error card. An
 * error in a callback that `api.guard`, `api.every` or `api.after` wraps shows it.
 */
export function svelteWidget(component: Component<WidgetProps>): WidgetModule {
	return {
		mount(el, api) {
			const app = mount(component, { target: el, props: { api } });
			return () => {
				void unmount(app);
			};
		}
	};
}
