These members tie timers, callbacks and elements to the widget's lifetime, so they stop with it, pause when nobody sees it, and report their errors on its error card.

## `api.every()`

```ts
every(ms: number, fn: () => unknown): () => void
```

A `setInterval` that EdgeStation manages:

- It stops while the widget's page is off screen, and while the Edge is frozen because another app is full screen. When it resumes, it calls `fn` right away, then every `ms` again.
- Errors thrown in `fn`, or a promise it returns that rejects, show the widget's error card.
- It is cleared when the widget unmounts. It returns a function that stops it sooner.

Like `setInterval`, the first call comes when `ms` has passed. To draw right away, call your function once yourself:

```js
const update = async () => {
	const { cpu } = await api.system();
	el.textContent = `${Math.round(cpu.load)} %`;
};

api.guard(update)();
api.every(1000, update);
```

Use it for anything that repeats. See [Lifecycle and errors](/docs/lifecycle#timers-at-a-glance) for how it compares with the browser's timers.

## `api.after()`

```ts
after(ms: number, fn: () => unknown): () => void
```

A `setTimeout` that EdgeStation manages:

- It waits while the widget's page is off screen. If it came due meanwhile, it fires as soon as the page is back; if not, it keeps its due time.
- Unlike `every`, it still fires while the Edge is frozen.
- Its errors show the error card, and it is cancelled if the widget unmounts first. It returns a function that cancels it.

```js
// Hide a confirmation after 3 seconds.
message.hidden = false;
api.after(3000, () => (message.hidden = true));
```

## `api.guard()`

```ts
guard<A extends unknown[]>(fn: (...args: A) => unknown): (...args: A) => void
```

Wraps a callback so an error thrown in it, or a promise it returns that rejects, shows the widget's error card. Use it for callbacks you register yourself, such as event listeners, whose errors EdgeStation can't otherwise see:

```js
button.addEventListener(
	'click',
	api.guard(async () => {
		await api.apps.launch(appId);
	})
);
```

Once the widget has unmounted, the wrapped function does nothing, and a promise that rejects after that is ignored.

## `api.loading()`

```ts
loading<T>(task: Promise<T>): Promise<T>
```

Shows on the widget's card that it is loading until `task` settles. It returns `task` itself, so you can await it in place:

```js
const load = async () => (await api.fetch(url)).json();

const refresh = api.guard(async () => {
	render(await api.loading(load()));
});

refresh();
api.every(60_000, refresh);
```

- The first load, the first call after the widget mounts, shows three dots in the middle of the card and hides the widget. The widget fades in when that load ends, whether it succeeded or failed, so draw your error message as usual.
- Each later load shows a thin animated bar along the top of the card. The widget stays as it is under it.
- Loads that overlap show one indicator, until the last of them settles.
- Both indicators wait 200 ms before they show, so a quick load shows nothing.
- The promise it returns rejects like `task`: catch the error where you await it.

Put the whole first load in one call. If it takes two requests, wrap an async function that makes both, like `load` above; otherwise the widget shows between the two.

[Web views](/docs/api/webview) report their page loads here on their own: you don't wrap them.

## `api.onDispose()`

```ts
onDispose(fn: () => void): void
```

Runs `fn` when the widget unmounts. Use it to stop what you started outside the `api`: your own timers, listeners on `window` or `document`, sockets, observers.

```js
const observer = new ResizeObserver(redraw);
observer.observe(el);
api.onDispose(() => observer.disconnect());
```

Returning a cleanup function from `mount` does the same.

## `api.asset()`

```ts
asset(path: string): string
```

The URL of a file in the widget folder, for images, JSON and other files the widget reads. `path` is relative to the folder.

```js
img.src = api.asset('images/logo.png');
const data = await (await fetch(api.asset('data/cities.json'))).json();
```

For a widget built with the SDK, the path is still relative to the widget folder, not to `src`. Keep such files out of `dist`, which each build empties, or put them in Vite's `public` folder, which the build copies into `dist`: their path then starts with `dist/`. Ship them with the widget.

Relative ES module imports (`import './chart.js'`) don't need `asset()`.

## `api.scrollbars()`

```ts
scrollbars(el: HTMLElement): () => void
```

Swaps the native scrollbars of `el`, an element of your widget that scrolls, for the overlay ones the app uses: thin, in the theme's colors, and only shown while it scrolls. Scrolling itself (touch, wheel, keyboard) stays native.

They are drawn in `el`'s parent, so give `el` a wrapper of its own. The wrapper takes the place `el` had in your layout (size it as you would have sized `el`), and `el` fills it. Its `overflow`, `padding` and `margin` are set for you: put padding on an element inside it.

```js
mount(el, api) {
	el.innerHTML = '<div class="list"><ul>...</ul></div>';
	api.scrollbars(el.querySelector('ul'));
}
```

They are removed when the widget unmounts; `api.scrollbars` also returns a function that removes them sooner, for an element your widget takes away before then. In Svelte, an attachment does both:

```svelte
<div class="list">
	<ul {@attach (ul) => api.scrollbars(ul)}>...</ul>
</div>
```
