<script lang="ts">
	import Download from '@lucide/svelte/icons/download';
	import FolderOpen from '@lucide/svelte/icons/folder-open';
	import LoaderCircle from '@lucide/svelte/icons/loader-circle';
	import RefreshCw from '@lucide/svelte/icons/refresh-cw';
	import Search from '@lucide/svelte/icons/search';
	import Store from '@lucide/svelte/icons/store';
	import type { Snippet } from 'svelte';
	import { Button } from '#lib/components/ui/button/index.ts';
	import * as Card from '#lib/components/ui/card/index.ts';
	import { Input } from '#lib/components/ui/input/index.ts';
	import { openWidgetsFolder, rescanWidgets } from '#lib/widgets/installed.svelte.ts';
	import { listingStatus } from '#lib/widgets/marketplace/listing.ts';
	import { marketplace, updateWidgets } from '#lib/widgets/marketplace/state.svelte.ts';
	import { filterInstalled } from '#lib/widgets/search.ts';
	import type { InstalledWidget } from '#lib/widgets/types.ts';
	import SegmentedControl from '#lib/components/settings/controls/SegmentedControl.svelte';
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

<section class="grid gap-3">
	<div class="flex min-h-8 items-center justify-between gap-2">
		<h2 class="text-sm font-medium">Installed</h2>
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
	</div>
	{#if updateError}
		<p class="px-1 text-sm text-destructive">{updateError}</p>
	{/if}
	<div class="flex flex-wrap items-center justify-between gap-2">
		<SegmentedControl
			items={FILTERS}
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
			<div class="relative flex-1">
				<Search
					class="pointer-events-none absolute top-1/2 left-2.5 size-4 -translate-y-1/2 text-muted-foreground"
				/>
				<Input
					type="search"
					bind:value={query}
					placeholder="Search installed widgets"
					aria-label="Search installed widgets"
					class="pl-8"
				/>
			</div>
			<Button
				variant="ghost"
				size="icon"
				title="Rescan the widgets folder"
				aria-label="Rescan the widgets folder"
				disabled={scanning}
				onclick={rescan}
			>
				<RefreshCw class={scanning ? 'animate-spin' : undefined} />
			</Button>
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
				{@render empty(
					Store,
					'Nothing from the marketplace yet',
					'Widgets you install from the marketplace show here, with their updates.',
					browse
				)}
			{:else if filter === 'local'}
				{@render empty(
					FolderOpen,
					'No widgets of your own yet',
					'A widget is a folder holding a manifest.json and a main.js. Drop one into the widgets folder, then rescan.',
					openFolder
				)}
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
</section>

{#snippet empty(Icon: typeof Store, title: string, text: string, action: Snippet)}
	<div class="grid justify-items-center gap-3 px-6 py-10 text-center">
		<span
			class="flex size-10 items-center justify-center rounded-lg bg-muted text-muted-foreground"
		>
			<Icon class="size-5" />
		</span>
		<div class="grid gap-1">
			<p class="font-medium">{title}</p>
			<p class="max-w-sm text-muted-foreground">{text}</p>
		</div>
		{@render action()}
	</div>
{/snippet}

{#snippet browse()}
	<Button variant="outline" size="sm" href="/settings/widgets/browse"
		><Store />Browse widgets</Button
	>
{/snippet}

{#snippet openFolder()}
	<Button variant="outline" size="sm" onclick={openWidgetsFolder}><FolderOpen />Open folder</Button>
{/snippet}
