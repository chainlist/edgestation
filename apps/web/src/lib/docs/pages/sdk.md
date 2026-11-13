`@edgestation/sdk` has one entry point per job, so a widget only pulls in what it uses. Nothing from it runs inside EdgeStation: the app hands each widget its `api`, and the SDK provides the types, the adapters and the build.

## Entry points

| Import                                  | Exports                                                           |
| --------------------------------------- | ----------------------------------------------------------------- |
| `@edgestation/sdk`                      | `defineWidget()`, and every type below.                           |
| `@edgestation/sdk/svelte`               | `svelteWidget()`. Needs `svelte` 5.                               |
| `@edgestation/sdk/react`                | `reactWidget()`. Needs `react` and `react-dom` 19 or later.       |
| `@edgestation/sdk/vue`                  | `vueWidget()`. Needs `vue` 3.3 or later.                          |
| `@edgestation/sdk/vite`                 | `widget()` and its `WidgetBuildOptions`. Needs `vite` 6 or later. |
| `@edgestation/sdk/manifest.schema.json` | The JSON Schema of `manifest.json`.                               |

## Functions

### `defineWidget()`

```ts
function defineWidget(widget: WidgetModule): WidgetModule;
```

Types a widget's default export, so editors know `el` and `api` without annotations, in plain JavaScript too. It returns `widget` unchanged.

```js
import { defineWidget } from '@edgestation/sdk';

export default defineWidget({
	mount(el, api) {
		el.textContent = api.manifest.name;
	}
});
```

### `svelteWidget()`

```ts
function svelteWidget(component: Component<WidgetProps>): WidgetModule;
```

Mounts a Svelte 5 component in the widget's root with the `api` as its prop, and unmounts it with the widget. See [Svelte](/docs/frameworks#svelte-5).

### `reactWidget()`

```ts
function reactWidget(component: ComponentType<WidgetProps>): WidgetModule;
```

Renders a React component in a React root on the widget's root, with the `api` as its prop, and unmounts the root with the widget. Uncaught render errors show the error card. See [React](/docs/frameworks#react-19).

### `vueWidget()`

```ts
function vueWidget(component: Component): WidgetModule;
```

Mounts a Vue 3 component as an app on the widget's root, with the `api` as its prop (`defineProps<WidgetProps>()`), and unmounts it with the widget. Errors Vue catches show the error card. See [Vue](/docs/frameworks#vue-3).

### `widget()`

```ts
function widget(options?: WidgetBuildOptions): Plugin;

interface WidgetBuildOptions {
	/** The entry module, relative to the project root. */
	entry?: string;
}
```

The Vite plugin that builds the widget into `dist/main.js` (and `dist/main.css`). See [Build, test and share](/docs/build#the-vite-plugin).

## Types

Everything is exported from `@edgestation/sdk` as types:

```ts
import type { WidgetApi, WidgetProps, SystemInfo } from '@edgestation/sdk';
```

| Type                                            | What it describes                                                                          |
| ----------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `WidgetModule`                                  | The default export of a widget's entry: `{ mount(el, api) }`.                              |
| `WidgetApi`                                     | The [`api`](/docs/api) object.                                                             |
| `WidgetProps`                                   | `{ api: WidgetApi }`: the props of a component mounted by an adapter.                      |
| `Cleanup`                                       | `() => void`: what `mount`, `every`, `after`, `webview` and `scrollbars` may return.       |
| `WidgetManifest`                                | `manifest.json`, as `api.manifest` holds it, with `id`.                                    |
| `WidgetSetting`, `SelectSetting`, `TextSetting` | A setting declared in the manifest.                                                        |
| `SettingSource`                                 | The lists a `select` can add choices from: [`source`](/docs/settings#choices-from-a-list). |
| `WidgetSettingValues`                           | `api.settings`: the values by key, all strings.                                            |
| `SystemInfo`, `NetRate`                         | What [`api.system()`](/docs/api/system) resolves to.                                       |
| `ProgramUsage`                                  | One entry of [`api.topPrograms()`](/docs/api/system#api-topprograms).                      |
| `MediaApi`, `MediaInfo`, `MediaAction`          | [`api.media`](/docs/api/media), its session and its actions.                               |
| `AudioApi`, `AudioMixer`, `AudioDevice`         | [`api.audio`](/docs/api/audio), what `mixer()` resolves to, and a device.                  |
| `AudioApp`, `AudioLevels`, `AudioOutput`        | An app's sound, what `levels()` resolves to, and an output `setOutput()` takes.            |
| `AudioTarget`                                   | What `setVolume()` and `setMuted()` change: `'output'`, `'input'` or an app's `id`.        |
| `AppsApi`, `AppInfo`                            | [`api.apps`](/docs/api/apps) and one app.                                                  |
| `StorageApi`, `WidgetStorage`                   | [`api.storage`](/docs/api/storage) and one of its scopes.                                  |
| `WebviewOptions`                                | The options of [`api.webview()`](/docs/api/webview#options).                               |

The types document every member in your editor too: hover a member of `api` to read what it does.
