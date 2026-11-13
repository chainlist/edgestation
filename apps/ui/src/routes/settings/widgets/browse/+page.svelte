<script lang="ts">
	import BrowseWidgets from '#lib/components/settings/widgets/BrowseWidgets.svelte';
	import PageHeader from '#lib/components/settings/PageHeader.svelte';
	import { loadListing } from '#lib/widgets/marketplace/state.svelte.ts';

	void loadListing();
</script>

<PageHeader
	title="Browse widgets"
	description="Widgets that other people make and publish."
	back="/settings/widgets"
/>

<BrowseWidgets />
