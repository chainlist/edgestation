<script lang="ts">
	import InstalledWidgets from '#lib/components/settings/widgets/InstalledWidgets.svelte';
	import { marketplaceOn } from '#lib/state/settings.svelte.ts';
	import { installed } from '#lib/widgets/installed.svelte.ts';
	import { loadListing } from '#lib/widgets/marketplace/state.svelte.ts';

	// The cards of marketplace widgets show their GitHub owner, thier updates and if they are delisted.
	if (marketplaceOn.current) void loadListing();
</script>

<InstalledWidgets widgets={installed.current} />
