<script lang="ts">
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import type { Snippet } from 'svelte';
	import { Badge } from '#lib/components/ui/badge/index.ts';
	import { pagesShowing } from '#lib/state/settings.svelte.ts';
	import type { Fact } from '#lib/widgets/marketplace/listing.ts';
	import { marketplace } from '#lib/widgets/marketplace/state.svelte.ts';
	import type { InstalledWidget } from '#lib/widgets/types.ts';
	import { installedFacts } from './facts.ts';
	import FactLine from './FactLine.svelte';
	import PageBadges from './PageBadges.svelte';
	import WidgetFacts from './WidgetFacts.svelte';
	import WidgetIcon from './WidgetIcon.svelte';
	import WidgetPermissions from './WidgetPermissions.svelte';

	let {
		widget,
		icon,
		href,
		more = [],
		problem,
		actions
	}: {
		widget: InstalledWidget;
		/** The icon URL of a marketplace widget that is not installed (see WidgetIcon). */
		icon?: string | null;
		/** The page that the row opens. */
		href?: string;
		/** Facts that show after the others, such as the download count. */
		more?: Fact[];
		/** Shows in red below the description, for example why an install failed. */
		problem?: string;
		/** Buttons at the end of the row. */
		actions?: Snippet;
	} = $props();

	const facts = $derived(installedFacts(widget, marketplace.listing, more));
</script>

<!-- One widget on a row of a card. With `href`, the name streches over the row, so the whole row is a
     link. Controls and elements with a tooltip are `relative`, so that they stay above it. -->
<div
	class={[
		'relative flex items-center gap-4 px-4 py-3.5',
		href && 'transition-colors hover:bg-muted/50'
	]}
>
	<WidgetIcon {widget} src={icon} />
	<div class="grid min-w-0 flex-1 gap-1">
		<p class="flex items-baseline gap-2">
			{#if href}
				<a
					{href}
					class="truncate font-medium outline-none after:absolute after:inset-0 focus-visible:after:ring-3 focus-visible:after:ring-ring/50 focus-visible:after:ring-inset"
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
		{#if widget.error}
			<p class="break-words text-destructive">{widget.error}</p>
		{:else if widget.description}
			<p class="line-clamp-2 text-muted-foreground">{widget.description}</p>
		{/if}
		{#if problem}
			<p class="text-destructive">{problem}</p>
		{/if}
		<FactLine class="mt-0.5">
			<WidgetFacts {facts} />
			<WidgetPermissions {widget} />
		</FactLine>
	</div>
	<PageBadges pages={pagesShowing(widget.id)} class="max-w-44 shrink-0" />
	{#if actions}
		<div class="relative flex shrink-0 items-center gap-2">{@render actions()}</div>
	{/if}
	{#if href}
		<ChevronRight class="size-4 shrink-0 text-muted-foreground" aria-hidden="true" />
	{/if}
</div>
