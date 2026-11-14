`mount(el, api)` receives the `api` object: everything a widget can read from EdgeStation and the machine, and the helpers that tie its timers and views to its lifetime. Its type is `WidgetApi`, from `@edgestation/sdk`.

```js
export default {
	mount(el, api) {
		el.textContent = api.manifest.name;
		api.every(5000, async () => {
			const { cpu } = await api.system();
			el.title = `CPU ${Math.round(cpu.load)} %`;
		});
	}
};
```

The API is deliberately small for now, and will grow. Set the manifest's [`minAppVersion`](/docs/manifest#minappversion) to the EdgeStation version you build against, so an older app doesn't load a widget that calls what it lacks.

## Members

| Member                       | What it does                                                                                                    |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------- |
| `instanceId`                 | Id of this placement. Two copies of a widget get different ids.                                                 |
| `manifest`                   | The widget's [manifest](/docs/manifest), with `id` set to its folder name.                                      |
| `settings`                   | This placement's [setting values](/docs/settings#read-them), by key.                                            |
| `system()`                   | The machine's CPU, GPU, network and Wi-Fi. [System and programs](/docs/api/system)                              |
| `topPrograms(n)`             | The `n` programs using the most CPU. [System and programs](/docs/api/system#api-topprograms)                    |
| `media`                      | What is playing, and its playback controls. [Media](/docs/api/media)                                            |
| `apps`                       | The apps of the Start menu or the app menu: list them, start them, show their icons. [Apps](/docs/api/apps)     |
| `audio`                      | Volumes, mute and levels of the output, the microphone and each app. [Audio](/docs/api/audio)                   |
| `fetch(input, init?)`        | The web's `fetch`, without CORS. [Fetch](/docs/api/fetch)                                                       |
| `storage`                    | Data kept across remounts and restarts. [Storage](/docs/api/storage)                                            |
| `webview(el, url, options?)` | A web page in a native browser view over `el`. [Web views](/docs/api/webview)                                   |
| `every(ms, fn)`              | A `setInterval` managed for the widget. [Timers and helpers](/docs/api/helpers#api-every)                       |
| `after(ms, fn)`              | A `setTimeout` managed for the widget. [Timers and helpers](/docs/api/helpers#api-after)                        |
| `guard(fn)`                  | Wraps a callback so its errors show the error card. [Timers and helpers](/docs/api/helpers#api-guard)           |
| `loading(task)`              | Shows a loading indicator on the card until `task` settles. [Timers and helpers](/docs/api/helpers#api-loading) |
| `onDispose(fn)`              | Runs `fn` when the widget unmounts. [Timers and helpers](/docs/api/helpers#api-ondispose)                       |
| `asset(path)`                | URL of a file in the widget folder. [Timers and helpers](/docs/api/helpers#api-asset)                           |
| `scrollbars(el)`             | The app's overlay scrollbars for a scrolling element. [Timers and helpers](/docs/api/helpers#api-scrollbars)    |

## Permissions

`system()`, `topPrograms()`, `media`, `apps`, `audio`, `fetch` and `webview` work only when the manifest lists their [permission](/docs/manifest#permissions). Otherwise a call shows the widget's error card and throws. The other members need nothing.

## Released on unmount

Everything a widget registers through the `api` is released when it unmounts: `every` and `after` timers are cleared, web views close, scrollbars are removed and unanswered `fetch` requests are cancelled. Members that return a cleanup function (`every`, `after`, `webview`, `scrollbars`) also let you release one sooner.

## Read on demand

`system()`, `topPrograms()`, `media.current()`, `apps.list()` and `audio.mixer()` read the machine when called. Nothing monitors in between, so a widget pays for what it shows, when it shows it. Poll them with `api.every`, which stops while the widget's page is off screen. The audio mixer and the media sessions are the exception: the system tells the app when they change, so read them again from `audio.onChange` and `media.onChange` instead of polling.

## instanceId

A string that identifies this placement of the widget, and stays the same across restarts. Two copies of the widget on the Edge get different ids. [`api.storage.instance`](/docs/api/storage) already keeps data per placement; use `instanceId` when you need the id itself, for example as a key in data shared through `api.storage.widget`.
