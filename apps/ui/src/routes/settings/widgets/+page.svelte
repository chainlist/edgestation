<script lang="ts">
	import InstalledWidgets from '#lib/components/settings/widgets/InstalledWidgets.svelte';
	import MarketplaceSettings from '#lib/components/settings/widgets/MarketplaceSettings.svelte';
	import PageHeader from '#lib/components/settings/PageHeader.svelte';
	import { installed } from '#lib/widgets/installed.svelte.ts';
	import { loadListing } from '#lib/widgets/marketplace/state.svelte.ts';

	// The rows of marketplace widgets show their GitHub owner and if they are delisted.
	void loadListing();
</script>

<PageHeader title="Widgets" description="Everything you can place on the Edge pages." />

<div class="grid gap-8">
	<MarketplaceSettings />
	<InstalledWidgets widgets={installed.current} />
</div>
