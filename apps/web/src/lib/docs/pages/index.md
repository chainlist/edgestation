EdgeStation fills a secondary display with swipeable pages of widgets. In these docs, "the Edge" is that display: the Corsair Xeneon Edge, which EdgeStation is made for, or any other one. Every widget on it, the built-in ones included, is a small web module that EdgeStation loads at runtime, like a plugin, and you can write your own.

## What a widget is

A widget is a folder with at least two files:

```text
my-widget/
  manifest.json   its name, default size and settings
  main.js         an ES module that draws it
  styles.css      (optional)
  icon.svg        (optional)
```

`manifest.json` describes the widget. `main.js` default-exports an object with a `mount` function, which EdgeStation calls with an element to draw into and the `api` object:

```js main.js
export default {
	mount(el, api) {
		el.textContent = `Hello from ${api.manifest.name}`;
	}
};
```

```json manifest.json
{ "name": "Hello" }
```

That is a complete widget. Everything else is optional: a stylesheet, an icon, [settings](/docs/settings) the user changes per placement, and the [api](/docs/api) to read the machine's performance, show what is playing, start apps, call web APIs or keep data.

## Two ways to write one

- **By hand.** Write `main.js` in plain JavaScript, with no build step. Good for a small widget.
- **With the SDK.** `@edgestation/sdk` lets you write the widget in TypeScript, Svelte, React or Vue, use packages from npm, and build it into the one `main.js` EdgeStation loads. It brings the types of the whole `api`, an adapter for each framework, a Vite plugin and a JSON Schema for the manifest. The built-in widgets are made this way.

The [Quick start](/docs/getting-started) writes the same widget both ways.

## Where widgets live

EdgeStation looks for widgets in three places. `<data>` is EdgeStation's data folder: `%APPDATA%\com.edgestation.dashboard` on Windows, and `~/.local/share/com.edgestation.dashboard` on Linux (under `$XDG_DATA_HOME` when it is set).

| Folder                     | What is there                                                                                                                                            |
| -------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `<data>/widgets/<id>/`     | Your own widgets. Settings → Widgets → Open folder opens it.                                                                                             |
| `<data>/marketplace/<id>/` | Widgets installed from Settings → Widgets → Browse. EdgeStation keeps this folder: it updates them, and deletes a widget's folder when you uninstall it. |
| Bundled with the app       | The built-in widgets, whose sources are in the repository.                                                                                               |

The folder name is the widget's id. A widget of yours replaces a marketplace or built-in widget with the same id, so you can start from a built-in widget's source and change it.

## Things to know

- **Any display.** A widget can't assume the Edge's size, shape or touch. It fills a grid cell the user sizes, so lay it out relative to that cell: see [Styling and theming](/docs/styling).
- **Isolated.** Each widget renders in its own shadow root and runs in its own error boundary. Its CSS can't leak into the app, and if it fails, an error card with a Retry button replaces it while the rest of the Edge keeps running.
- **No focus.** Touching the Edge leaves the keyboard with the app in front, so a game keeps running. A text field (`input`, `textarea`, editable content) takes the keyboard when the user touches it, and gives it back once they touch something else on the Edge. Focusing one from code (`focus()`) doesn't bring the keyboard until it is touched.
- **Light.** A dashboard runs all day. Values are read when a widget asks for them, and timers stop while a widget's page is off screen: see [Lifecycle and errors](/docs/lifecycle).
- **Windows and Linux.** EdgeStation runs on Windows, and on Linux under X11 (Wayland sessions run it on XWayland). Parts of the `api` (media, audio, apps, programs, Wi-Fi) wrap features of each system, so some details differ: each page says how. Widgets run in Microsoft Edge WebView2 on Windows and in WebKitGTK on Linux, so use web features that both have: see [Dependencies](/docs/dependencies#using-packages-from-npm).

## Examples

The built-in widgets are complete examples, in the [widgets folder](https://github.com/chainlist/edgestation/tree/main/widgets) of the repository:

| Widget                                                                                      | Shows how to use                                                  |
| ------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| [`clock`](https://github.com/chainlist/edgestation/tree/main/widgets/clock)                 | A complete small widget, with `select` settings.                  |
| [`performance`](https://github.com/chainlist/edgestation/tree/main/widgets/performance)     | Settings shown on condition, `api.system()`, `api.topPrograms()`. |
| [`media`](https://github.com/chainlist/edgestation/tree/main/widgets/media)                 | `api.media` and buttons.                                          |
| [`mixer`](https://github.com/chainlist/edgestation/tree/main/widgets/mixer)                 | `api.audio`, a `source` setting, faders dragged by touch, meters. |
| [`todo`](https://github.com/chainlist/edgestation/tree/main/widgets/todo)                   | `api.storage`, text input and `api.scrollbars`.                   |
| [`logs-timeline`](https://github.com/chainlist/edgestation/tree/main/widgets/logs-timeline) | Data that every copy of a widget shares and keeps in step.        |
| [`launcher`](https://github.com/chainlist/edgestation/tree/main/widgets/launcher)           | `api.apps`.                                                       |
| [`youtube`](https://github.com/chainlist/edgestation/tree/main/widgets/youtube)             | `api.webview`.                                                    |
| [`url`](https://github.com/chainlist/edgestation/tree/main/widgets/url)                     | A `text` setting, a web view's mobile version and Home button.    |
| [`weather`](https://github.com/chainlist/edgestation/tree/main/widgets/weather)             | `api.fetch`, and a layout for each value of a setting.            |
