<script lang="ts">
	import Settings from '@lucide/svelte/icons/settings';
	import X from '@lucide/svelte/icons/x';
	import type { Corner, DragMode } from '#lib/layout/grid.ts';
	import type { Placement, Rect } from '#lib/layout/types.ts';
	import PreviewItem from './PreviewItem.svelte';

	let {
		item,
		rect,
		invalid,
		dragging,
		selected,
		ondrag,
		onremove,
		onsettings
	}: {
		item: Placement;
		rect: Rect;
		invalid: boolean;
		dragging: boolean;
		selected: boolean;
		ondrag: (event: PointerEvent, mode: DragMode) => void;
		onremove: () => void;
		/** Opens the widget settings. Not set if the widget has none, and then no button shows. */
		onsettings?: () => void;
	} = $props();

	// For each corner: the name for screen readers, the handle position and cursor, and the
	// bracket in the handle.
	const handles: [Corner, string, string, string][] = [
		['nw', 'top left', 'top-0 left-0 cursor-nw-resize', 'top-1 left-1 border-t-2 border-l-2'],
		['ne', 'top right', 'top-0 right-0 cursor-ne-resize', 'top-1 right-1 border-t-2 border-r-2'],
		[
			'sw',
			'bottom left',
			'bottom-0 left-0 cursor-sw-resize',
			'bottom-1 left-1 border-b-2 border-l-2'
		],
		[
			'se',
			'bottom right',
			'right-0 bottom-0 cursor-se-resize',
			'right-1 bottom-1 border-r-2 border-b-2'
		]
	];
</script>

<PreviewItem
	widget={item.widget}
	{rect}
	{invalid}
	lifted={dragging}
	class={['group cursor-grab touch-none active:cursor-grabbing', selected && 'ring-2 ring-primary']}
	data-placement={item.id}
	onpointerdown={(event) => ondrag(event, 'move')}
>
	{#if onsettings}
		<button
			class="absolute top-1 right-9 rounded-sm p-0.5 opacity-0 group-hover:opacity-100 hover:bg-accent focus-visible:opacity-100"
			aria-label="{item.widget} settings"
			onpointerdown={(event) => event.stopPropagation()}
			onclick={onsettings}
		>
			<Settings class="size-3.5" />
		</button>
	{/if}
	<button
		class="absolute top-1 right-4 rounded-sm p-0.5 opacity-0 group-hover:opacity-100 hover:bg-destructive/20 hover:text-destructive"
		aria-label="Remove {item.widget}"
		onpointerdown={(event) => event.stopPropagation()}
		onclick={onremove}
	>
		<X class="size-3.5" />
	</button>
	{#each handles as [corner, label, position, bracket] (corner)}
		<button
			class="absolute size-4 {position}"
			aria-label="Resize {item.widget} from the {label}"
			tabindex="-1"
			onpointerdown={(event) => {
				event.stopPropagation();
				ondrag(event, corner);
			}}
		>
			<span class="absolute size-2 border-primary/70 {bracket}"></span>
		</button>
	{/each}
</PreviewItem>
