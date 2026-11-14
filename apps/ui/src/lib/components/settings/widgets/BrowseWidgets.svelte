<script lang="ts">
	import ChevronLeft from '@lucide/svelte/icons/chevron-left';
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import LoaderCircle from '@lucide/svelte/icons/loader-circle';
	import { Button } from '#lib/components/ui/button/index.ts';
	import * as Card from '#lib/components/ui/card/index.ts';
	import { browseSort } from '#lib/state/settings.svelte.ts';
	import { paginate, sortListing, type ListingSort } from '#lib/widgets/marketplace/listing.ts';
	import { loadListing, marketplace } from '#lib/widgets/marketplace/state.svelte.ts';
	import { filterListing } from '#lib/widgets/search.ts';
	import ChoiceSelect from '#lib/components/settings/controls/ChoiceSelect.svelte';
	import RefreshButton from '#lib/components/settings/controls/RefreshButton.svelte';
	import SearchField from '#lib/components/settings/controls/SearchField.svelte';
	import BrowseCard from './BrowseCard.svelte';
	import CardGrid from './CardGrid.svelte';

	const SORTS: { value: ListingSort; label: string }[] = [
		{ value: 'downloads', label: 'Most downloaded' },
		{ value: 'released', label: 'Recently updated' },
		{ value: 'name', label: 'Name' }
	];
	let query = $state('');
	const shown = $derived(
		sortListing(filterListing(marketplace.listing ?? [], query), browseSort.current)
	);

	/** Browse shows 20 widgets at a time. A new search or sort goes back to the first page. */
	const PAGE_SIZE = 20;
	let pageIndex = $state(0);
	const browsePage = $derived(paginate(shown, pageIndex, PAGE_SIZE));
	let browseTop = $state<HTMLElement>();

	function turnPage(by: number) {
		pageIndex = browsePage.index + by;
		// The buttons are below the list, so scroll the start of the new page into veiw.
		browseTop?.scrollIntoView({ block: 'nearest' });
	}

	// The list loads one time for each Settings window. Refresh shows the widgets listed since then.
	let refreshing = $state(false);
	async function refresh() {
		refreshing = true;
		await loadListing(true);
		refreshing = false;
	}
</script>

<div class="grid gap-3">
	<div class="flex flex-wrap gap-2" bind:this={browseTop}>
		<SearchField
			bind:value={query}
			oninput={() => (pageIndex = 0)}
			label="Search the marketplace"
			class="min-w-56 flex-1"
		/>
		<ChoiceSelect
			choices={SORTS}
			value={browseSort.current}
			class="w-48"
			onchange={(value) => {
				browseSort.current = value as ListingSort;
				pageIndex = 0;
			}}
		/>
		<RefreshButton label="Refresh the list" busy={refreshing} onclick={refresh} />
	</div>

	<!-- A failed refresh keeps the previous list. -->
	{#if marketplace.listing && marketplace.error}
		<p class="px-1 text-sm text-destructive">Couldn't refresh the list. {marketplace.error}</p>
	{/if}

	{#if !marketplace.listing}
		<Card.Root class="items-center gap-3 px-6 py-10 text-center">
			{#if marketplace.error}
				<p class="text-destructive">{marketplace.error}</p>
				<Button variant="outline" size="sm" onclick={() => loadListing(true)}>Try again</Button>
			{:else}
				<p class="flex items-center gap-2 text-muted-foreground">
					<LoaderCircle class="size-4 animate-spin" />Looking for widgets…
				</p>
			{/if}
		</Card.Root>
	{:else if shown.length}
		<CardGrid>
			{#each browsePage.items as listed (listed.id)}
				<BrowseCard {listed} />
			{/each}
		</CardGrid>
	{:else}
		<Card.Root class="px-6 py-10 text-center text-muted-foreground">
			{query.trim() ? 'No widget matches your search.' : 'The marketplace has no widgets yet.'}
		</Card.Root>
	{/if}

	{#if browsePage.count > 1}
		<div class="flex items-center justify-between gap-2 px-1">
			<p class="text-sm text-muted-foreground tabular-nums">
				{browsePage.first}-{browsePage.last} of {shown.length}
			</p>
			<div class="flex gap-2">
				<Button
					variant="outline"
					size="sm"
					disabled={browsePage.index === 0}
					onclick={() => turnPage(-1)}
				>
					<ChevronLeft />Previous
				</Button>
				<Button
					variant="outline"
					size="sm"
					disabled={browsePage.index === browsePage.count - 1}
					onclick={() => turnPage(1)}
				>
					Next<ChevronRight />
				</Button>
			</div>
		</div>
	{/if}
	<p class="px-1 text-sm text-muted-foreground">
		Marketplace widgets are made by their authors. Each is checked once, when it is listed, not at
		every release, and runs with the same access as the built-in widgets.
	</p>
</div>
