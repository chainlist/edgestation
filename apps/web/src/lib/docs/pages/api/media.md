`api.media` gives the media sessions on the machine, and the one the system considers current. On Windows, it is the one the volume flyout shows and the media keys control. On Linux, it is one of the MPRIS players on the session bus, which the media keys and the desktop's media controls also reach: the first one that plays, else the one playerctld follows, else a paused one. A stopped player doesn't count. Spotify, browsers (YouTube, ...) and most players publish one. It needs `"permissions": ["media"]` in the [manifest](/docs/manifest#permissions).

Read it once, then again when the system says it changed, so nothing polls:

```js
const show = api.guard(async () => {
	const now = await api.media.current();
	el.textContent = now ? `${now.title} · ${now.artist}` : 'Nothing playing';
});
show();
api.media.onChange(show);
```

## Members

| Member                 | What it does                                                                                                                                                  |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `current()`            | Promise of the current session below, or `null` when nothing is playing.                                                                                      |
| `sessions()`           | Promise of every session, in the system's order. Empty when nothing is playing. See [several sessions](#several-sessions).                                    |
| `art(id?)`             | Promise of the album art of session `id` from the latest `current()` or `sessions()` call, as a data URL (`null` without art). Without `id`, the current one. |
| `control(action, id?)` | Asks the app of session `id`, or of the current one, to `'play'`, `'pause'`, skip to `'next'` or go to `'previous'`. Resolves `false` if it declined.         |
| `onChange(fn)`         | Calls `fn` when something changes: see [Changes](#changes). Returns a function that stops it.                                                                 |

## Changes

`onChange(fn)` calls `fn` when a session opens, closes or becomes current, or when its track, playback state or timeline changes. The system tells EdgeStation, so nothing polls in between.

- `fn` gets no arguments. It doesn't say what changed or in which session: read `current()` or `sessions()` in it.
- A burst of changes (a new track often sends several) calls `fn` at most every 100 ms.
- It listens only while the widget's page shows and the Edge isn't frozen. When the widget shows again, it calls `fn` once, whether or not something changed meanwhile.
- The position of a playing track moves on without a call: neither Windows nor MPRIS sends it. Advance it yourself from the last read, by `rate` seconds per second; `api.every` asks the system nothing:

```js
let read = null;
let readAt = 0;
const refresh = api.guard(async () => {
	read = await api.media.current();
	readAt = performance.now();
});
refresh();
api.media.onChange(refresh);
api.every(1000, () => {
	if (!read || read.position === null) return;
	const played = read.playing ? ((performance.now() - readAt) / 1000) * read.rate : 0;
	bar.value = Math.min(read.position + played, read.duration) / read.duration;
});
```

## The session

`current()` resolves to a `MediaInfo`, and `sessions()` to an array of them:

| Field                      | Meaning                                                                                                                                                            |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `id`                       | Names the session in `art()` and `control()`. On Windows, the app id, with `#2`, `#3`, ... when one app has more sessions. On Linux, the player's name on the bus. |
| `current`                  | `true` for the session the media keys control.                                                                                                                     |
| `title`, `artist`, `album` | As the app reports them; empty when it doesn't.                                                                                                                    |
| `app`                      | Id of the app that owns the session (`Spotify.exe`, `MSEdge`, ...). On Linux, the player's desktop entry (`spotify`, `firefox`, ...), else its name on the bus.    |
| `playing`                  | `true` while playing, `false` when paused or stopped.                                                                                                              |
| `rate`                     | Playback speed, `1` at normal speed: while playing, `position` moves on by `rate` seconds per second.                                                              |
| `position`, `duration`     | Seconds, `position` as of the call. `null` when the app reports no timeline (see below).                                                                           |
| `art`                      | A number that changes when the art does, or `null`. Call `art()` only when it changes: the image is large.                                                         |
| `can`                      | `{ play, pause, next, previous }`: which actions the app accepts right now.                                                                                        |

## Album art

The art is a large image, so `current()` only says when it changed. Fetch it then:

```js
let shown = null;

const showArt = api.guard(async () => {
	const now = await api.media.current();
	const art = now?.art ?? null;
	if (art === shown) return;
	shown = art;
	const url = art === null ? null : await api.media.art();
	if (url) img.src = url;
	else img.removeAttribute('src');
});
showArt();
api.media.onChange(showArt);
```

## Controls

```js
playButton.addEventListener(
	'click',
	api.guard(async () => {
		const now = await api.media.current();
		if (now) await api.media.control(now.playing ? 'pause' : 'play');
	})
);
```

Disable the buttons whose action `can` says the app doesn't accept right now.

## Several sessions

Spotify and a browser can both have a session at the same time. `sessions()` lists them all, with `current` set on the one `current()` would give. Pass a session's `id` to `art()` and `control()` to act on that one instead of the current one:

```js
let sessions = [];
let shownId = null;
const shown = () =>
	sessions.find((s) => s.id === shownId) ?? sessions.find((s) => s.current) ?? null;

const list = api.guard(async () => {
	sessions = await api.media.sessions();
	el.textContent = shown()?.title ?? 'Nothing playing';
});
list();
api.media.onChange(list);

nextSessionButton.addEventListener('click', () => {
	const next = sessions[(sessions.indexOf(shown()) + 1) % sessions.length];
	shownId = next?.id ?? null;
});

pauseButton.addEventListener(
	'click',
	api.guard(async () => {
		const now = shown();
		if (now) await api.media.control('pause', now.id);
	})
);
```

On Windows, apps built on WebView2 share the app id `msedgewebview2.exe`, hence the `#2`. Ids stay the same from one call to the next, but when the first of two sessions of one app closes, the second one's `#2` goes away.

## Timeline quirks

Some apps report the timeline only now and then: on Windows, Zen and Firefox report it only on play, pause and seek, and blank it out about once a minute. While a widget listens with `onChange`, or widgets keep reading a session with `current()` or `sessions()` at least every 5 seconds, its position carries on from the last one reported. After a longer gap it stays `null` until the next play, pause or seek, rather than guessing across a pause or seek the widget didn't see.

The [`media` widget](https://github.com/chainlist/edgestation/tree/main/widgets/media) is a complete example, with art, progress, controls and a switch between sessions.
