<script lang="ts">
	import Download from '@lucide/svelte/icons/download';
	import FolderOpen from '@lucide/svelte/icons/folder-open';
	import LoaderCircle from '@lucide/svelte/icons/loader-circle';
	import Store from '@lucide/svelte/icons/store';
	import { Button } from '#lib/components/ui/button/index.ts';
	import * as Card from '#lib/components/ui/card/index.ts';
	import { marketplaceOn } from '#lib/state/settings.svelte.ts';
	import { installed, openWidgetsFolder, rescanWidgets } from '#lib/widgets/installed.svelte.ts';
	import { listingStatus } from '#lib/widgets/marketplace/listing.ts';
	import { marketplace, updateWidgets } from '#lib/widgets/marketplace/state.svelte.ts';
	import { filterInstalled } from '#lib/widgets/search.ts';
	import type { InstalledWidget } from '#lib/widgets/types.ts';
	import RefreshButton from '#lib/components/settings/controls/RefreshButton.svelte';
	import SearchField from '#lib/components/settings/controls/SearchField.svelte';
	import SegmentedControl from '#lib/components/settings/controls/SegmentedControl.svelte';
	import EmptyState from '#lib/components/settings/EmptyState.svelte';
	import PageHeader from '#lib/components/settings/PageHeader.svelte';
	import CardGrid from './CardGrid.svelte';
	import Count from './Count.svelte';
	import InstalledCard from './InstalledCard.svelte';

	type Filter = 'all' | InstalledWidget['source'];

	const FILTERS: { value: Filter; label: string }[] = [
		{ value: 'all', label: 'All' },
		{ value: 'builtin', label: 'Built-in' },
		{ value: 'marketplace', label: 'Marketplace' },
		{ value: 'local', label: 'Local' }
	];

	let { widgets }: { widgets: InstalledWidget[] } = $props();

	// While the marketplace is off, its widgets are not in the list.
	const filters = $derived(
		marketplaceOn.current ? FILTERS : FILTERS.filter(({ value }) => value !== 'marketplace')
	);

	let query = $state('');
	let filter = $state<Filter>('all');
	let scanning = $state(false);

	const matched = $derived(filterInstalled(widgets, query));
	const inFilter = (value: Filter) =>
		value === 'all' ? matched : matched.filter((widget) => widget.source === value);
	const shown = $derived(inFilter(filter));

	/** The ids of the marketplace widgets that have a newer release they can install. */
	const outdated = $derived(
		widgets
			.filter((widget) => {
				const listed = marketplace.listing?.find((l) => l.id === widget.id);
				return listed && listingStatus(listed, widgets, marketplace.appVersion).kind === 'update';
			})
			.map((widget) => widget.id)
	);
	let updating = $state(false);
	let updateError = $state<string>();

	async function updateAll() {
		updating = true;
		updateError = undefined;
		try {
			await updateWidgets(outdated);
		} catch (error) {
			updateError = `Couldn't update the widgets. ${error}`;
		} finally {
			updating = false;
		}
	}

	async function rescan() {
		scanning = true;
		try {
			await rescanWidgets();
		} finally {
			scanning = false;
		}
	}
</script>

<PageHeader title="Installed" description="Everything you can place on the Edge pages.">
	{#snippet actions()}
		{#if outdated.length}
			<Button
				size="sm"
				disabled={updating || outdated.some((id) => marketplace.busy(id))}
				onclick={updateAll}
			>
				{#if updating}<LoaderCircle class="animate-spin" />{:else}<Download />{/if}
				Update all <Count count={outdated.length} />
			</Button>
		{/if}
	{/snippet}
</PageHeader>

<div class="grid gap-3">
	{#if updateError}
		<p class="px-1 text-sm text-destructive">{updateError}</p>
	{/if}
	<!-- A failed scan keeps the previous list. -->
	{#if installed.error}
		<p class="px-1 text-sm text-destructive">Couldn't read the widgets folder. {installed.error}</p>
	{/if}
	<div class="flex flex-wrap items-center justify-between gap-2">
		<SegmentedControl
			items={filters}
			selected={({ value }) => value === filter}
			onpick={({ value }) => (filter = value)}
			name="Source"
		>
			{#snippet label({ value, label })}
				{@const list = inFilter(value)}
				{label}
				<Count count={list.length} broken={list.filter((widget) => widget.error).length} />
			{/snippet}
		</SegmentedControl>
		<div class="flex min-w-56 flex-1 items-center gap-1">
			<SearchField bind:value={query} label="Search installed widgets" class="flex-1" />
			<RefreshButton label="Rescan the widgets folder" busy={scanning} onclick={rescan} />
			<Button
				variant="ghost"
				size="icon"
				title="Open the widgets folder"
				aria-label="Open the widgets folder"
				onclick={openWidgetsFolder}
			>
				<FolderOpen />
			</Button>
		</div>
	</div>

	{#if shown.length}
		<CardGrid>
			{#each shown as widget (widget.id)}
				<InstalledCard {widget} />
			{/each}
		</CardGrid>
	{:else}
		<Card.Root class="gap-0 py-0">
			{#if query.trim()}
				<p class="px-4 py-3.5 text-muted-foreground">No installed widget matches your search.</p>
			{:else if filter === 'marketplace'}
				<EmptyState
					icon={Store}
					title="Nothing from the marketplace yet"
					text="Widgets you install from the marketplace show here, with their updates."
				>
					<Button variant="outline" size="sm" href="/settings/widgets/marketplace"
						><Store />Browse widgets</Button
					>
				</EmptyState>
			{:else if filter === 'local'}
				<EmptyState
					icon={FolderOpen}
					title="No widgets of your own yet"
					text="A widget is a folder holding a manifest.json and a main.js. Drop one into the widgets folder, then rescan."
				>
					<Button variant="outline" size="sm" onclick={openWidgetsFolder}
						><FolderOpen />Open folder</Button
					>
				</EmptyState>
			{:else}
				<p class="px-4 py-3.5 text-muted-foreground">No widgets found.</p>
			{/if}
		</Card.Root>
	{/if}
	{#if filter === 'local'}
		<p class="px-1 text-sm text-muted-foreground">
			A folder named after a built-in or marketplace widget, such as <code class="font-mono"
				>clock</code
			>, replaces it.
		</p>
	{/if}
</div>
