`api.apps` reaches the apps of the Start menu's All apps list, Store apps included, the way the Start menu lists, starts and draws them. On Linux, it reaches the apps the app menu shows, their desktop entries, through GIO as the menus of GNOME and KDE do, with icons from the desktop's icon theme. It needs `"permissions": ["apps"]` in the [manifest](/docs/manifest#permissions).

```js
const apps = await api.apps.list();
const terminal = apps.find((app) => app.name === 'Terminal');
img.src = api.apps.icon(terminal.id, 96);
button.addEventListener(
	'click',
	api.guard(() => api.apps.launch(terminal.id))
);
```

## Members

| Member            | What it does                                                                                                                                                                                                                                                             |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `list()`          | Promise of every app, by name: `{ id, name }`. Read when called (a few hundred ms on Windows), so call it when you show the list, not on a timer.                                                                                                                        |
| `launch(id)`      | Starts the app with its default action. On Windows, its window is asked for on the main screen rather than the Edge (apps that remember their position may still open where they were). On Linux, the window manager places it. Rejects when the app is no longer there. |
| `icon(id, size?)` | URL of the app's icon, a PNG `size` pixels square (16 to 256, default 64), for an `<img>`. The image fails to load when the app is gone.                                                                                                                                 |

## App ids

On Windows, `id` is the app's name in the shell's Apps folder: its AppUserModelID, or a path for a program without one. On Linux, it is the id of its desktop entry, such as `firefox.desktop`. It stays the same across restarts, so save it with [`api.storage`](/docs/api/storage) to remember an app the user picked:

```js
const pinned = (await api.storage.instance.get('apps')) ?? [];
```

Like the Start menu or the app menu, `launch` starts the app even when it is already open, so most apps open a new window.

## Sharp icons

Ask `icon()` for the size the image shows at in device pixels, not CSS pixels: on a display scaled to 150 %, a 64 px icon needs 96.

```js
const size = Math.round(64 * devicePixelRatio);
img.src = api.apps.icon(app.id, Math.min(256, size));
```

The [`launcher` widget](https://github.com/chainlist/edgestation/tree/main/widgets/launcher) is a complete example: keys that open apps, like a Stream Deck.
