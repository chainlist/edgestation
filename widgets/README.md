# Writing a widget

A widget is a folder with a manifest and an ES module. Here, "the Edge" is the display EdgeStation shows its pages on: the Corsair Xeneon Edge it is made for, or any other secondary display. Widgets can't assume its size, shape or touch: lay them out in their cell (see [Styling and theming](#styling-and-theming)).

EdgeStation looks for widgets in three places:

- `%APPDATA%\com.edgestation.dashboard\widgets\<id>\`: your own widgets (Settings → Widgets → Open folder).
- `%APPDATA%\com.edgestation.dashboard\marketplace\<id>\`: widgets installed from the marketplace (Settings → Widgets → Browse), which EdgeStation updates on its own.
- This folder (`widgets/` in the repo): the built-in examples, bundled with the app.

The folder name is the widget id. A user widget replaces a marketplace or built-in one with the same id. To list yours in the marketplace, see [Publish to the marketplace](../apps/web/src/lib/docs/pages/build.md#publish-to-the-marketplace).

The built-in widgets are written in Svelte and also have a `package.json` that builds them (see [Built-in widgets](#built-in-widgets-svelte)). EdgeStation ignores that file; your own widgets don't need one.

```
my-widget/
  manifest.json
  main.js
  styles.css      (optional)
  icon.svg        (optional)
```

`main.js` can be written by hand, as below. To write the widget in TypeScript, Svelte, React or Vue and build `main.js` from it, use the [`@edgestation/sdk`](../packages/sdk/README.md) package: it has the types of everything on this page, adapters that mount a component as a widget, a Vite plugin for the build and a JSON Schema for `manifest.json`.

## manifest.json

```json
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

Only `name` is required. `main` defaults to `main.js`. `size` is the default size in grid cells when the widget is added to a page. `icon` is an image (SVG or PNG, square, on a transparent background) shown next to the widget in Settings → Widgets; only its shape counts, as it is drawn in the accent color. Without one, the widget gets a puzzle piece.

`permissions` lists the parts of the `api` the widget uses: `system` (for `api.system()` and `api.topPrograms()`), `media`, `apps`, `audio`, `fetch` and `webview`. Before a marketplace install, a dialog tells the user what each one lets the widget do and asks them to confirm. An update that adds permissions waits for the user too, and its dialog lists only the new ones. A widget that asks for no permissions installs with no dialog. Calling a member without its permission shows the widget's error card and throws. `webview` also gives the widget's settings a Profile choice. Permissions tell the user what a widget means to use; they are not a sandbox, as widgets run inside the Edge page.

`minAppVersion` is the oldest EdgeStation the widget runs on, as `major.minor.patch`: the version whose `api` it uses. An older app shows the widget's error card, saying which version it needs, instead of loading it. `$schema` is only for editors: with the SDK installed, it points them at its schema, which checks the manifest and completes its fields as you type (EdgeStation ignores it).

## Settings

A widget can declare options that the user sets separately for each placement (Settings → Pages → click the widget). List them under `settings`, in the order they should appear:

```json
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
```

| Field         | Meaning                                                                            |
| ------------- | ---------------------------------------------------------------------------------- |
| `key`         | Name of the value in `api.settings`.                                               |
| `type`        | `select` (one of `options`) or `text` (anything the user types, such as a URL).    |
| `options`     | `select` only: the choices, each a saved `value` and a shown `label`.              |
| `source`      | `select` only. Adds the choices of a list EdgeStation reads (see below).           |
| `default`     | Value until the user sets one. Defaults to the first option, or empty for `text`.  |
| `placeholder` | `text` only. Optional hint shown while the field is empty.                         |
| `secret`      | `text` only. `true` hides the value, as in a password field, for a token or a key. |
| `when`        | Optional. Show only while the named settings have these values, or one of a list.  |
| `unless`      | Optional. Hide while the named settings have these values, or one of a list.       |

