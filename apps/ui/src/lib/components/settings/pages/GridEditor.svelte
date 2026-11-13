<script lang="ts">
	import { gridPitch, trackCellDrag } from '#lib/layout/drag.ts';
	import { dragRect, isFree, swapRects, type DragMode } from '#lib/layout/grid.ts';
	import { movePlacements, removeIn, removePlacement, updateIn } from '#lib/layout/pages.ts';
	import type { Page, Placement, Rect } from '#lib/layout/types.ts';
	import { editPage, editWaiting } from '#lib/state/page-actions.ts';
	import { waiting } from '#lib/state/waiting.svelte.ts';
	import { installed } from '#lib/widgets/installed.svelte.ts';
	import { hasSettings } from '#lib/widgets/settings.ts';
	import GridEditorItem from './GridEditorItem.svelte';
	import GridPreview from './GridPreview.svelte';

	let {
		page,
		selected,
		onselect,
		ondragstart
	}: {
		page: Page;
		selected?: string;
		/** The user clicked a widget or its settings button, which shows only if it has settings. */
		onselect: (id: string) => void;
		/** A move or a resize of a widget started. */
		ondragstart: () => void;
	} = $props();

	// The saved widgets, then the widgets that wait for space (see `waiting`). These come last, so
	// they show above the widgets that they cover. A widget keeps its element when it gets saved.
	const items = $derived([...page.items, ...(waiting[page.id] ?? [])]);
	const isWaiting = (item: Placement) => !page.items.some((saved) => saved.id === item.id);

	const configurable = (item: Placement) =>
		hasSettings(
			installed.current.find((w) => w.id === item.widget),
			item.settings
		);

	let grid = $state<HTMLElement>();
	// The new position of the dragged widget, and the new position of the widgets that it swaps
	// with. `moves` is null if the position is not usable. Then the release saves nothing.
	let draft = $state<{ id: string; rect: Rect; moves: Record<string, Rect> | null } | null>(null);

	/**
	 * New positions if `item` goes to `rect`. A move swaps widgets, a resize needs free cells. A
	 * waiting widget needs free cells for both, so it never pushes a saved widget away.
	 */
	function plan(item: Placement, mode: DragMode, rect: Rect) {
		if (mode === 'move' && !isWaiting(item)) return swapRects(page, page.items, item.id, rect);
		const others = page.items.filter((other) => other.id !== item.id);
		return isFree(others, rect) ? { [item.id]: rect } : null;
	}

	/** A waiting widget stays where the user drops it, also where it does not fit. */
	function drop(item: Placement, rect: Rect, moves: Record<string, Rect> | null) {
		if (isWaiting(item)) {
			editWaiting(page.id, (list) => updateIn(list, item.id, (w) => ({ ...w, ...rect })));
		} else if (moves) {
			editPage(page.id, (p) => movePlacements(p, moves));
		}
	}

	function remove(item: Placement) {
		if (isWaiting(item)) editWaiting(page.id, (list) => removeIn(list, item.id));
		else editPage(page.id, (p) => removePlacement(p, item.id));
	}

	function drag(event: PointerEvent, item: Placement, mode: DragMode) {
		if (!grid || event.button !== 0) return;
		trackCellDrag(
			event,
			gridPitch(grid, page),
			(dx, dy) => {
				if (!draft) ondragstart();
				const rect = dragRect(page, item, mode, dx, dy);
				draft = { id: item.id, rect, moves: plan(item, mode, rect) };
			},
			(completed) => {
				if (completed && draft) drop(item, draft.rect, draft.moves);
				// A release before a move of one cell is a click. It selects the widget. A waiting
				// widget has no settings until it is saved.
				if (completed && !draft && !isWaiting(item)) onselect(item.id);
				draft = null;
			}
		);
	}
</script>

<GridPreview {page} bind:ref={grid}>
	{#each items as item (item.id)}
		{@const dragging = draft?.id === item.id}
		<GridEditorItem
			{item}
			rect={draft?.moves?.[item.id] ?? (dragging && draft ? draft.rect : item)}
			invalid={dragging ? !draft?.moves : isWaiting(item)}
			{dragging}
			selected={item.id === selected}
			ondrag={(event, mode) => drag(event, item, mode)}
			onremove={() => remove(item)}
			onsettings={configurable(item) && !isWaiting(item) ? () => onselect(item.id) : undefined}
		/>
	{/each}
</GridPreview>
