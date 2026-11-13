`api.webview` shows a web page in a native browser view laid over an element of your widget. It is for sites that refuse to load in an iframe, like YouTube. An iframe is lighter, so use one when the site allows it.

```ts
webview(el: HTMLElement, url: string, options?: WebviewOptions): () => void
```

It needs `"permissions": ["webview"]` in the [manifest](/docs/manifest#permissions); without it, the widget gets its error card.

```json manifest.json
{
	"name": "YouTube",
	"permissions": ["webview"]
}
```

```js main.js
export default {
	mount(el, api) {
		el.textContent = 'YouTube';
		api.webview(el, 'https://www.youtube.com/feed/subscriptions');
	}
};
```

Only `http` and `https` URLs open; anything else shows the widget's error card. The view closes when the widget unmounts, so the next mount loads `url` again; `api.webview` also returns a function that closes it sooner.

## Options

| Option       | Meaning                                                                                                                  |
| ------------ | ------------------------------------------------------------------------------------------------------------------------ |
| `mobile`     | `true` asks for the site's mobile version, which fits narrow cells better. See [below](#mobile-version).                 |
| `homeButton` | `true` shows a round Home button above the keyboard button while the view is away from `url`, which takes it back there. |

```js
api.webview(el, 'https://en.wikipedia.org/', { mobile: true, homeButton: true });
```

The Home button is an element the app adds to the widget's root, and the view leaves a hole for it. Where `url` redirects to as it loads counts as `url`, so a site that sends you elsewhere on arrival doesn't show the button.

## How it shows

The view is a separate webview drawn above the Edge, not part of the page, so:

- It only shows while `el` sits still and fully on screen. While pages slide, and while its page is off screen, it hides behind a picture of what it showed last, laid over `el`: an `<img>` the app adds to your widget's root, and positions. Until it has shown once, there's no picture and your own content under it shows instead, so put a placeholder there.
- It opens the first time `el` is on screen, so a view on a page nobody visits costs nothing. Then it stays hidden until its first page has loaded: meanwhile the card shows the loading dots of [`api.loading`](/docs/api/helpers#api-loading), not a blank page. Each later page load (a link, the Home button) shows the loading bar along the top of the card, and the view leaves a 2px gap under the bar so it shows through.
- It follows `el` when the cell is resized or the page layout changes, but not when your widget moves `el` without resizing it (a CSS transform, an animation).
- It takes every touch and click on its area: the two fingers of a page swipe have to land outside it.
- Touching it leaves the keyboard with the app in front. To type in it, the user presses the round keyboard button the app floats over its bottom right corner, which gives it the keyboard until they touch something else on the Edge. The view leaves a hole for the button, so it covers what the page shows there. While the page shows something full screen, such as a video, the keyboard and Home buttons hide.
- It is a whole browser page with its own memory and CPU use. Pausing doesn't apply to it once open: hidden on another page, it keeps running, and playing audio.

## Profiles and sign-ins

The view opens in the browser profile the user picked in the placement's settings: Default, unless they picked another (Settings → Browser profiles creates them). Each profile keeps its own sign-ins and cookies, so two placements can use different accounts on the same site. A site you sign in to stays signed in across restarts.

Changing the profile remounts the widget, like any setting.

Links that open a new tab, and sign-in popups, open in a separate window in the same profile.

## Mobile version

With `{ mobile: true }`, the view presents itself as Chrome on a phone for every page it loads, user agent and client hints alike (Google's sites read the hints), so sites that pick their layout from the browser serve their mobile one. On Linux, WebKitGTK has no client hints, so only the user agent changes. Popup windows it opens don't: they present the app's own browser. Sites that only follow the width already fit the cell either way.

The [`url` widget](https://github.com/chainlist/edgestation/tree/main/widgets/url) lets the user choose between the two with a setting, and shows the Home button. The [`youtube` widget](https://github.com/chainlist/edgestation/tree/main/widgets/youtube) opens one fixed page.