With `"source": "audioApps"`, a `select` also lists the apps with sound on the default output (`api.audio`), after `options`, read when the settings show; each is saved as its executable's file name (`AudioApp.program`, such as `Discord.exe`). A saved value that isn't in the list then (an app that isn't running) stays saved, marked `(not running)` in the field, and the widget gets it as it is.

The widget reads the values from `api.settings`, e.g. `api.settings.metric`. Every declared key is always present (saved value, else its default), and a saved `select` value that is no longer one of the options falls back to the default (any saved value stays when the select has a `source`). A `text` value is saved when the user presses Enter or leaves the field, not on each key. When the user changes a setting, the widget is unmounted and mounted again with the new values.

## main.js

The module default-exports an object with a `mount` function:

```js
export default {
	mount(el, api) {
		el.textContent = 'Hello';
		api.every(1000, () => {
			/* runs every second, stopped automatically on unmount */
		});
		return () => {
			/* optional cleanup */
		};
	}
};
```

`mount` may be async. Relative imports (`import './chart.js'`) work.

## The `api` object

The API is intentionally small for now; it will grow.

| Member                       | What it does                                                                                                                                                               |
| ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `instanceId`                 | Id of this placement. Two copies of a widget get different ids.                                                                                                            |
| `manifest`                   | The widget's manifest, with `id` set.                                                                                                                                      |
| `settings`                   | This placement's setting values, by key (see Settings above).                                                                                                              |
| `system()`                   | Promise of the machine's performance (see below).                                                                                                                          |
| `topPrograms(n)`             | Promise of the `n` programs using the most CPU (see below).                                                                                                                |
| `media`                      | What is playing and its playback controls (see below).                                                                                                                     |
| `apps`                       | The apps of the Start menu: list them, start them, show their icons (see below).                                                                                           |
| `audio`                      | Volumes, mute and level meters of the default output, the microphone and each app (see below).                                                                             |
| `fetch(input, init?)`        | The web's `fetch`, sent by the app so that CORS doesn't apply: any API answers (see below).                                                                                |
| `storage`                    | Data kept across remounts and restarts, for this placement or for every copy of the widget (see below).                                                                    |
| `asset(path)`                | URL of a file in the widget folder (images, JSON, ...).                                                                                                                    |
| `every(ms, fn)`              | `setInterval` that is cleared on unmount, paused while its page is off screen or the Edge is frozen, and whose errors are caught. Returns a function that stops it.        |
| `after(ms, fn)`              | `setTimeout` that is cleared on unmount, waits while its page is off screen (not while frozen), and whose errors are caught. Returns a function that cancels it.           |
| `webview(el, url, options?)` | Shows a web page in a native browser view over element `el`, for sites that refuse iframes (see below). Needs the `webview` permission. Returns a function that closes it. |
| `scrollbars(el)`             | Gives the scrolling element `el` the app's overlay scrollbars instead of the native ones (see below). Returns a function that removes them.                                |
| `guard(fn)`                  | Wraps an event handler so an error in it shows the widget's error card.                                                                                                    |
| `loading(task)`              | Shows that the widget is loading until the promise `task` settles: dots for the first load, a bar along the top of the card after that (see below). Returns `task`.        |
| `onDispose(fn)`              | Runs `fn` when the widget unmounts.                                                                                                                                        |

### `api.system()`

Reads the machine when called; nothing monitors it in between. CPU load and network speeds are differences between two reads, so they cover the time since the previous call (from any widget). Calls less than 500 ms apart share one read, so several widgets polling every second cost one read per second. Keep a history yourself if you want to chart one:

```js
api.every(1000, async () => {
	const { cpu } = await api.system();
	el.textContent = `${Math.round(cpu.load)} %`;
});
```

| Field                            | Meaning                                                                                 |
| -------------------------------- | --------------------------------------------------------------------------------------- |
| `hostname`                       | Machine name.                                                                           |
| `wifi`                           | Connected Wi-Fi network (its Windows profile name, the SSID unless renamed), or `null`. |
| `cpu.load`                       | Load of all cores, percent.                                                             |
| `gpu.load`, `gpu.temperature`    | First NVIDIA GPU: percent and °C. `gpu` is `null` without one.                          |
| `net.down`, `net.up`             | Internet traffic since the previous call, bytes per second.                             |
| `net.avg1m`, `.avg5m`, `.avg15m` | `{ down, up }` averaged over the calls of the last 1, 5 or 15 minutes.                  |

The averages only know the calls widgets made: they cover the full window while a widget keeps polling, and less right after polling starts or resumes (for example when a page comes back on screen). CPU temperature is not available yet: Windows doesn't expose it to normal apps.

### `api.topPrograms(count)`

The programs that used the most CPU since the previous call (from any widget), busiest first: at most `count`, fewer when not that many used any. Like `api.system()`, it reads when called and calls less than 500 ms apart share one read. It lists every process on the machine, which costs more than `api.system()`, so it is separate: only call it while you show the result.

```js
api.every(1000, async () => {
	const [top] = await api.topPrograms(1);
	el.textContent = top ? `${top.name} · ${Math.round(top.cpu)} %` : 'Idle';
});
```

| Field  | Meaning                                                                                                                                                                                       |
| ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name` | What Task Manager calls it: the executable's description (`THE FINALS`, `Google Chrome`), else its file name (`crashpad_handler.exe`). All the processes with that name count as one program. |
| `cpu`  | Percent of all logical processors, like Task Manager.                                                                                                                                         |

Services and system processes are included, as in Task Manager.

### `api.media`

The media session Windows considers current: the one the volume flyout shows and the media keys control. Spotify, browsers (YouTube, ...) and most players publish one. Read it once, then again when the system says it changed, so nothing polls:

```js
const show = api.guard(async () => {
	const now = await api.media.current();
	el.textContent = now ? `${now.title} · ${now.artist}` : 'Nothing playing';
});
show();
api.media.onChange(show);
```

| Member            | What it does                                                                                                                         |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `current()`       | Promise of the session below, or `null` when nothing is playing.                                                                     |
| `art()`           | Promise of the album art from the latest `current()` call, as a data URL (`null` without art).                                       |
| `control(action)` | Asks the app to `'play'`, `'pause'`, skip to `'next'` or go to `'previous'`. Resolves `false` if it declined.                        |
| `seek(position)`  | Asks the app to go to `position` seconds in the track: `seek(0)` plays it from the start. Resolves `false` if it declined.           |
| `onChange(fn)`    | Calls `fn` when a session opens, closes or becomes current, or its track, playback state or timeline changes: read `current()` then. |

| Field                      | Meaning                                                                                                    |
| -------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `title`, `artist`, `album` | As the app reports them; empty when it doesn't.                                                            |
| `app`                      | Id of the app that owns the session (`Spotify.exe`, `MSEdge`, ...).                                        |
| `playing`                  | `true` while playing, `false` when paused or stopped.                                                      |
| `rate`                     | Playback speed, `1` at normal speed: while playing, `position` moves on by `rate` seconds per second.      |
| `position`, `duration`     | Seconds, `position` as of the call. `null` when the app reports no timeline (see below).                   |
| `art`                      | A number that changes when the art does, or `null`. Call `art()` only when it changes: the image is large. |
| `can`                      | `{ play, pause, next, previous, seek }`: which actions the app accepts right now.                          |

`fn` gets no arguments: it doesn't say what changed or in which session, so read the sessions again. A burst of changes (a new track often sends several) calls it at most every 100 ms. It only listens while the widget's page shows and the Edge isn't frozen, and calls `fn` once when the widget shows again. The position of a playing track moves on without a call: advance it yourself from the last read, by `rate` seconds per second, with `api.every`, which asks the system nothing.

Zen and Firefox report the timeline only on play, pause and seek, and blank it out about once a minute. While a widget listens with `onChange`, or widgets keep calling `current()` at least every 5 seconds, the position carries on from the last one reported. After a longer gap it stays `null` until the next play, pause or seek, rather than guessing across a pause or seek the widget didn't see.

### `api.apps`

The apps of the Start menu's All apps list, Store apps included, the way the Start menu lists, starts and draws them:

```js
const apps = await api.apps.list();
const terminal = apps.find((app) => app.name === 'Terminal');
img.src = api.apps.icon(terminal.id, 96);
button.onclick = () => api.apps.launch(terminal.id);
```

| Member            | What it does                                                                                                                                                                                                         |
| ----------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `list()`          | Promise of every app, by name: `{ id, name }`. Read when called (a few hundred ms), so call it when you show the list, not on a timer.                                                                               |
| `launch(id)`      | Starts the app with its default action. Its window is asked for on the main screen rather than the Edge (apps that remember their position may still open where they were). Rejects when the app is no longer there. |
| `icon(id, size?)` | URL of the app's icon, a PNG `size` pixels square (16 to 256, default 64), for an `<img>`. Ask for the size it shows at in device pixels. The image fails to load when the app is gone.                              |

`id` is the app's name in the shell's Apps folder (its AppUserModelID, or a path for a program without one). It stays the same across restarts, so save it with `api.storage` to remember an app. Like the Start menu, `launch` opens a new window for most apps when one is already open.

### `api.audio`

The machine's sound as Windows' volume mixer shows it. Read it once, then again when Windows says it changed, so nothing polls:

```js
const show = api.guard(async () => {
	const { output } = await api.audio.mixer();
	el.textContent = output ? `${Math.round(output.volume * 100)} %` : 'No output';
});
show();
api.audio.onChange(show);
```

| Member                      | What it does                                                                                                                                          |
| --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mixer()`                   | Promise of `{ output, input, apps }`: the default output and microphone (`null` without one) and the apps on that output (see below).                 |
| `onChange(fn)`              | Calls `fn` when the mixer changes (a volume, a mute, an app's sound opening, closing, starting or stopping, the default device): read `mixer()` then. |
| `onLevels(fn)`              | Calls `fn` with `{ output, input, apps }`, the peaks of the last few milliseconds, 0 to 1, 25 times a second while there is sound, for meters.        |
| `levels()`                  | Promise of the same levels, read once.                                                                                                                |
| `setVolume(target, volume)` | Sets the volume of `'output'`, `'input'` or an app's `id`, from 0 to 1. Resolves `false` when it is gone (the app closed, the device was unplugged).  |
| `setMuted(target, muted)`   | Mutes or unmutes the same targets. Muting `'input'` also mutes the default microphone for calls when it is another device.                            |
| `outputs()`                 | Promise of the output devices plugged in and enabled, by name: `{ id, name }`.                                                                        |
| `setOutput(id)`             | Makes output `id` the default one, as Sound settings does; calls keep theirs. Resolves `false` when it is no longer plugged in.                       |
| `icon(path, size?)`         | URL of the icon of an app's executable (its `path`), a PNG `size` pixels square (16 to 256, default 64), for an `<img>`.                              |

A device is `{ id, name, volume, muted }`, `volume` from 0 to 1. The microphone's `muted` is `true` only when the one for calls is muted too. Each app is one group of sessions, the way the volume mixer groups them, playing or not, by name with System sounds last:

| Field     | Meaning                                                                                                             |
| --------- | ------------------------------------------------------------------------------------------------------------------- |
| `id`      | Names the app's sound while it runs, for `setVolume`, `setMuted` and `levels()`. Not kept across restarts.          |
| `name`    | The name the app gives its sound, else what Task Manager calls it, else its executable's file name.                 |
| `path`    | Its executable, for `icon()`; `null` for System sounds.                                                             |
| `program` | Its executable's file name (`Discord.exe`), `null` for System sounds: it names the app across restarts and updates. |
| `volume`  | From 0 to 1, a share of the output's volume.                                                                        |
| `muted`   | `true` while muted.                                                                                                 |
| `active`  | `true` while it plays, `false` while its session is open but silent.                                                |

`onChange` listens to Windows' notifications, so nothing runs while nothing changes; a burst of changes calls it at most every 100 ms. Windows has no notification for levels, so `onLevels` has the app read them, and only while a widget listens: 25 times a second while there is sound and for two seconds after, so meters can fall, then four times a second without calling `fn` until there is sound again. Both only listen while the widget's page shows and the Edge isn't frozen; `onChange` then calls `fn` once when the widget shows again. Calls of `mixer()` less than 250 ms apart share one read, whichever widgets make them; a change made from here is in the next `mixer()` read. Windows has no public API to change the default output: `setOutput` uses the one Sound settings itself uses, which other volume tools rely on too.

### `api.fetch(input, init?)`

The web's `fetch`, with the same arguments and a standard `Response`, but sent by the app rather than the Edge page. Plain `fetch` works too, though only with APIs that let other sites call them (CORS); `api.fetch` reaches any `http` or `https` URL the way a server would:

```js
const response = await api.fetch('https://api.example.com/forecast?city=Paris');
const forecast = await response.json();
```

Headers that browsers forbid (`Cookie`, `Origin`, `Referer`, ...) can't be set, and every request carries the Edge page's `Origin`. Cookies that sites set are kept across restarts and shared by every widget, as in a browser. A request still unanswered when the widget unmounts is cancelled and its promise rejects; a body still being read then is not.

### `api.storage`

Data the widget keeps when it is unmounted (a setting change, a reload) and when the app restarts. Pick a scope for each value:

| Scope                  | Shared by                                                                                         |
| ---------------------- | ------------------------------------------------------------------------------------------------- |
| `api.storage.instance` | This placement only: two copies of the widget each have their own (a to-do list, a note).         |
| `api.storage.widget`   | Every placement of this widget (an account, a cache, a preference that should follow every copy). |

```js
const todos = (await api.storage.instance.get('todos')) ?? [];
// ... later, after an edit:
await api.storage.instance.set('todos', todos);
```

Both have the same members:

| Member            | What it does                                                                    |
| ----------------- | ------------------------------------------------------------------------------- |
| `get(key)`        | Promise of the value saved under `key`, or `undefined` when there is none.      |
| `set(key, value)` | Saves `value` under `key`, replacing the previous one. Values must fit in JSON. |
| `delete(key)`     | Removes `key`. Resolves `true` if it had a value.                               |

Nothing is deleted for you: data stays when the placement, its page or the widget is removed, so use `delete(key)` for what you no longer need. Every widget's data lives in one file, `%APPDATA%\com.edgestation.dashboard\widget-data.json`, rewritten whole on each change (at most every 100 ms), so keep it to small data.

### `api.webview(el, url, options?)`

Shows web page `url` in a native browser view laid over `el`, an element of your widget (usually its root). It is for sites that refuse to load in an iframe, like YouTube: an iframe is lighter, so use one when the site allows it. Only widgets with the `webview` permission can call it; any other gets its error card.

```js
mount(el, api) {
	el.textContent = 'YouTube';
	api.webview(el, 'https://www.youtube.com/feed/subscriptions');
}
```

The view is a separate webview drawn above the Edge, not part of the page, so:

- It only shows while `el` sits still and fully on screen. While pages slide, and while its page is off screen, it hides behind a picture of what it showed last, laid over `el` (an `<img>` the app adds to your widget's root, which it positions). Until it has shown once, there's no picture and your own content under it shows instead: put a placeholder there.
- It opens the first time `el` is on screen, so a view on a page nobody visits costs nothing. Then it stays hidden until its first page has loaded: meanwhile the card shows the loading dots of `api.loading` (see below), not a blank page. Each later page load (a link, the Home button) shows the loading bar along the top of the card, and the view leaves a 2px gap under the bar so it shows through.
- It follows `el` when the cell is resized or the page layout changes, but not when your widget moves `el` without resizing it (a CSS transform, an animation).
- It takes every touch and click on its area: the two fingers of a page swipe have to land outside it.
- Touching it leaves the keyboard with the app in front (see [Keyboard](#keyboard)). To type in it, the user presses the round keyboard button the app floats over its bottom right corner, which gives it the keyboard until they touch something else on the Edge. The view leaves a hole for the button, so it covers what the page shows there.
- It opens in the browser profile the user picked in the placement's settings, Default unless they picked another (Settings → Browser profiles creates them). Each profile keeps its own sign-ins, so two placements can use different accounts on the same site. A site you sign in to stays signed in across restarts. Changing the profile remounts the widget, like any setting.
- Links that open a new tab, and sign-in popups, open in a separate window in the same profile.
- It is a whole browser page with its own memory and CPU use. Pausing doesn't apply to it once open: hidden on another page, it keeps running (and playing audio).

It closes when the widget unmounts, so the next mount loads `url` again; `api.webview` also returns a function that closes it sooner. Only `http` and `https` URLs open; anything else shows the widget's error card.

Pass `{ mobile: true }` to ask for the site's mobile version, which fits narrow cells better: the view then presents itself as Chrome on a phone for every page it loads, user agent and client hints alike (Google's sites read the hints), so sites that pick their layout from the browser serve their mobile one. Popup windows it opens don't: they present the app's own browser. Sites that only follow the width already fit the cell either way.

```js
api.webview(el, 'https://en.wikipedia.org/', { mobile: true });
```

### `api.scrollbars(el)`

Swaps the native scrollbars of `el`, an element of your widget that scrolls, for the overlay ones the app uses: thin, in the theme's colors, and only shown while it scrolls. Scrolling itself (touch, wheel, keyboard) stays native.

They are drawn in `el`'s parent, so give `el` a wrapper of its own. The wrapper takes the place `el` had in your layout (size it as you would have sized `el`), and `el` fills it. Its `overflow`, `padding` and `margin` are set for you: put padding on an element inside it.

```js
mount(el, api) {
	el.innerHTML = '<div class="list"><ul>...</ul></div>';
	api.scrollbars(el.querySelector('ul'));
}
```

They are removed when the widget unmounts; `api.scrollbars` also returns a function that removes them sooner, for an element your widget takes away before then. In Svelte, an attachment does both: `<ul {@attach (ul) => api.scrollbars(ul)}>`.

### `api.loading(task)`

Shows on the card that the widget is loading until `task` settles, and returns `task`, so it can be awaited in place:

```js
const load = async () => (await api.fetch(url)).json();
const refresh = api.guard(async () => render(await api.loading(load())));
refresh();
api.every(60_000, refresh);
```

The first call after the widget mounts shows three dots in the middle of the card and hides the widget, which fades in when that load ends, failed or not. Each later call shows a thin animated bar along the top of the card, over the widget. Loads that overlap show one indicator until the last one settles, and both indicators wait 200 ms before they show, so a quick load shows nothing. Put the whole first load in one call (an async function that makes every request), or the widget shows between them. Web views report their page loads here on their own.

## Pausing

A widget is mounted the first time its page shows (a page nobody visits loads nothing), then stays mounted while the app runs, so it keeps its state when the user swipes away and back. While its page is off screen, it is paused: `api.every` timers stop and `api.after` timers wait. As soon as the page starts sliding back into view, each `every` callback runs right away, then on its interval again, and an `after` that came due meanwhile fires right away (one that didn't keeps its due time).

While another app is full screen (a game, an F11 browser, a full-screen video), the Edge freezes: `api.every` timers stop and CSS animations pause, so widgets neither poll nor redraw, and a Paused badge shows. Touching the Edge keeps it frozen. When the user switches to another app, or the app leaves full screen, each `every` callback runs right away, then on its interval again. Users can turn this off in Settings → General.

`api.after` timers keep running while frozen. Timers you start yourself (`setTimeout`, `setInterval`, `requestAnimationFrame`) keep running in both cases, and also outlive the widget unless you clear them in `api.onDispose`: use `api.every` for anything that repeats and `api.after` for a one-off delay.

## Keyboard

Touching the Edge leaves the keyboard with the app in front, so a game keeps running while the user taps a widget. A text field (`input`, `textarea`, editable content) takes the keyboard when the user touches it, and gives it back once they touch something else on the Edge. Focusing one from code (`focus()`) doesn't bring the keyboard until it is touched.

## Errors

Each widget runs inside its own error boundary. If `mount` throws, its promise rejects, or a callback wrapped with `api.guard` / `api.every` / `api.after` throws, only that widget is replaced by an error card with a Retry button. The rest of the display keeps running.

Errors in callbacks you register yourself (plain `addEventListener`, `setTimeout`) are not caught: wrap them with `api.guard`.

## Styling and theming

Each widget renders inside its own shadow root, so its CSS can't leak into the app (and the app's Tailwind classes don't apply inside). `el` has the class `widget`, fills the cell, and is a size container: use `cqw` / `cqh` units to scale with the cell.

The theme's CSS variables inherit into the widget, so use them to match the user's colors:

| Variable                                | Use                        |
| --------------------------------------- | -------------------------- |
| `--background`, `--foreground`          | Page colors                |
| `--card`, `--card-foreground`           | Widget background and text |
| `--primary`, `--primary-foreground`     | Accent color               |
| `--secondary`, `--secondary-foreground` | Secondary surfaces         |
| `--muted`, `--muted-foreground`         | Subtle surfaces, dim text  |
| `--accent`, `--destructive`             | Highlights, errors         |
| `--border`, `--ring`, `--radius`        | Borders, focus, corners    |
| `--chart-1` … `--chart-5`               | Data series                |

See `clock/` for a complete example, `performance/` for settings, `api.system()` and `api.topPrograms()`, `media/` for `api.media` and buttons, `todo/` for `api.storage`, text input and `api.scrollbars`, `logs-timeline/` for data that every copy of a widget shares and keeps in step, `launcher/` for `api.apps`, `mixer/` for `api.audio`, a `source` setting and faders dragged by touch, `youtube/` for `api.webview`, `url/` for a `text` setting and the mobile version, and `weather/` for `api.fetch`.

## Built-in widgets (Svelte)

The widgets in this folder are Svelte 5 components compiled to the module described above, built with `@edgestation/sdk` (`packages/sdk`) the way a widget of your own would be. Each is a pnpm workspace package (`@edgestation/widget-<id>`):

```
clock/
  manifest.json   ("main": "dist/main.js", no "styles")
  package.json    (build and dev scripts)
  src/
    main.ts
    Clock.svelte
  dist/main.js    (built, not committed)
```

As in an Obsidian plugin, there is no Svelte-specific API in the app: `main.ts` turns the component into the module with the SDK's Svelte adapter, which mounts it with the `api` as a prop and unmounts it with the widget. Components import their types from `@edgestation/sdk`.

```ts
import { svelteWidget } from '@edgestation/sdk/svelte';
import Clock from './Clock.svelte';

export default svelteWidget(Clock);
```

- A component reads `api` from its props once, at the top of its script. Svelte warns that this only captures the prop's first value (`state_referenced_locally`); that is what we want, since changing a setting remounts the widget, so that one line carries a `svelte-ignore`.
- Svelte redraws after the callback that changed its state has returned, so `api.guard` / `api.every` don't catch errors thrown while drawing: those don't show the error card.
- Styles go in the components' `<style>` blocks. Svelte adds them to the widget's shadow root when it mounts, so there is no `styles` file. Lay out inside a root element of your own (`el` is outside the component, and is the size container for `cqw` / `cqh`).
- `vite.config.ts` builds `src/main.ts` into one self-contained `dist/main.js`, with its own copy of the Svelte runtime, through the SDK's `widget()` Vite plugin. Svelte, Vite, the SDK and the type checker are root dev dependencies.

`pnpm tauri dev` rebuilds the widgets whenever their source changes, and `pnpm tauri build` builds them before bundling. To build one on its own:

```bash
pnpm --filter @edgestation/widget-clock build
```

`pnpm check` type-checks them with `tsconfig.json`.

## License

The built-in widgets are under the [MIT License](LICENSE), like the SDK, so you can start your own widget from one of them and publish it under any license. The app itself is under the [GPL-3.0](../LICENSE).
