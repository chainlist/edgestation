# @edgestation/sdk

Write widgets for [EdgeStation](https://github.com/chainlist/edgestation), in plain JavaScript, Svelte, React or Vue. EdgeStation is a widget dashboard for a secondary display: made first for the Corsair Xeneon Edge, it works on any other one. The docs call the display it runs on "the Edge", whichever it is.

A widget is a folder with a `manifest.json` and an ES module that EdgeStation loads at runtime. The app hands each widget its `api` when it mounts, so nothing from this package runs inside the app: it gives you

- the TypeScript types of the `api`, the manifest and the widget module,
- `defineWidget()`, to get those types in plain JavaScript too,
- adapters that mount a Svelte, React or Vue component as a widget,
- a Vite plugin that builds the one self-contained `dist/main.js` EdgeStation loads,
- a JSON Schema for `manifest.json`, for autocomplete and checks in your editor.

What the `api` does, the manifest fields, settings, styling and theming are documented in [Writing a widget](https://github.com/chainlist/edgestation/blob/main/widgets/README.md).

## Start a widget

```bash
npm install -D @edgestation/sdk vite
```

```
my-widget/
  manifest.json
  package.json      ("type": "module")
  vite.config.ts
  src/main.ts       (or main.js, main.tsx, main.jsx)
  dist/main.js      (built)
```

```json
{
	"$schema": "./node_modules/@edgestation/sdk/manifest.schema.json",
	"name": "My widget",
	"minAppVersion": "0.1.0",
	"main": "dist/main.js",
	"size": { "w": 2, "h": 1 },
	"permissions": ["system"]
}
```

`minAppVersion` is the oldest EdgeStation that can run the widget. The `0.1.0` above is only an example: set it to the version of this package you build with (they are released together), and older apps show an error card instead of a broken widget.

`permissions` lists the parts of the `api` the widget uses. The examples below call `api.system()`, so they need `system`.

### Plain JavaScript or TypeScript

```js
// vite.config.js
import { widget } from '@edgestation/sdk/vite';
import { defineConfig } from 'vite';

export default defineConfig({ plugins: [widget()] });
```

```js
// src/main.js
import { defineWidget } from '@edgestation/sdk';

export default defineWidget({
	mount(el, api) {
		api.every(1000, async () => {
			const { cpu } = await api.system();
			el.textContent = `${Math.round(cpu.load)} %`;
		});
	}
});
```

### Svelte 5

```bash
npm install -D svelte @sveltejs/vite-plugin-svelte
```

```ts
// vite.config.ts
import { widget } from '@edgestation/sdk/vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

export default defineConfig({ plugins: [svelte({ emitCss: false }), widget()] });
```

```ts
// src/main.ts
import { svelteWidget } from '@edgestation/sdk/svelte';
import Cpu from './Cpu.svelte';

export default svelteWidget(Cpu);
```

```svelte
<!-- src/Cpu.svelte -->
<script lang="ts">
	import type { WidgetProps } from '@edgestation/sdk';

	let props: WidgetProps = $props();
	// svelte-ignore state_referenced_locally
	const { api } = props;
	let load = $state(0);
	api.every(1000, async () => (load = (await api.system()).cpu.load));
</script>

<p>{Math.round(load)} %</p>
```

`emitCss: false` keeps each component's styles in its JavaScript, and Svelte adds them to the widget's shadow root when it mounts.

Read `api` from the props once, at the top of the script. Svelte warns that this only captures the prop's first value (`state_referenced_locally`): that is what you want, since a setting change remounts the widget rather than changing the prop. Taking `api` out of `props` in one statement keeps the warning to that line, which the `svelte-ignore` comment silences. Destructured straight from `$props()`, `api` would raise it at every use.

### React 19

```bash
npm install -D react react-dom @vitejs/plugin-react
```

```ts
// vite.config.ts
import { widget } from '@edgestation/sdk/vite';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

export default defineConfig({ plugins: [react(), widget()] });
```

```tsx
// src/main.tsx
import { reactWidget } from '@edgestation/sdk/react';
import type { WidgetProps } from '@edgestation/sdk';
import { useEffect, useState } from 'react';

function Cpu({ api }: WidgetProps) {
	const [load, setLoad] = useState(0);
	useEffect(() => api.every(1000, async () => setLoad((await api.system()).cpu.load)), []);
	return <p>{Math.round(load)} %</p>;
}

export default reactWidget(Cpu);
```

An error thrown while rendering, that no error boundary of yours catches, shows the widget's error card.

### Vue 3

```bash
npm install -D vue @vitejs/plugin-vue
```

```ts
// vite.config.ts
import { widget } from '@edgestation/sdk/vite';
import vue from '@vitejs/plugin-vue';
import { defineConfig } from 'vite';

export default defineConfig({ plugins: [vue(), widget()] });
```

```ts
// src/main.ts
import { vueWidget } from '@edgestation/sdk/vue';
import Cpu from './Cpu.vue';

export default vueWidget(Cpu);
```

```vue
<!-- src/Cpu.vue -->
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
```

Errors Vue catches (rendering, lifecycle hooks, watchers, template event handlers) show the widget's error card.

## Build

```bash
npx vite build
```

`widget()` builds the entry into `dist/main.js`, one ES module with every dependency bundled in, your framework's runtime included. CSS your code imports (React, Vue `<style>` blocks) goes to `dist/main.css`: add `"styles": "dist/main.css"` to the manifest so EdgeStation loads it into the widget's shadow root. The build warns when the manifest doesn't point at what it wrote, and when your own code (not your dependencies) calls `setTimeout` or `setInterval`, which keep running while the widget's page is off screen: use `api.after` and `api.every` instead.

Pass `widget({ entry: 'src/other.ts' })` for another entry. For TypeScript to accept CSS imports, add `"types": ["vite/client"]` to your `tsconfig.json`.

## Try it in EdgeStation

EdgeStation loads every folder of its widgets folder that has a `manifest.json`: `%APPDATA%\com.edgestation.dashboard\widgets\` on Windows, `~/.local/share/com.edgestation.dashboard/widgets/` on Linux (Settings → Widgets → Open folder). The folder name is the widget's id. Rather than copying your project there, link it once. On Windows, from a Command Prompt:

```bat
mklink /J "%APPDATA%\com.edgestation.dashboard\widgets\my-widget" C:\path\to\my-widget
```

On Linux, from a terminal:

```bash
ln -s /path/to/my-widget ~/.local/share/com.edgestation.dashboard/widgets/my-widget
```

Then rebuild on each change, and click Reload Edge at the bottom of the Settings sidebar to load the new build:

```bash
npx vite build --watch
```

To share the widget, ship the folder with `manifest.json`, `dist/` and any files it reads with `api.asset()`.

## Types

Everything is exported from `@edgestation/sdk` as types: `WidgetApi`, `WidgetModule`, `WidgetManifest`, `WidgetPermission`, `WidgetProps`, the settings types and the results of `api.system()`, `api.media`, `api.audio`, `api.apps` and `api.storage`.
