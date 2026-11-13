import type { Component } from 'svelte';
import { matchesQuery } from '../search';
import type { InstalledWidget, ListedWidget } from '../types';
import { isNewer, versionProblem } from '../version';

/** The sort orders in Settings > Widgets > Browse. */
export type ListingSort = 'downloads' | 'released' | 'name';

export const listedName = (widget: ListedWidget) => widget.manifest?.name ?? widget.id;

/**
 * Shows a marketplace widget as a widget row. The row uses the manifest of its latest release,
 * or tells why the host cannot read it.
 */
export const listedAsWidget = (widget: ListedWidget): InstalledWidget => ({
	name: widget.id,
	...widget.manifest,
	id: widget.id,
	source: 'marketplace',
	error: widget.manifest ? undefined : (widget.error ?? undefined)
});

/** The trusted GitHub accounts, in lower case becuase GitHub names ignore case. */
const VERIFIED_OWNERS = new Set(['chainlist']);

/**
 * The GitHub account that owns the repo of marketplace widget `widget` in `listing`, or
 * `undefined`. Only that account and its collaborators can publish releases there.
 */
export function repoOwner(
	widget: Pick<InstalledWidget, 'id' | 'source'>,
	listing: ListedWidget[] | undefined
): string | undefined {
	if (widget.source !== 'marketplace') return undefined;
	return listing?.find((listed) => listed.id === widget.id)?.repo.split('/')[0];
}

/**
 * Tells if GitHub account `owner` is trusted. The `author` of a manifest does not count, because
 * any widget can set it.
 */
export const verifiedOwner = (owner: string) => VERIFIED_OWNERS.has(owner.toLowerCase());

/** A fact in the line below a widget description. `verified` adds the badge. */
export interface Fact {
	text: string;
	title?: string;
	verified?: boolean;
	mono?: boolean;
	/** A Lucide icon before the text. */
	icon?: Component<{ class?: string }>;
}

/**
 * Who publishes `widget`: its author and the GitHub account that owns its repo. The author of a
 * built-in widget is verified, because the widget ships with the app.
 */
export function publisherFacts(
	widget: Pick<InstalledWidget, 'id' | 'source' | 'author'>,
	listing: ListedWidget[] | undefined
): Fact[] {
	const owner = repoOwner(widget, listing);
	const facts: Fact[] = [];
	if (widget.author) facts.push({ text: widget.author, verified: widget.source === 'builtin' });
	if (owner) {
		const title = 'Owner of its GitHub repository';
		facts.push({ text: `@${owner}`, title, verified: verifiedOwner(owner) });
	}
	return facts;
}

export const filterListing = (widgets: ListedWidget[], query: string) =>
	widgets.filter((widget) => {
		const { name, description, author } = widget.manifest ?? {};
		return matchesQuery(query, [widget.id, name, description, author]);
	});

/**
 * Sorts `widgets` with the most downloads or the latest release first, or by name. Widgets wich
 * the stats do not have yet come after the others. Ties go by name.
 */
export function sortListing(widgets: ListedWidget[], by: ListingSort): ListedWidget[] {
	const key = (widget: ListedWidget) => {
		if (by === 'downloads') return widget.downloads;
		if (by === 'released' && widget.released) return Date.parse(widget.released);
		return null;
	};
	return [...widgets].sort((a, b) => {
		const [ka, kb] = [key(a), key(b)];
		if (ka !== null && kb !== null && ka !== kb) return kb - ka;
		if ((ka === null) !== (kb === null)) return ka === null ? 1 : -1;
		return listedName(a).localeCompare(listedName(b));
	});
}

/** One page of a list. `index` is 0-based. `first` and `last` are 1-based item positions. */
export interface Page<T> {
	items: T[];
	index: number;
	count: number;
	first: number;
	last: number;
}

/**
 * Returns page `index` (0-based) of `items`, with `size` items per page. An index after the last
 * page gives the last page. Then a list that gets shorter, for example after a search, still
 * shows items.
 */
export function paginate<T>(items: T[], index: number, size: number): Page<T> {
	const count = Math.max(1, Math.ceil(items.length / size));
	const page = Math.min(Math.max(0, index), count - 1);
	const start = page * size;
	const pageItems = items.slice(start, start + size);
	return { items: pageItems, index: page, count, first: start + 1, last: start + pageItems.length };
}

/** `1234` → "1.2K downloads". */
export function downloadsLabel(count: number): string {
	const number = new Intl.NumberFormat('en', { notation: 'compact' }).format(count);
	return `${number} ${count === 1 ? 'download' : 'downloads'}`;
}

/** What the Install button of a marketplace widget can do. */
export type ListingStatus =
	| { kind: 'install' }
	| { kind: 'update'; version: string }
	| { kind: 'installed' }
	| { kind: 'unavailable'; reason: string };

/**
 * Tells wether marketplace widget `widget` can be installed or updated. It uses the `installed`
 * widgets and the app version (`undefined` until it is known).
 */
export function listingStatus(
	widget: ListedWidget,
	installed: InstalledWidget[],
	appVersion: string | undefined
): ListingStatus {
	const same = installed.find((w) => w.id === widget.id);
	if (same?.source === 'local') {
		return { kind: 'unavailable', reason: 'One of your own widgets uses this id.' };
	}
	const mine = same?.source === 'marketplace' ? same : undefined;
	const latest = widget.manifest;
	if (!latest) {
		return mine
			? { kind: 'installed' }
			: { kind: 'unavailable', reason: "Its latest release can't be read." };
	}

	const { version, minAppVersion: min } = latest;
	const tooNew = min !== undefined && appVersion !== undefined && versionProblem(appVersion, min);
	if (mine) {
		return version !== undefined && isNewer(version, mine.version ?? '') && !tooNew
			? { kind: 'update', version }
			: { kind: 'installed' };
	}
	return tooNew
		? { kind: 'unavailable', reason: `Needs EdgeStation ${min} or later.` }
		: { kind: 'install' };
}
