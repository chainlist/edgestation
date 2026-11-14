<script lang="ts">
	import EdgeBackground from '#lib/components/edge/EdgeBackground.svelte';
	import { cellStyle, gridStyle } from '#lib/layout/grid.ts';
	import type { Page } from '#lib/layout/types.ts';
	import { edgeScreen } from '#lib/state/edge-screen.svelte.ts';
	import { widgetGlassStyle } from '#lib/theme/background.ts';

	let {
		image,
		blur,
		opacity,
		widgetBlur,
		radius,
		page
	}: {
		image: string;
		/** The blur of the image, from 0 to 1. */
		blur: number;
		/** The opacity of the widget cards and the blur behind them, from 0 to 1. */
		opacity: number;
		widgetBlur: number;
		/** The corner radius of the theme, in rem. */
		radius: number;
		/** The page who's widgets show as empty cards. */
		page?: Page;
	} = $props();

	/**
	 * Converts a length in CSS pixels on the Edge to `cqmin` of this preview. The length is the
	 * same part of the shorter side as on the Edge window, for all displays and scales.
	 */
	const edge = (px: number) => `${(px / edgeScreen.shortSide) * 100}cqmin`;

	const glass = $derived(widgetGlassStyle(opacity, widgetBlur));
</script>

<!-- A small copy of the Edge, drawn like the real one (see the Edge page, GridPage and WidgetSlot).
     It has the shape of the Edge and fits in 14rem × 6rem, wide or tall. It is a size container,
     so blurs and spacing scale with it. -->
<div
	class="[container-type:size] relative overflow-hidden rounded-md border bg-background shadow-sm"
	style="aspect-ratio:{edgeScreen.aspect};width:min(14rem, 6rem * {edgeScreen.aspect});{glass}"
>
	<EdgeBackground {image} {blur} />
	{#if page}
		<div
			class="relative grid size-full"
			style="{gridStyle(page)};gap:{edge(12)};padding:{edge(12)}"
		>
			{#each page.items as item (item.id)}
				<div
					class="border bg-card/[var(--widget-opacity,100%)] [backdrop-filter:var(--widget-backdrop,none)]"
					style="{cellStyle(item)};border-radius:{edge(radius * 16 * 1.4)}"
				></div>
			{/each}
		</div>
	{/if}
</div>
