<script lang="ts">
	import type { InstalledWidget } from '#lib/widgets/types.ts';

	/** Static class names, so that Tailwind finds them. */
	const CLAMP = { 2: 'line-clamp-2', 3: 'line-clamp-3' };

	let {
		widget,
		problem,
		lines
	}: {
		widget: InstalledWidget;
		/** Shows in red below the description, for example why an install failed. */
		problem?: string;
		/** The most lines that the description takes. */
		lines: keyof typeof CLAMP;
	} = $props();
</script>

<!-- The error of a widget, else its description, then a problem, for WidgetCard and WidgetRow. -->
{#if widget.error}
	<p class="break-words text-destructive">{widget.error}</p>
{:else if widget.description}
	<p class={['text-muted-foreground', CLAMP[lines]]}>{widget.description}</p>
{/if}
{#if problem}
	<p class="text-destructive">{problem}</p>
{/if}
