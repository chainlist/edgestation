declare module '*.svelte' {
	import type { Component } from 'svelte';
	import type { WidgetProps } from '../src/index.js';

	const component: Component<WidgetProps>;
	export default component;
}
