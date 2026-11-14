Every widget folder has a `manifest.json`. EdgeStation only lists folders that have one, and reads it to show the widget in Settings and to load it.

```json manifest.json
{
	"$schema": "./node_modules/@edgestation/sdk/manifest.schema.json",
	"name": "My widget",
	"version": "1.0.0",
	"description": "What it shows.",
	"author": "You",
	"minAppVersion": "0.1.0",
	"main": "main.js",
	"styles": "styles.css",
	"icon": "icon.svg",
	"size": { "w": 2, "h": 1 },
	"permissions": ["fetch"]
}
```

## Fields

Only `name` is required. Paths are relative to the widget folder.

| Field           | Meaning                                                                                                       |
| --------------- | ------------------------------------------------------------------------------------------------------------- |
| `name`          | Required. The name shown in Settings.                                                                         |
| `version`       | The widget's own version.                                                                                     |
| `description`   | What the widget shows, for Settings.                                                                          |
| `author`        | Who made it.                                                                                                  |
| `minAppVersion` | The oldest EdgeStation the widget runs on, as `major.minor.patch`. See [below](#minappversion).               |
| `main`          | The entry ES module. Defaults to `main.js`; `dist/main.js` for a widget built with the SDK.                   |
| `styles`        | A stylesheet loaded into the widget's shadow root; `dist/main.css` when the SDK build writes one.             |
| `icon`          | An image shown next to the widget in Settings. See [below](#icon).                                            |
| `size`          | `{ "w", "h" }`: the default size in grid cells when the widget is added to a page. Whole numbers, at least 1. |
| `settings`      | Options the user sets for each placement. See [Settings](/docs/settings).                                     |
| `permissions`   | The parts of the `api` the widget uses, such as `["media", "fetch"]`. See [below](#permissions).              |
| `$schema`       | Only for editors: the JSON Schema that checks this file. EdgeStation ignores it.                              |

The widget's id is not in the manifest: it is the name of its folder. The widget reads its own manifest in `api.manifest`, with `id` set.

## `minAppVersion`

`minAppVersion` is the version of EdgeStation whose `api` the widget uses. An older app shows the widget's error card, saying which version it needs, instead of loading the widget and letting it fail on a missing `api` member.

With the SDK, set it to the SDK version you build with: the two are released together. The `0.1.0` in the examples of these pages is only a placeholder.

## `permissions`

Six members of the `api` reach the machine or the web, and a widget must ask for each one it uses:

| Permission | Gives                                                                             |
| ---------- | --------------------------------------------------------------------------------- |
| `system`   | [`api.system()` and `api.topPrograms()`](/docs/api/system)                        |
| `media`    | [`api.media`](/docs/api/media)                                                    |
| `apps`     | [`api.apps`](/docs/api/apps)                                                      |
| `audio`    | [`api.audio`](/docs/api/audio)                                                    |
| `fetch`    | [`api.fetch`](/docs/api/fetch)                                                    |
| `webview`  | [`api.webview`](/docs/api/webview), and a Profile choice in the widget's settings |

The other members (`storage`, `every`, `loading`, ...) only touch the widget itself, so they need nothing. A manifest without `permissions` gets none.

Calling a member the manifest doesn't list throws, and shows the widget's error card: it tells the user that the widget needs an update. The error in the console names the permission to add.

When the user clicks Install in Browse, a dialog lists what the widget will be able to do, one short sentence per permission, and asks them to confirm. A new version that asks for more permissions does the same with only the new ones: Update all skips it, and it waits for the user to click its own Update button. A widget that asks for no permissions installs with no dialog. Built-in widgets and widgets dropped into the widgets folder show no dialog.

Permissions tell the user what a widget means to use. They are not a sandbox: widgets run inside the Edge page itself, so install only widgets you trust.

## `icon`

A square SVG or PNG on a transparent background. Only its shape counts: Settings draws it in the accent color, like the icons of the built-in widgets. Without one, the widget gets a puzzle piece.

## The schema

The SDK ships a JSON Schema for the manifest. Point `$schema` at it and editors such as VS Code complete the fields as you type, show what each one means, and flag mistakes: a missing `name`, a misspelled field, a `select` setting without options.

```json
{
	"$schema": "./node_modules/@edgestation/sdk/manifest.schema.json"
}
```

The path is relative to the manifest. The schema is also exported as `@edgestation/sdk/manifest.schema.json`, for tools that validate manifests themselves.
