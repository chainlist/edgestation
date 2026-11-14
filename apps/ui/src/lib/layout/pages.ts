import { clampRect, findSpot, isFree } from './grid';
import type { Page, Placement, Rect } from './types';
import { newId, removeIn, updateIn } from '#lib/lists.ts';

export const createPage = (name: string): Page => ({
	id: newId(),
	name,
	cols: 8,
	rows: 2,
	items: []
});

/** Changes the grid size and moves the widgets back inside the grid. */
export const resizePage = (page: Page, cols: number, rows: number): Page => ({
	...page,
	cols,
	rows,
	items: page.items.map((item) => ({ ...item, ...clampRect({ cols, rows }, item) }))
});

/**
 * A new placement of `widget` at the spot that suits its size best (see `findSpot`). The spot can
 * cover other placements when the page has no free space for it. Then it must wait (see
 * `placeWaiting`).
 */
export const newPlacement = (
	page: Page,
	widget: string,
	size: { w: number; h: number }
): Placement => ({ id: newId(), widget, ...findSpot(page, page.items, size.w, size.h) });

/**
 * Saves on `page` each of the `waiting` placements that fits, in order. A placement fits if it
 * covers no cell of the placments on the page. The others keep waiting, moved inside the grid.
 * If none fits, the page is the same object.
 */
export function placeWaiting(page: Page, waiting: Placement[]) {
	const items = [...page.items];
	const left: Placement[] = [];
	for (const item of waiting) {
		const placed = { ...item, ...clampRect(page, item) };
		if (isFree(items, placed)) items.push(placed);
		else left.push(placed);
	}
	return { page: left.length < waiting.length ? { ...page, items } : page, waiting: left };
}

/** Moves placements by id to new spots, for example after a swap. Keeps them inside the grid. */
export const movePlacements = (page: Page, rects: Record<string, Rect>): Page => ({
	...page,
	items: page.items.map((item): Placement => {
		const rect = rects[item.id];
		return rect ? { ...item, ...clampRect(page, rect) } : item;
	})
});

export const setPlacementSetting = (page: Page, id: string, key: string, value: string): Page => ({
	...page,
	items: updateIn(page.items, id, (item) => ({
		...item,
		settings: { ...item.settings, [key]: value }
	}))
});

/** Sets the profile for the web views of the placement. Use `undefined` for Default. */
export const setPlacementProfile = (page: Page, id: string, profile: string | undefined): Page => ({
	...page,
	items: updateIn(page.items, id, (item) => ({ ...item, profile }))
});

/**
 * Finds the placements of `widgets` (web view widgets) that use profile `profile`, or Default for
 * `undefined`. Each comes with its page. The order is page order, then top to bottom and left to
 * rigth.
 */
export const profileUsers = (pages: Page[], profile: string | undefined, widgets: Set<string>) =>
	pages.flatMap((page) =>
		page.items
			.filter((item) => item.profile === profile && widgets.has(item.widget))
			.sort((a, b) => a.y - b.y || a.x - b.x)
			.map((item) => ({ page, item }))
	);

export const leaveProfile = (pages: Page[], profile: string): Page[] =>
	pages.map((page) => ({
		...page,
		items: page.items.map((item) =>
			item.profile === profile ? { ...item, profile: undefined } : item
		)
	}));

export const removePlacement = (page: Page, id: string): Page => ({
	...page,
	items: removeIn(page.items, id)
});

/** Removes case and accents, so that "ete" finds "Été". */
const fold = (text: string) => text.normalize('NFD').replace(/\p{M}/gu, '').toLowerCase();

/**
 * Finds the indexes of the pages whose name holds `query`, without regard to case and accents.
 * First come the names where the query starts the name or a word, then the other names, in page
 * order. An empty query finds all pages.
 */
export function findPages(pages: Page[], query: string): number[] {
	const q = fold(query.trim());
	const starts: number[] = [];
	const inside: number[] = [];
	pages.forEach((page, i) => {
		const name = fold(page.name);
		if (name.startsWith(q) || name.split(/[^\p{L}\p{N}]+/u).some((word) => word.startsWith(q))) {
			starts.push(i);
		} else if (name.includes(q)) {
			inside.push(i);
		}
	});
	return [...starts, ...inside];
}
