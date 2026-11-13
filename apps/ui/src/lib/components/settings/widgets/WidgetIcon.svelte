<script lang="ts">
	import Puzzle from '@lucide/svelte/icons/puzzle';
	import TriangleAlert from '@lucide/svelte/icons/triangle-alert';
	import type { InstalledWidget } from '#lib/widgets/types.ts';
	import { widgetUrl } from '#lib/widgets/url.ts';

	let {
		widget,
		src
	}: {
		widget: InstalledWidget;
		/** The icon URL for a widget that is not installed. Else the icon comes from its folder. */
		src?: string | null;
	} = $props();

	const icon = $derived(src !== undefined ? src : widget.icon && widgetUrl(widget.id, widget.icon));
</script>

<!-- The widget icon on an accent tile. The image is a mask, so all icons get the accent color. A
     widget with no icon shows a puzzle peice, and a broken manifest shows a warning. -->
<span
	class={[
		'flex size-10 shrink-0 items-center justify-center rounded-lg',
		widget.error ? 'bg-destructive/10 text-destructive' : 'bg-primary/10 text-primary'
	]}
	aria-hidden="true"
>
	{#if widget.error}
		<TriangleAlert class="size-5" />
	{:else if icon}
		<span class="size-5 bg-current" style:mask="url({icon}) center / contain no-repeat"></span>
	{:else}
		<Puzzle class="size-5" />
	{/if}
</span>
