A widget fills a grid cell whose size the user picks, on a display whose size EdgeStation doesn't know in advance, in colors the user picked. This page is about fitting all three.

## The widget's root

Each widget renders inside its own shadow root, so its CSS can't leak into the app, and the app's styles (its Tailwind classes included) don't apply inside it.

`el`, the element `mount` receives:

- has the class `widget`,
- fills the grid cell,
- is a size container: inside it, `cqw` and `cqh` units are percents of the cell's width and height.

Styles reach the shadow root in one of three ways:

- the `styles` file of the manifest (`dist/main.css` for CSS the SDK build collects),
- Svelte components compiled with `emitCss: false`, which add their own,
- `<style>` elements or inline styles your code creates in `el`.

## Size it to the cell

The cell can be one small square or the whole width of the Edge, and the Edge itself can be any display. Size text and spacing in `cqh` / `cqw` rather than pixels, and take the smaller of the two so content fits either way:

```css
.time {
	/* As large as the cell allows, whether it is tall or wide. */
	font-size: min(40cqh, 17cqw);
	white-space: nowrap;
}

.date {
	margin-top: 3cqh;
	font-size: min(11cqh, 7cqw);
}
```

That is how the built-in Clock scales: the same widget reads well in a 1 × 1 cell and in a 4 × 2 one.

Container units apply to the elements inside `el`, not to `el` itself, so put your layout in an element of your own inside it. With a framework that comes naturally: your component's root is that element.

## Theme colors

The theme's CSS variables inherit into the widget. Use them, rather than fixed colors, and the widget follows the user's dark or light mode, accent color and corner radius:

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

```css
.label {
	color: var(--muted-foreground);
}

.value {
	color: var(--primary);
}

button {
	border: 1px solid var(--border);
	border-radius: calc(var(--radius) - 4px);
	background: var(--secondary);
	color: var(--secondary-foreground);
}
```

The widget card itself, its background and corners, is drawn by the app. Leave the root transparent: the user may have picked a frosted glass look over a background image.

## Touch, mouse and keys

The Edge may be a touch screen, or a monitor used with a mouse. Make controls large enough for a finger, and don't rely on hover to show something.

The user turns pages with a two-finger swipe along the pages' axis (across, or up and down when the user stacks pages). One finger, the mouse and a pen always stay with your widget: taps, drags in any direction and scrolling are yours. As soon as a second finger touches the Edge, the page takes both touches, even on your widget: your widget gets no click from them, and `lostpointercapture` for a finger it was following. A web view from [`api.webview`](/docs/api/webview) is the exception: it takes every touch on its area, so the two fingers of a swipe have to land outside it.

## Scrollbars

A list that scrolls gets the browser's native scrollbars. [`api.scrollbars(el)`](/docs/api/helpers#api-scrollbars) swaps them for the app's overlay ones: thin, in the theme's colors, and only shown while scrolling.
