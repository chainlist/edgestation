<script lang="ts">
	import { listedAsWidget } from '#lib/widgets/marketplace/listing.ts';
	import { marketplace } from '#lib/widgets/marketplace/state.svelte.ts';
	import type { ListedWidget } from '#lib/widgets/types.ts';
	import { downloadFacts, publisherFacts } from './facts.ts';
	import MarketplaceButtons from './MarketplaceButtons.svelte';
	import WidgetCard from './WidgetCard.svelte';

	let { listed }: { listed: ListedWidget } = $props();

	const widget = $derived(listedAsWidget(listed));
	const facts = $derived([
		...publisherFacts(widget, marketplace.listing),
		...downloadFacts(listed.downloads)
	]);
</script>

<!-- A marketplace widget in the Browse grid. The card opens the widget page. -->
<WidgetCard
	{widget}
	icon={listed.icon}
	href="/settings/widgets/browse/{listed.id}"
	{facts}
	problem={marketplace.errorOf(listed.id)}
>
	{#snippet actions()}
		<MarketplaceButtons id={listed.id} />
	{/snippet}
</WidgetCard>
