<script lang="ts">
	import Plus from '@lucide/svelte/icons/plus';
	import { tick } from 'svelte';
	import { flip } from 'svelte/animate';
	import { cubicOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import { fly, scale } from 'svelte/transition';
	import SearchField from '#lib/components/settings/controls/SearchField.svelte';
	import WidgetIcon from '#lib/components/settings/widgets/WidgetIcon.svelte';
	import { Button } from '#lib/components/ui/button/index.ts';
	import * as Sheet from '#lib/components/ui/sheet/index.ts';
	import { widgetSize } from '#lib/layout/defaults.ts';
	import { newPlacement } from '#lib/layout/pages.ts';
	import type { Page } from '#lib/layout/types.ts';
	import { overlayScrollbars } from '#lib/scrollbars.ts';
	import { editWaiting } from '#lib/state/page-actions.ts';
	import { installed } from '#lib/widgets/installed.svelte.ts';
	import { filterInstalled } from '#lib/widgets/search.ts';
	import type { InstalledWidget } from '#lib/widgets/types.ts';

	let { page }: { page: Page } = $props();

	/** The view transition name of the row that flies into its new tile. The styles below use it. */
	const FLY = 'added-widget';

	let open = $state(false);
	let query = $state('');
	let content = $state<HTMLElement | null>(null);
	// After a touch on the button, the search field does not get the focus. Else the touch keybord
	// opens and covers the list.
	let touched = false;
	let openedAt = 0;

	const widgets = $derived(
		filterInstalled(
			installed.current.filter((widget) => !widget.error),
			query
		)
	);

	/** A duration in ms, or 0 when the system asks for reduced motion. */
	const ms = (duration: number) => (prefersReducedMotion.current ? 0 : duration);

	function show(event: MouseEvent) {
		touched = event instanceof PointerEvent && event.pointerType === 'touch';
		query = '';
		openedAt = performance.now();
		open = true;
	}

	// While the sheet opens, the rows come in from the right, one after another. Later, a row that
	// matches the search again only fades and grows in.
	function rowIn(node: Element, index: number) {
		if (performance.now() - openedAt > 300) return scale(node, { start: 0.95, duration: ms(150) });
		const delay = ms(Math.min(index * 30, 300));
		return fly(node, { x: 24, delay, duration: ms(250), easing: cubicOut });
	}

	// The new widget waits, and `editWaiting` saves it at once if it fits. If not, it stays red on
	// the grid until the user moves or resizes it onto free cells.
	function add(widget: InstalledWidget, row: HTMLElement) {
		const item = newPlacement(page, widget.id, widgetSize(widget));
		const place = () => {
			editWaiting(page.id, (items) => [...items, item]);
			open = false;
		};
		if (prefersReducedMotion.current || !('startViewTransition' in document)) return place();

		// The row flies into the new tile while the sheet fades out. A name can be on one element
		// only, so the row gives its name to the tile.
		const tile = () =>
			document.querySelector<HTMLElement>(`[data-placement="${CSS.escape(item.id)}"]`);
		row.style.viewTransitionName = FLY;
		const transition = document.startViewTransition({
			update: async () => {
				row.style.viewTransitionName = '';
				place();
				await tick();
				tile()?.style.setProperty('view-transition-name', FLY);
			},
			types: ['add-widget']
		});
		void transition.finished.then(() => tile()?.style.removeProperty('view-transition-name'));
	}
</script>

<Button onclick={show}><Plus />Add widget</Button>

<Sheet.Root bind:open>
	<Sheet.Content
		bind:ref={content}
		side="right"
		class="gap-0 outline-none"
		onOpenAutoFocus={(event) => {
			if (!touched) return;
			event.preventDefault();
			content?.focus({ preventScroll: true });
		}}
	>
		<!-- Space on the right for the close button. -->
		<Sheet.Header class="pr-12">
			<Sheet.Title>Add a widget</Sheet.Title>
			<Sheet.Description>
				Pick one to add it to {page.name}. The size is in columns × rows.
			</Sheet.Description>
		</Sheet.Header>
		<div class="px-4 pb-3">
			<SearchField bind:value={query} label="Search widgets" />
		</div>
		<!-- Holds the overlay scrollbars of the list, which fills it (see `overlayScrollbars`). -->
		<div class="min-h-0 flex-1">
			<div {@attach overlayScrollbars}>
				<!-- A row is 60 px high, so a finger can hit it on a touch screen. The intro is global,
				     so it also plays when the sheet opens. The outro stays local, so it plays only
				     when the search hides the row. -->
				{#each widgets as widget, index (widget.id)}
					{@const { w, h } = widgetSize(widget)}
					<button
						type="button"
						class="flex w-full items-center gap-3 px-4 py-2.5 text-left transition-colors outline-none hover:bg-muted/50 focus-visible:bg-muted/50 active:bg-muted"
						aria-label="Add {widget.name} ({w}×{h})"
						onclick={(event) => add(widget, event.currentTarget)}
						in:rowIn|global={index}
						out:scale={{ start: 0.95, duration: ms(150) }}
						animate:flip={{ duration: ms(200), easing: cubicOut }}
					>
						<WidgetIcon {widget} />
						<span class="grid min-w-0 flex-1">
							<span class="truncate font-medium">{widget.name}</span>
							<span class="truncate text-muted-foreground">{widget.description}</span>
						</span>
						<span class="shrink-0 text-xs text-muted-foreground tabular-nums">{w}×{h}</span>
					</button>
				{:else}
					<p class="px-4 py-3 text-muted-foreground">No widget matches your search.</p>
				{/each}
			</div>
		</div>
	</Sheet.Content>
</Sheet.Root>

<style>
	/* During the flight, only the whole window and the flying row are captured. If the editor and
	   its grid kept their own names, they would cover the sheet while it fades out. */
	:global(:root:active-view-transition-type(add-widget)) {
		view-transition-name: root;
	}
	:global(:root:active-view-transition-type(add-widget) main),
	:global(:root:active-view-transition-type(add-widget) [data-page]) {
		view-transition-name: none !important;
	}
	:global(::view-transition-group(added-widget)) {
		animation-duration: 250ms;
		animation-timing-function: cubic-bezier(0.2, 0, 0, 1);
	}
</style>
