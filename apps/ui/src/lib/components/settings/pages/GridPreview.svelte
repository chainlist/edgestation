<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cellStyle, gridStyle } from '#lib/layout/grid.ts';
	import type { Page } from '#lib/layout/types.ts';
	import { edgeScreen } from '#lib/state/edge-screen.svelte.ts';

	let {
		page,
		children,
		ref = $bindable()
	}: { page: Pick<Page, 'id' | 'cols' | 'rows'>; children: Snippet; ref?: HTMLElement } = $props();

	const cells = $derived(page.cols * page.rows);
</script>

<!-- A small Edge page with the same grid styles as the real page, in the shape of its screen. A
     tall screen stays below 60% of the window heigth. The view transition name comes from the
     page, so the settings transitions change a page card into its editor and back. -->
<div
	bind:this={ref}
	data-page={page.id}
	class="mx-auto grid gap-1 rounded-lg border bg-background p-1"
	style="{gridStyle(page)};aspect-ratio:{edgeScreen.aspect};width:min(100%, 60vh * {edgeScreen.aspect})"
	style:view-transition-name="page-{page.id}"
	style:view-transition-class="page-grid"
>
	{#each { length: cells }, i (i)}
		<div
			class="rounded-sm bg-muted/50"
			style={cellStyle({ x: i % page.cols, y: Math.floor(i / page.cols), w: 1, h: 1 })}
		></div>
	{/each}
	{@render children()}
</div>
