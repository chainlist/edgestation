import { createPage, placeWaiting } from '#lib/layout/pages.ts';
import { moveIn, removeIn, updateIn } from '#lib/lists.ts';
import type { Page, Placement } from '#lib/layout/types.ts';
import { pages } from './settings.svelte';
import { waiting } from './waiting.svelte';

/**
 * Changes page `id` and the widgets that wait for space on it. The waiting widgets that fit go on
 * the page. An unchanged page is not saved, because each save goes to the Edge window.
 */
function edit(id: string, fn: (page: Page) => Page, wait: (items: Placement[]) => Placement[]) {
	const page = pages.current.find((p) => p.id === id);
	if (!page) return;
	const next = placeWaiting(fn(page), wait(waiting[id] ?? []));
	waiting[id] = next.waiting;
	if (next.page !== page) pages.current = updateIn(pages.current, id, () => next.page);
}

export const editPage = (id: string, fn: (page: Page) => Page) => edit(id, fn, (items) => items);

export const editWaiting = (id: string, fn: (items: Placement[]) => Placement[]) =>
	edit(id, (page) => page, fn);

export function addPage(): string {
	const page = createPage(`Page ${pages.current.length + 1}`);
	pages.current = [...pages.current, page];
	return page.id;
}

export function removePage(id: string) {
	pages.current = removeIn(pages.current, id);
}

export function movePage(id: string, delta: number) {
	pages.current = moveIn(pages.current, id, delta);
}
