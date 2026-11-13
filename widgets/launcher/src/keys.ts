import type { AppInfo } from '@edgestation/sdk';

/** The app on each key, row by row from the top left. `null` is an empty key. */
export type Keys = (AppInfo | null)[];

/** Returns `keys` with `app` on key `index` (`null` empties it), with no empty keys at the end. */
export function setKey(keys: Keys, index: number, app: AppInfo | null): Keys {
	const next = Array.from({ length: Math.max(keys.length, index + 1) }, (_, i) => keys[i] ?? null);
	next[index] = app && { id: app.id, name: app.name };
	while (next.length && !next.at(-1)) next.pop();
	return next;
}
