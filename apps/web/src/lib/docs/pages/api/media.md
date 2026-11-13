`api.media` is the media session the system considers current. On Windows, it is the one the volume flyout shows and the media keys control. On Linux, it is one of the MPRIS players on the session bus, which the media keys and the desktop's media controls also reach: the first one that plays, else the one playerctld follows, else a paused one. A stopped player doesn't count. Spotify, browsers (YouTube, ...) and most players publish one. It needs `"permissions": ["media"]` in the [manifest](/docs/manifest#permissions).

Like `api.system()`, it is read when called and nothing listens in between, so poll it:

```js
api.every(1000, async () => {
	const now = await api.media.current();
	el.textContent = now ? `${now.title} · ${now.artist}` : 'Nothing playing';
});
```

## Members

| Member            | What it does                                                                                                  |
| ----------------- | ------------------------------------------------------------------------------------------------------------- |
| `current()`       | Promise of the session below, or `null` when nothing is playing.                                              |
| `art()`           | Promise of the album art from the latest `current()` call, as a data URL (`null` without art).                |
| `control(action)` | Asks the app to `'play'`, `'pause'`, skip to `'next'` or go to `'previous'`. Resolves `false` if it declined. |

## The session

`current()` resolves to a `MediaInfo`:

| Field                      | Meaning                                                                                                                                                         |
| -------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`, `artist`, `album` | As the app reports them; empty when it doesn't.                                                                                                                 |
| `app`                      | Id of the app that owns the session (`Spotify.exe`, `MSEdge`, ...). On Linux, the player's desktop entry (`spotify`, `firefox`, ...), else its name on the bus. |
| `playing`                  | `true` while playing, `false` when paused or stopped.                                                                                                           |
| `position`, `duration`     | Seconds, `position` as of the call. `null` when the app reports no timeline (see below).                                                                        |
| `art`                      | A number that changes when the art does, or `null`. Call `art()` only when it changes: the image is large.                                                      |
| `can`                      | `{ play, pause, next, previous }`: which actions the app accepts right now.                                                                                     |

## Album art

The art is a large image, so `current()` only says when it changed. Fetch it then:

```js
let shown = null;

api.every(1000, async () => {
	const now = await api.media.current();
	const art = now?.art ?? null;
	if (art === shown) return;
	shown = art;
	const url = art === null ? null : await api.media.art();
	if (url) img.src = url;
	else img.removeAttribute('src');
});
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

## Timeline quirks

Some apps report the timeline only now and then: on Windows, Zen and Firefox report it only on play, pause and seek, and blank it out about once a minute. While widgets keep calling `current()` at least every 5 seconds, the position carries on from the last one reported. After a longer gap it stays `null` until the next play, pause or seek, rather than guessing across a pause or seek the widget didn't see.

The [`media` widget](https://github.com/chainlist/edgestation/tree/main/widgets/media) is a complete example, with art, progress and controls.
