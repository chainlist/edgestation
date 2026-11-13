`api.fetch` is the web's `fetch`, with the same arguments and a standard `Response`, but sent by the app rather than the Edge page. It needs `"permissions": ["fetch"]` in the [manifest](/docs/manifest#permissions).

```ts
fetch(input: RequestInfo | URL, init?: RequestInit): Promise<Response>
```

```js
const response = await api.fetch('https://api.example.com/forecast?city=Paris');
const forecast = await response.json();
```

## Why not plain `fetch`

Plain `fetch` works in a widget too, but only with APIs that let other sites call them (CORS). `api.fetch` reaches any `http` or `https` URL the way a server would, so APIs that don't send CORS headers answer as well.

Use plain `fetch` when the API allows it, and `api.fetch` when it doesn't.

## What to know

- Headers that browsers forbid (`Cookie`, `Origin`, `Referer`, ...) can't be set, and every request carries the Edge page's `Origin`.
- Cookies that sites set are kept across restarts and shared by every widget, as in a browser.
- A request still unanswered when the widget unmounts is cancelled, and its promise rejects. A body still being read then is not.

## Polling an API

Pair it with `api.every`, which stops while the page is off screen, so an API is only called while someone can see the result. Check `response.ok`: a `fetch` resolves on HTTP errors too. Here the city comes from a `text` [setting](/docs/settings) with the key `city`:

```js
async function update() {
	const city = encodeURIComponent(api.settings.city);
	const response = await api.fetch(`https://api.example.com/forecast?city=${city}`);
	if (!response.ok) throw new Error(`Forecast: HTTP ${response.status}`);
	const { temperature } = await response.json();
	el.textContent = `${Math.round(temperature)}°`;
}

api.guard(update)(); // now
api.every(10 * 60_000, update); // then every 10 minutes
```

`api.every` first runs its callback when the interval has passed, like `setInterval`, so the first update is called directly, wrapped in `api.guard`. An error thrown in either shows the widget's error card, with a Retry button. To keep the last value on screen instead, catch it.

The [`weather` widget](https://github.com/chainlist/edgestation/tree/main/widgets/weather) is a complete example: it looks its city up once and keeps it with `api.storage`, then fetches the forecast from Open-Meteo while it shows, keeping the last one through failures.

## API keys

A key you put in a widget is readable by anyone who has the widget's files. For a key that belongs to the user, add a `text` [setting](/docs/settings) and read it from `api.settings`, rather than shipping yours.
