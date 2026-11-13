EdgeStation has no framework-specific API: it calls the widget module's `mount(el, api)` and runs the cleanup it returns. The SDK's adapters turn a component into such a module. Each one mounts the component in `el` with the `api` as its only prop, and unmounts it with the widget.

| Framework | Adapter                                       | Plugin to add next to `widget()` |
| --------- | --------------------------------------------- | -------------------------------- |
| Svelte 5  | `svelteWidget` from `@edgestation/sdk/svelte` | `svelte({ emitCss: false })`     |
| React 19  | `reactWidget` from `@edgestation/sdk/react`   | `react()`                        |
| Vue 3     | `vueWidget` from `@edgestation/sdk/vue`       | `vue()`                          |

The component's props are typed `WidgetProps`: `{ api: WidgetApi }`. [Installing the SDK](/docs/dependencies#what-to-install) lists the packages each one needs.

## Svelte 5

```bash
npm install -D @edgestation/sdk vite svelte @sveltejs/vite-plugin-svelte
```

```ts vite.config.ts
import { widget } from '@edgestation/sdk/vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

export default defineConfig({ plugins: [svelte({ emitCss: false }), widget()] });
```

```ts src/main.ts
import { svelteWidget } from '@edgestation/sdk/svelte';
import Cpu from './Cpu.svelte';

export default svelteWidget(Cpu);
```

```svelte src/Cpu.svelte
<script lang="ts">
	import type { WidgetProps } from '@edgestation/sdk';

	let props: WidgetProps = $props();
	// svelte-ignore state_referenced_locally
	const { api } = props;
	let load = $state(0);
	api.every(1000, async () => (load = (await api.system()).cpu.load));
</script>

<p>{Math.round(load)} %</p>

<style>
	p {
		color: var(--primary);
	}
</style>
```

- **Styles.** `emitCss: false` keeps each component's `<style>` block in its JavaScript, and Svelte adds it to the widget's shadow root when the component mounts. There is no CSS file, so leave `styles` out of the manifest.
- **Reading `api`.** Read `api` from the props once, at the top of the script. Svelte warns that this only captures the prop's first value (`state_referenced_locally`): that is what you want, since a setting change remounts the widget rather than changing the prop. Taking `api` out of `props` in one statement, as above, keeps the warning to that line, which the `svelte-ignore` comment silences. Destructured straight from `$props()`, `api` would raise it at every use.
- **Errors.** Svelte redraws after the callback that changed its state has returned, so errors thrown while drawing don't show the error card. Errors in callbacks wrapped with `api.guard`, `api.every` or `api.after` do.
- **Scrollbars.** An attachment gives an element the app's scrollbars and removes them with it: `<ul {@attach (ul) => api.scrollbars(ul)}>`.

## React 19

```bash
npm install -D @edgestation/sdk vite react react-dom @vitejs/plugin-react
```

```ts vite.config.ts
import { widget } from '@edgestation/sdk/vite';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

export default defineConfig({ plugins: [react(), widget()] });
```

```tsx src/main.tsx
import { reactWidget } from '@edgestation/sdk/react';
import type { WidgetProps } from '@edgestation/sdk';
import { useEffect, useState } from 'react';
import './styles.css';

function Cpu({ api }: WidgetProps) {
	const [load, setLoad] = useState(0);
	useEffect(() => api.every(1000, async () => setLoad((await api.system()).cpu.load)), []);
	return <p>{Math.round(load)} %</p>;
}

export default reactWidget(Cpu);
```

- **Effects.** `api.every`, `api.after`, `api.webview` and `api.scrollbars` return their cleanup, so an effect can return their result directly, as above.
- **Styles.** CSS you import goes to `dist/main.css`. Add `"styles": "dist/main.css"` to the manifest.
- **Errors.** An error thrown while rendering, that no error boundary of yours catches, shows the widget's error card.

## Vue 3

```bash
npm install -D @edgestation/sdk vite vue @vitejs/plugin-vue
```

```ts vite.config.ts
import { widget } from '@edgestation/sdk/vite';
import vue from '@vitejs/plugin-vue';
import { defineConfig } from 'vite';

export default defineConfig({ plugins: [vue(), widget()] });
```

```ts src/main.ts
import { vueWidget } from '@edgestation/sdk/vue';
import Cpu from './Cpu.vue';

export default vueWidget(Cpu);
```

```vue src/Cpu.vue
<script setup lang="ts">
import { ref } from 'vue';
import type { WidgetProps } from '@edgestation/sdk';

const { api } = defineProps<WidgetProps>();
const load = ref(0);
api.every(1000, async () => (load.value = (await api.system()).cpu.load));
</script>

<template>
	<p>{{ Math.round(load) }} %</p>
</template>

<style>
p {
	color: var(--primary);
}
</style>
```

- **Styles.** `<style>` blocks go to `dist/main.css`. Add `"styles": "dist/main.css"` to the manifest. They don't need `scoped`: the widget's shadow root already keeps them in.
- **Errors.** Errors Vue catches (in rendering, lifecycle hooks, watchers and template event handlers) show the widget's error card.
- **TypeScript.** For `tsc` to understand `.vue` files, check them with `vue-tsc`.

## Another framework

Any framework that can render into an element works without an adapter: write the module yourself. `mount` renders into `el` and returns what unmounts it, and errors you catch can go to the error card through `api.guard`:

```js src/main.js
import { defineWidget } from '@edgestation/sdk';

export default defineWidget({
	mount(el, api) {
		const app = renderMyApp(el, { api }); // your framework's mount
		return () => app.destroy(); // and its unmount
	}
});
```
