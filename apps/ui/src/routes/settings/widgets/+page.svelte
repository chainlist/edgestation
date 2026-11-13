<script lang="ts">
	import RotateCcw from '@lucide/svelte/icons/rotate-ccw';
	import InstalledWidgets from '#lib/components/settings/widgets/InstalledWidgets.svelte';
	import MarketplaceSettings from '#lib/components/settings/widgets/MarketplaceSettings.svelte';
	import PageHeader from '#lib/components/settings/PageHeader.svelte';
	import { Button } from '#lib/components/ui/button/index.ts';
	import { reloadEdge } from '#lib/tauri/events.ts';
	import { installed } from '#lib/widgets/installed.svelte.ts';
	import { loadListing } from '#lib/widgets/marketplace/state.svelte.ts';

	// The rows of marketplace widgets show their GitHub owner and if they are delisted.
	void loadListing();
</script>

<PageHeader title="Widgets" description="Everything you can place on the Edge pages.">
	{#snippet actions()}
		<Button variant="outline" onclick={reloadEdge}><RotateCcw />Reload Edge</Button>
	{/snippet}
</PageHeader>

<div class="grid gap-8">
	<MarketplaceSettings />
	<InstalledWidgets widgets={installed.current} />
</div>
