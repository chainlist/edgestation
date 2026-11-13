<script lang="ts">
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import RefreshCw from '@lucide/svelte/icons/refresh-cw';
	import Store from '@lucide/svelte/icons/store';
	import { Button } from '#lib/components/ui/button/index.ts';
	import { Switch } from '#lib/components/ui/switch/index.ts';
	import { widgetAutoUpdate } from '#lib/state/settings.svelte.ts';
	import {
		loadListing,
		marketplace,
		updateWidgets
	} from '#lib/widgets/marketplace/state.svelte.ts';
	import SettingsGroup from '#lib/components/settings/SettingsGroup.svelte';
	import SettingsRow from '#lib/components/settings/SettingsRow.svelte';
	import Count from './Count.svelte';

	const id = $props.id();

	/** The result of Check for updates. It shows in place of the row description. */
	let updateStatus = $state<string>();
	let updateError = $state<string>();
	let checking = $state(false);

	async function checkForUpdates() {
		checking = true;
		updateStatus = 'Looking for newer releases…';
		updateError = undefined;
		try {
			const attempts = await updateWidgets();
			const updated = attempts.filter((attempt) => !attempt.error).length;
			const failed = attempts.length - updated;
			updateStatus =
				updated > 0
					? `Updated ${updated} ${updated === 1 ? 'widget' : 'widgets'}.`
					: failed > 0
						? undefined
						: 'Every widget is up to date.';
			if (failed > 0) updateError = `${failed} couldn't be updated: the reason is under each.`;
			void loadListing(true);
		} catch (e) {
			updateStatus = undefined;
			updateError = `Couldn't check for updates. ${e}`;
		} finally {
			checking = false;
		}
	}
</script>

<SettingsGroup title="Marketplace">
	<SettingsRow label="Community widgets" description="Widgets that other people make and publish.">
		<Button variant="outline" href="/settings/widgets/browse">
			<Store />Browse
			{#if marketplace.listing}<Count count={marketplace.listing.length} />{/if}
			<ChevronRight />
		</Button>
	</SettingsRow>
	<SettingsRow
		label="Update automatically"
		description="Installs newer releases of marketplace widgets each time EdgeStation opens. A release that asks for more access waits for you to update it."
		for="{id}-auto-update"
	>
		<Switch
			id="{id}-auto-update"
			bind:checked={() => widgetAutoUpdate.current, (on) => (widgetAutoUpdate.current = on)}
		/>
	</SettingsRow>
	<SettingsRow
		label="Updates"
		description={updateStatus ?? 'Look for newer releases now.'}
		error={updateError}
	>
		<Button variant="outline" onclick={checkForUpdates} disabled={checking}>
			<RefreshCw class={checking ? 'animate-spin' : undefined} />Check for updates
		</Button>
	</SettingsRow>
</SettingsGroup>
