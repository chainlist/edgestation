<script lang="ts">
	import type { Snippet } from 'svelte';
	import { pagesShowing } from '#lib/state/settings.svelte.ts';
	import { marketplace } from '#lib/widgets/marketplace/state.svelte.ts';
	import type { InstalledWidget } from '#lib/widgets/types.ts';
	import { installedFacts, type Fact } from './facts.ts';
	import FactLine from './FactLine.svelte';
	import PageBadges from './PageBadges.svelte';
	import WidgetDescription from './WidgetDescription.svelte';
	import WidgetFacts from './WidgetFacts.svelte';
	import WidgetIcon from './WidgetIcon.svelte';
	import WidgetName from './WidgetName.svelte';
	import WidgetPermissions from './WidgetPermissions.svelte';

	let {
		widget,
		icon,
		more = [],
		problem,
		actions
	}: {
		widget: InstalledWidget;
		/** The icon URL of a marketplace widget that is not installed (see WidgetIcon). */
		icon?: string | null;
		/** Facts that show after the others, such as the download count. */
		more?: Fact[];
		/** Shows in red below the description, for example why an install failed. */
		problem?: string;
		/** Buttons at the end of the row. */
		actions?: Snippet;
	} = $props();

	const facts = $derived(installedFacts(widget, marketplace.listing, more));
</script>

<!-- One widget on a row of a card. -->
<div class="relative flex items-center gap-4 px-4 py-3.5">
	<WidgetIcon {widget} src={icon} />
	<div class="grid min-w-0 flex-1 gap-1">
		<WidgetName {widget} />
		<WidgetDescription {widget} {problem} lines={2} />
		<FactLine class="mt-0.5">
			<WidgetFacts {facts} />
			<WidgetPermissions {widget} />
		</FactLine>
	</div>
	<PageBadges pages={pagesShowing(widget.id)} class="max-w-44 shrink-0" />
	{#if actions}
		<div class="relative flex shrink-0 items-center gap-2">{@render actions()}</div>
	{/if}
</div>
