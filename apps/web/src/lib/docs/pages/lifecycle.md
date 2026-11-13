A widget is mounted the first time its page shows, so a page nobody visits loads nothing. It then stays mounted while the app runs, so it keeps its state when the user swipes away and back. EdgeStation pauses it when nobody can see it, and contains it when it fails.

## Mount

EdgeStation imports the module named by the manifest's `main` and calls its default export's `mount(el, api)`:

```js
export default {
	async mount(el, api) {
		const items = (await api.storage.instance.get('items')) ?? [];
		el.textContent = `${items.length} items`;
		return () => {
			// optional cleanup, run on unmount
		};
	}
};
```

- `mount` may be async. It may return a cleanup function, or resolve to one.
- Relative imports work: `import './chart.js'` loads `chart.js` from the widget folder.
- Files in the folder other than modules, such as images or JSON, are reached through [`api.asset(path)`](/docs/api/helpers#api-asset).

## Unmount

A widget is unmounted when:

- the user removes it from its page, or removes the page,
- the user changes one of its [settings](/docs/settings): it is then mounted again, with the new values,
- the Edge reloads (Settings → Widgets → Reload Edge).

On unmount, EdgeStation runs the cleanup `mount` returned and the functions passed to [`api.onDispose`](/docs/api/helpers#api-ondispose), and releases everything the widget registered through the `api`: `every` and `after` timers, web views, scrollbars, and `fetch` requests still waiting for an answer. Your own `setTimeout`, `setInterval`, `requestAnimationFrame`, event listeners on `window` or `document`, and sockets are yours to stop.

Data kept in variables is lost on unmount. Keep what should survive a settings change or a restart in [`api.storage`](/docs/api/storage).

## Pausing off screen

While a widget's page is off screen, the widget is paused:

- `api.every` timers stop.
- `api.after` timers wait.

As soon as the page starts sliding back into view, each `every` callback runs right away, then on its interval again, and an `after` that came due meanwhile fires right away (one that didn't keeps its due time).

## Freezing during full screen

While another app is full screen (a game, a browser in F11, a full-screen video), the Edge freezes: `api.every` timers stop and CSS animations pause, so widgets neither poll nor redraw, and a Paused badge shows on the Edge. Touching the Edge keeps it frozen.

When the user switches to another app, or the app leaves full screen, each `every` callback runs right away, then on its interval again. Users can turn freezing off in Settings → General.

## Timers at a glance

| Timer                                                | Page off screen                        | Edge frozen                      | Widget unmounts |
| ---------------------------------------------------- | -------------------------------------- | -------------------------------- | --------------- |
| `api.every`                                          | Stops, runs right away when back       | Stops, runs right away when back | Cleared         |
| `api.after`                                          | Waits, fires right away if it came due | Keeps running                    | Cleared         |
| `setTimeout`, `setInterval`, `requestAnimationFrame` | Keeps running                          | Keeps running                    | Keeps running   |

Use `api.every` for anything that repeats and `api.after` for a one-off delay. A timer of your own also outlives the widget unless you clear it in `api.onDispose`. The [Vite plugin](/docs/build#the-vite-plugin) warns at build time about each file of yours that calls `setTimeout` or `setInterval`.

## Errors

Each widget runs inside its own error boundary. If `mount` throws, its promise rejects, or a callback wrapped with `api.guard`, `api.every` or `api.after` throws, only that widget is replaced by an error card with the message and a Retry button. The rest of the Edge keeps running.

Errors in callbacks you register yourself, such as a plain `addEventListener` or `setTimeout`, are not caught: wrap them with [`api.guard`](/docs/api/helpers#api-guard).

```js
button.addEventListener(
	'click',
	api.guard(async () => {
		await api.media.control('next');
	})
);
```

The framework adapters send some errors to the error card too: see [Svelte, React and Vue](/docs/frameworks).

A widget whose manifest asks for a newer EdgeStation (`minAppVersion`), or that calls an `api` member without its [permission](/docs/manifest#permissions), shows the error card as well, with the reason.
