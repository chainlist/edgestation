<script lang="ts">
	import { Badge } from '#lib/components/ui/badge/index.ts';
	import type { InstalledWidget } from '#lib/widgets/types.ts';

	let {
		widget,
		href
	}: {
		widget: InstalledWidget;
		/** With it, the name is a link that stretches over its card (the nearest `relative`). */
		href?: string;
	} = $props();
</script>

<!-- The name of a widget, its version and a Local badge, for WidgetCard and WidgetRow. -->
<p class="flex items-baseline gap-2">
	{#if href}
		<a
			{href}
			class="truncate font-medium outline-none after:absolute after:inset-0 after:rounded-xl focus-visible:after:ring-3 focus-visible:after:ring-ring/50"
		>
			{widget.name}
		</a>
	{:else}
		<span class="truncate font-medium">{widget.name}</span>
	{/if}
	{#if widget.version}
		<span class="text-xs text-muted-foreground tabular-nums">{widget.version}</span>
	{/if}
	{#if widget.source === 'local'}
		<Badge variant="secondary" title="From your widgets folder">Local</Badge>
	{/if}
</p>
