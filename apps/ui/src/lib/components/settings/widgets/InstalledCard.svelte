<script lang="ts">
	import { pagesShowing } from '#lib/state/settings.svelte.ts';
	import { marketplace } from '#lib/widgets/marketplace/state.svelte.ts';
	import type { InstalledWidget } from '#lib/widgets/types.ts';
	import { installedFacts } from './facts.ts';
	import MarketplaceButtons from './MarketplaceButtons.svelte';
	import PageBadges from './PageBadges.svelte';
	import WidgetCard from './WidgetCard.svelte';

	let { widget }: { widget: InstalledWidget } = $props();

	const fromMarketplace = $derived(widget.source === 'marketplace');
	const usedOn = $derived(pagesShowing(widget.id));
</script>

<!-- An installed widget in the Installed grid. Only a marketplace widget opens its page and has an
     Uninstall button, and an Update button when it has a newer release. -->
<WidgetCard
	{widget}
	href={fromMarketplace ? `/settings/widgets/browse/${widget.id}?from=installed` : undefined}
	facts={installedFacts(widget, marketplace.listing)}
	problem={fromMarketplace ? marketplace.problemOf(widget.id) : undefined}
	actions={fromMarketplace || usedOn.length ? pagesAndUninstall : undefined}
/>

{#snippet pagesAndUninstall()}
	<PageBadges pages={usedOn} />
	{#if fromMarketplace}
		<MarketplaceButtons id={widget.id} removable compact updatable />
	{/if}
{/snippet}
