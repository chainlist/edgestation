import type { WidgetApi } from '../src/index.js';

/** The part of the api that the adapters use. `errors` gets what would show the error card. */
export function fakeApi() {
	const errors: unknown[] = [];
	const guard =
		<A extends unknown[]>(fn: (...args: A) => unknown) =>
		(...args: A) => {
			try {
				const result = fn(...args);
				if (result instanceof Promise) result.catch((error) => errors.push(error));
			} catch (error) {
				errors.push(error);
			}
		};
	const api = { instanceId: 'placement-1', guard } as Partial<WidgetApi> as WidgetApi;
	return { api, errors };
}

/** A widget root as the host makes it: an element in a shadow root. */
export function widgetRoot(): HTMLElement {
	const host = document.createElement('div');
	document.body.append(host);
	const el = document.createElement('div');
	host.attachShadow({ mode: 'open' }).append(el);
	return el;
}
