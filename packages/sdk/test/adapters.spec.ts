import { act, createElement } from 'react';
import { flushSync } from 'svelte';
import { describe, expect, it, onTestFinished, vi } from 'vitest';
import { defineComponent, h, type PropType } from 'vue';
import { defineWidget, type Cleanup, type WidgetApi, type WidgetProps } from '../src/index.js';
import { reactWidget } from '../src/react.js';
import { svelteWidget } from '../src/svelte.js';
import { vueWidget } from '../src/vue.js';
import { fakeApi, widgetRoot } from './fake-api.js';
import Hello from './Hello.svelte';

declare global {
	var IS_REACT_ACT_ENVIRONMENT: boolean;
}
globalThis.IS_REACT_ACT_ENVIRONMENT = true;

describe('defineWidget', () => {
	it('returns the widget unchanged', () => {
		const widget = { mount() {} };
		expect(defineWidget(widget)).toBe(widget);
	});
});

describe('svelteWidget', () => {
	it('mounts the component with the api, and unmounts it', async () => {
		const { api } = fakeApi();
		const el = widgetRoot();
		const cleanup = (await svelteWidget(Hello).mount(el, api)) as Cleanup;
		flushSync();
		expect(el.textContent).toBe('placement-1');
		cleanup();
		expect(el.textContent).toBe('');
	});
});

describe('reactWidget', () => {
	const Hello = ({ api }: WidgetProps) => createElement('p', null, api.instanceId);

	it('renders the component with the api, and unmounts it', async () => {
		const { api } = fakeApi();
		const el = widgetRoot();
		let cleanup: Cleanup | undefined;
		await act(async () => {
			cleanup = (await reactWidget(Hello).mount(el, api)) as Cleanup;
		});
		expect(el.textContent).toBe('placement-1');
		act(() => cleanup!());
		expect(el.textContent).toBe('');
	});

	it('shows an error thrown while rendering on the error card', async () => {
		// In `act`, React throws uncaught errors again and does not call `onUncaughtError`.
		globalThis.IS_REACT_ACT_ENVIRONMENT = false;
		onTestFinished(() => void (globalThis.IS_REACT_ACT_ENVIRONMENT = true));
		const { api, errors } = fakeApi();
		const Broken = () => {
			throw new Error('render failed');
		};
		await reactWidget(Broken).mount(widgetRoot(), api);
		await vi.waitFor(() => expect(errors).toEqual([new Error('render failed')]));
	});
});

describe('vueWidget', () => {
	const props = { api: { type: Object as PropType<WidgetApi>, required: true } } as const;
	const Hello = defineComponent({ props, setup: (p) => () => h('p', p.api.instanceId) });

	it('mounts the component with the api, and unmounts it', async () => {
		const { api } = fakeApi();
		const el = widgetRoot();
		const cleanup = (await vueWidget(Hello).mount(el, api)) as Cleanup;
		expect(el.textContent).toBe('placement-1');
		cleanup();
		expect(el.textContent).toBe('');
	});

	it('shows an error thrown while rendering on the error card', async () => {
		const { api, errors } = fakeApi();
		const Broken = defineComponent({
			props,
			setup: () => () => {
				throw new Error('render failed');
			}
		});
		await vueWidget(Broken).mount(widgetRoot(), api);
		expect(errors).toEqual([new Error('render failed')]);
	});
});
