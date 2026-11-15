<script lang="ts">
	import Store from '@lucide/svelte/icons/store';
	import EmptyState from '#lib/components/settings/EmptyState.svelte';
	import PageHeader from '#lib/components/settings/PageHeader.svelte';
	import BrowseWidgets from '#lib/components/settings/widgets/BrowseWidgets.svelte';
	import MarketplaceSwitch from '#lib/components/settings/widgets/MarketplaceSwitch.svelte';
	import * as Card from '#lib/components/ui/card/index.ts';
	import { marketplaceOn } from '#lib/state/settings.svelte.ts';
	import { loadListing } from '#lib/widgets/marketplace/state.svelte.ts';

	// Turning the marketplace on loads the list too (see `switchMarketplace`).
	if (marketplaceOn.current) void loadListing();
</script>

<PageHeader title="Marketplace" description="Widgets that other people make and publish." />

<div class="grid gap-6">
	<MarketplaceSwitch />
	{#if marketplaceOn.current}
		<BrowseWidgets />
	{:else}
		<Card.Root class="gap-0 py-0">
			<EmptyState
				icon={Store}
				title="The marketplace is off"
				text="Turn it on to browse and install widgets that other people make."
			/>
		</Card.Root>
	{/if}
</div>
