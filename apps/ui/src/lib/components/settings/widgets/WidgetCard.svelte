<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Badge } from '#lib/components/ui/badge/index.ts';
	import type { Fact } from '#lib/widgets/marketplace/listing.ts';
	import { shownPermissions } from '#lib/widgets/permissions.ts';
	import type { InstalledWidget } from '#lib/widgets/types.ts';
	import FactLine from './FactLine.svelte';
	import WidgetFacts from './WidgetFacts.svelte';
	import WidgetIcon from './WidgetIcon.svelte';
	import WidgetPermissions from './WidgetPermissions.svelte';

	let {
		widget,
		icon,
		href,
		facts,
		problem,
		actions
	}: {
		widget: InstalledWidget;
		/** The icon URL of a marketplace widget that is not installed (see WidgetIcon). */
		icon?: string | null;
		/** The page that the card opens. */
		href?: string;
		/** The facts below the name. */
		facts: Fact[];
		/** Shows in red below the description, for example why an install failed. */
		problem?: string;
		/** Links and buttons at the bottom right. */
		actions?: Snippet;
	} = $props();

	/** A card with no permissions to show and no actions has no bottom line. */
	const footer = $derived(!!actions || shownPermissions(widget).length > 0);
</script>

<!-- One widget in a CardGrid. With `href`, the name stretches over the card, so the card opens the
     page. Controls and elements with a tooltip are `relative` to stay above it. -->
<article
	class={[
		'relative flex flex-col gap-3 rounded-xl bg-card p-4 text-sm text-card-foreground shadow-xs ring-1 ring-foreground/10',
		href && 'transition-colors hover:bg-muted/50'
	]}
>
	<div class="flex items-start gap-3">
		<WidgetIcon {widget} src={icon} />
		<div class="grid min-w-0 flex-1 gap-1">
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
			<FactLine>
				<WidgetFacts {facts} />
			</FactLine>
		</div>
	</div>
	{#if widget.error}
		<p class="break-words text-destructive">{widget.error}</p>
	{:else if widget.description}
		<p class="line-clamp-3 text-muted-foreground">{widget.description}</p>
	{/if}
	{#if problem}
		<p class="text-destructive">{problem}</p>
	{/if}
	{#if footer}
		<div class="mt-auto flex min-h-8 flex-wrap items-center justify-between gap-2">
			<FactLine>
				<WidgetPermissions {widget} />
			</FactLine>
			{#if actions}
				<div class="relative ml-auto flex items-center gap-2">{@render actions()}</div>
			{/if}
		</div>
	{/if}
</article>
