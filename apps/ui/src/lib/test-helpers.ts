import type { InstalledWidget, ListedWidget } from './widgets/types';

/** Lets the waiting promise callbacks run before a test checks the result. */
export const flush = () => new Promise((resolve) => setTimeout(resolve));

/** The ids of `list`, to compare lists in a test. */
export const ids = (list: { id: string }[]) => list.map((item) => item.id);

/** A marketplace widget whose release manifest names it after `id`. */
export const listedWidget = (id: string, more: Partial<ListedWidget> = {}): ListedWidget => ({
	id,
	repo: `someone/${id}`,
	downloads: null,
	released: null,
	manifest: { name: id[0].toUpperCase() + id.slice(1), version: '1.0.0' },
	error: null,
	icon: null,
	readme: '',
	...more
});

/** An installed marketplace widget. */
export const installedWidget = (
	id: string,
	more: Partial<InstalledWidget> = {}
): InstalledWidget => ({
	id,
	name: id,
	version: '1.0.0',
	source: 'marketplace',
	...more
});
