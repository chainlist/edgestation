<script lang="ts">
	import * as AlertDialog from '#lib/components/ui/alert-dialog/index.ts';
	import * as Card from '#lib/components/ui/card/index.ts';
	import { Switch } from '#lib/components/ui/switch/index.ts';
	import { marketplaceOn } from '#lib/state/settings.svelte.ts';
	import { switchMarketplace } from '#lib/widgets/marketplace/state.svelte.ts';
	import SettingsRow from '#lib/components/settings/SettingsRow.svelte';

	const id = $props.id();
	let asking = $state(false);
	let error = $state<string>();

	async function turn(on: boolean) {
		error = undefined;
		try {
			await switchMarketplace(on);
		} catch (reason) {
			error = `Couldn't turn the marketplace ${on ? 'on' : 'off'}. ${reason}`;
		}
	}
</script>

<!-- The switch shows the saved value only. Turning it on first asks for the warning to be
     accepted, and the host saves the change. -->
<Card.Root class="gap-0 py-0">
	<SettingsRow
		label="Community widgets"
		description="While this is off, the marketplace widgets you installed don't run."
		{error}
		for={id}
	>
		<Switch
			{id}
			bind:checked={() => marketplaceOn.current, (on) => (on ? (asking = true) : void turn(false))}
		/>
	</SettingsRow>
</Card.Root>

<AlertDialog.Root bind:open={asking}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Turn on the marketplace?</AlertDialog.Title>
			<AlertDialog.Description>
				The marketplace lists widgets that other people make and share.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<p class="text-sm">
			Each widget is reviewed when it's first listed, not at every update. It runs with the same
			access as the built-in widgets, so pick the ones whose authors you trust.
		</p>
		<p class="text-xs text-muted-foreground">You can turn the marketplace off at any time.</p>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action
				onclick={() => {
					asking = false;
					void turn(true);
				}}
			>
				Turn on
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
