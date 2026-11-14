<script lang="ts">
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import Store from '@lucide/svelte/icons/store';
	import { Button } from '#lib/components/ui/button/index.ts';
	import { marketplace } from '#lib/widgets/marketplace/state.svelte.ts';
	import SettingsGroup from '#lib/components/settings/SettingsGroup.svelte';
	import SettingsRow from '#lib/components/settings/SettingsRow.svelte';
	import Count from './Count.svelte';
</script>

<SettingsGroup title="Marketplace">
	<SettingsRow label="Community widgets" description="Widgets that other people make and publish.">
		<Button variant="outline" href="/settings/widgets/browse">
			<Store />Browse
			{#if marketplace.listing}<Count count={marketplace.listing.length} />{/if}
			<ChevronRight />
		</Button>
	</SettingsRow>
</SettingsGroup>
