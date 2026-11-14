A widget written by hand needs nothing installed. This page is for widgets built with `@edgestation/sdk`: what to install for each way of writing one, how npm packages end up in the widget, and how to set up TypeScript.

## What the SDK is

EdgeStation hands each widget its `api` when it mounts, so nothing from the SDK runs inside the app. The SDK is a set of build-time helpers:

- the TypeScript types of the `api`, the manifest and the widget module,
- `defineWidget()`, to get those types in plain JavaScript too,
- adapters that mount a Svelte, React or Vue component as a widget,
- a Vite plugin that builds the one self-contained `dist/main.js` EdgeStation loads,
- a JSON Schema for `manifest.json`, for autocomplete and checks in your editor.

## Requirements

- [Node.js](https://nodejs.org), in a current LTS version, and a package manager: npm, pnpm or Yarn.
- [Vite](https://vite.dev) 6 or later. The SDK's build is a Vite plugin.
- EdgeStation, on Windows or Linux, to run the widget.

## What to install

Install everything as a dev dependency (`-D`): the build bundles what the widget uses into `dist/main.js`, so nothing in `node_modules` is loaded at runtime.

| You write the widget in  | Install                                                      |
| ------------------------ | ------------------------------------------------------------ |
| JavaScript or TypeScript | `@edgestation/sdk vite`                                      |
| Svelte 5                 | `@edgestation/sdk vite svelte @sveltejs/vite-plugin-svelte`  |
| React 19                 | `@edgestation/sdk vite react react-dom @vitejs/plugin-react` |
| Vue 3                    | `@edgestation/sdk vite vue @vitejs/plugin-vue`               |

For example, for React:

```bash
npm install -D @edgestation/sdk vite react react-dom @vitejs/plugin-react
```

The SDK lists Svelte (`^5`), React and React DOM (`>=19`), Vue (`^3.3`) and Vite (`>=6`) as optional peer dependencies: install only the framework you use. Importing an adapter without its framework fails at build time.

## Using packages from npm

Any package that runs in a browser works in a widget: install it and import it.

```bash
npm install -D date-fns
```

```ts src/main.ts
import { defineWidget } from '@edgestation/sdk';
import { formatDistanceToNow } from 'date-fns';

export default defineWidget({
	mount(el, api) {
		const started = new Date();
		const show = () => (el.textContent = `On for ${formatDistanceToNow(started)}`);
		show();
		api.every(60_000, show);
	}
});
```

The build bundles each package into `dist/main.js`, keeping only what you import, so the widget folder needs no `node_modules` to run. Whether a package is in `dependencies` or `devDependencies` makes no difference: it is bundled either way.

Some things to keep in mind:

- **No Node.js.** A widget runs in a web page. Packages that need Node.js built-ins, such as `fs` or `child_process`, can't run. To read the machine, use the [`api`](/docs/api); to call a web API that doesn't allow CORS, use [`api.fetch`](/docs/api/fetch).
- **Two web engines.** The page is Microsoft Edge WebView2 on Windows, so a recent Chromium, and WebKitGTK 4.1 on Linux, the GTK port of WebKit, Safari's engine. Use web features that both have, or check for a feature before you use it.
- **One copy per widget.** Every widget bundles its own copy of what it imports, its framework included: two React widgets each carry React. A framework-free widget is the smallest.
- **Their CSS too.** A stylesheet a package asks you to import goes into `dist/main.css` with yours, and EdgeStation loads it into the widget's shadow root. Set `"styles": "dist/main.css"` in the manifest.

## TypeScript

Name the entry `src/main.ts` (or `.tsx`) and the build compiles it. For your editor and `tsc`, a `tsconfig.json` like this one works:

```json tsconfig.json
{
	"compilerOptions": {
		"strict": true,
		"target": "esnext",
		"module": "esnext",
		"moduleResolution": "bundler",
		"lib": ["esnext", "dom", "dom.iterable"],
		"types": ["vite/client"],
		"verbatimModuleSyntax": true,
		"isolatedModules": true,
		"skipLibCheck": true,
		"noEmit": true
	},
	"include": ["src"]
}
```

`"types": ["vite/client"]` lets TypeScript accept CSS imports. Add `"jsx": "react-jsx"` for React.

Every type of the `api` is exported from `@edgestation/sdk`, as types only: see the [SDK reference](/docs/sdk#types).

```ts
import type { SystemInfo, WidgetApi } from '@edgestation/sdk';
```

In plain JavaScript, `defineWidget()` types the widget for you. A JSDoc comment does the same without importing anything at runtime:

```js
/** @type {import('@edgestation/sdk').WidgetModule} */
export default {
	mount(el, api) {
		// el and api are typed here
	}
};
```

## Versions

The SDK and EdgeStation are released together, with the same version number. Set the manifest's `minAppVersion` to the version of the SDK you build with: an older EdgeStation then shows an error card saying which version the widget needs, rather than a broken widget. The current version is 0.8.0. <!-- x-release-please-version -->
