import { clamp } from '#lib/clamp.ts';

// On Windows (WebView2), custom protocols are at http://<scheme>.localhost.
const WINDOWS = navigator.userAgent.includes('Windows');
const base = (scheme: string) => (WINDOWS ? `http://${scheme}.localhost` : `${scheme}://localhost`);

/** URL of `path` in the widget folder `id`. The Rust `widget` protocol serves it. */
export const widgetUrl = (id: string, path: string) =>
	`${base('widget')}/${[id, ...path.split('/')].map(encodeURIComponent).join('/')}`;

/** Limits `size` to the pixel sizes that the Rust `appicon` protocal draws. */
const iconSize = (size: number) => Math.round(clamp(size || 64, 16, 256));

/** URL of the icon of app `id`, `size` pixels square. The Rust `appicon` protocol serves it. */
export const appIconUrl = (id: string, size: number) =>
	`${base('appicon')}/${iconSize(size)}/${encodeURIComponent(id)}`;

/** URL of the icon of the executable at `path`, like `appIconUrl`. */
export const fileIconUrl = (path: string, size: number) =>
	`${base('appicon')}/file/${iconSize(size)}/${encodeURIComponent(path)}`;
