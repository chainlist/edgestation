A widget can declare options that the user sets separately for each placement: two copies of the same widget can show different things. The user finds them in Settings → Pages, by clicking the widget.

## Declare them

List the settings under `settings` in the manifest, in the order they should appear:

```json manifest.json
{
	"name": "Performance",
	"settings": [
		{
			"key": "metric",
			"label": "Metric",
			"type": "select",
			"default": "cpu",
			"options": [
				{ "value": "cpu", "label": "CPU load" },
				{ "value": "net", "label": "Internet usage" }
			]
		},
		{
			"key": "direction",
			"label": "Direction",
			"type": "select",
			"when": { "metric": "net" },
			"options": [
				{ "value": "down", "label": "Download" },
				{ "value": "up", "label": "Upload" }
			]
		}
	]
}
```

| Field         | Meaning                                                                               |
| ------------- | ------------------------------------------------------------------------------------- |
| `key`         | Name of the value in `api.settings`.                                                  |
| `label`       | Shown next to the field.                                                              |
| `type`        | `select` (one of `options`) or `text` (anything the user types, such as a URL).       |
| `options`     | `select` only: the choices, each a saved `value` and a shown `label`.                 |
| `source`      | `select` only. Adds the choices of a list EdgeStation reads (see below).              |
| `default`     | The value until the user sets one. Defaults to the first option, or empty for `text`. |
| `placeholder` | `text` only. A hint shown while the field is empty.                                   |
| `secret`      | `text` only. `true` hides the value, as in a password field (see below).              |
| `when`        | Only show this setting while the named settings have these values.                    |
| `unless`      | Hide this setting while the named settings have these values.                         |

### A text setting

```json
{
	"key": "url",
	"label": "URL",
	"type": "text",
	"placeholder": "https://example.com"
}
```

A `text` value is saved when the user presses Enter or leaves the field, not on each key.

For a token or a key, add `"secret": true`: the field then hides what the user types. It only hides it on screen. The value is saved as plain text in `settings.json`, like any other setting, and older versions of EdgeStation show it in a plain field.

### Choices from a list

A `select` can add choices that are only known on the user's machine, such as the apps playing sound. Name the list in `source`: when the settings show, EdgeStation reads it and adds its choices after `options`.

```json
{
	"key": "channel",
	"label": "Channel",
	"type": "select",
	"default": "all",
	"options": [{ "value": "all", "label": "All" }],
	"source": "audioApps"
}
```

| `source`    | Its choices                                                                                                                                                                                        |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `audioApps` | The apps with sound on the default output ([`api.audio`](/docs/api/audio)), by name, each saved as its executable's file name (`program`, such as `Discord.exe` on Windows or `firefox` on Linux). |

A saved value that isn't in the list when the settings show, an app that isn't running, stays saved: the field marks it (`Discord (not running)`) and the widget gets it as it is. So a widget handles a value it can't find right now, for example by greying out until the app runs.

### Settings that depend on others

`when` maps other settings' keys to a value. The setting only shows while every one of them has that value: above, Direction only shows while Metric is Internet usage. A list of values matches any of them, so `"when": { "metric": ["cpu", "gpu"] }` shows a setting for both. Its value stays in `api.settings` either way, so read it only when it applies.

`unless` is the opposite: the setting hides while every one of the named settings has that value. Use it when the values that hide a setting are fewer than the ones that show it, or when the other setting has a `source`, whose choices a `when` list can't name. The Mixer hides Show for one channel with `"unless": { "channel": "outputs" }`, because its other channels include every app with sound.

## Read them

The widget reads the values from `api.settings`, by key:

```js
mount(el, api) {
	if (api.settings.metric === 'net') {
		// api.settings.direction is 'down' or 'up'
	}
}
```

- Every declared key is always present: the saved value, else its default.
- Values are strings.
- A saved `select` value that is no longer one of the options (you removed it in a new version of the widget) falls back to the default, unless the select has a `source`: then any saved value stays.

## When they change

When the user changes a setting, the widget is unmounted and mounted again with the new values. `api.settings` never changes while the widget is mounted, so read it once, in `mount` or at the top of your component. Data that should survive the remount belongs in [`api.storage`](/docs/api/storage).

## The Profile setting

Widgets with `webview` in their [permissions](/docs/manifest#permissions) also get a **Profile** setting, which the app adds and handles itself: the browser profile, with its own sign-ins and cookies, that [`api.webview`](/docs/api/webview) opens pages in. Changing it remounts the widget too.
