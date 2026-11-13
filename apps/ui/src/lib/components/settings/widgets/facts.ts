import Download from '@lucide/svelte/icons/download';
import Folder from '@lucide/svelte/icons/folder';
import Grid2x2 from '@lucide/svelte/icons/grid-2x2';
import SlidersHorizontal from '@lucide/svelte/icons/sliders-horizontal';
import { DEFAULT_WIDGET_SIZE } from '#lib/layout/defaults.ts';
import { downloadsLabel, publisherFacts, type Fact } from '#lib/widgets/marketplace/listing.ts';
import { usesWebview } from '#lib/widgets/permissions.ts';
import type { InstalledWidget, ListedWidget } from '#lib/widgets/types.ts';

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
	const { w, h } = widget.size ?? DEFAULT_WIDGET_SIZE;
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
