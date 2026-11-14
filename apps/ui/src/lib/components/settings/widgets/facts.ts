import Download from '@lucide/svelte/icons/download';
import Folder from '@lucide/svelte/icons/folder';
import Grid2x2 from '@lucide/svelte/icons/grid-2x2';
import SlidersHorizontal from '@lucide/svelte/icons/sliders-horizontal';
import type { Component } from 'svelte';
import { widgetSize } from '#lib/layout/defaults.ts';
import { repoOwner, verifiedOwner } from '#lib/widgets/marketplace/listing.ts';
import { usesWebview } from '#lib/widgets/permissions.ts';
import type { InstalledWidget, ListedWidget } from '#lib/widgets/types.ts';

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

/** `1234` → "1.2K downloads". */
export function downloadsLabel(count: number): string {
	const number = new Intl.NumberFormat('en', { notation: 'compact' }).format(count);
	return `${number} ${count === 1 ? 'download' : 'downloads'}`;
}

/** The download count of a marketplace widget, or no fact while the stats do not have it. */
export const downloadFacts = (downloads: number | null | undefined): Fact[] =>
	downloads == null ? [] : [{ text: downloadsLabel(downloads), icon: Download }];

/**
 * The publisher, the default size, the number of settings, `more` and the folder name. Web view
 * widgets also get a Profile setting, which counts as one.
 */
export function installedFacts(
	widget: InstalledWidget,
	listing: ListedWidget[] | undefined,
	more: Fact[] = []
): Fact[] {
	const folder = { text: widget.id, title: 'Folder name', mono: true, icon: Folder };
	if (widget.error) return [folder];
	const { w, h } = widgetSize(widget);
	const settings = (widget.settings?.length ?? 0) + (usesWebview(widget) ? 1 : 0);
	return [
		...publisherFacts(widget, listing),
		{ text: `${w} × ${h} cells`, icon: Grid2x2 },
		...(settings > 0
			? [
					{
						text: `${settings} ${settings === 1 ? 'setting' : 'settings'}`,
						icon: SlidersHorizontal
					}
				]
			: []),
		...more,
		folder
	];
}
