<script lang="ts">
	import BringToFront from '@lucide/svelte/icons/bring-to-front';
	import ScanEye from '@lucide/svelte/icons/scan-eye';
	import type { Monitor } from '@tauri-apps/api/window';
	import ChoiceSelect from '#lib/components/settings/controls/ChoiceSelect.svelte';
	import PageHeader from '#lib/components/settings/PageHeader.svelte';
	import SettingsSection from '#lib/components/settings/SettingsSection.svelte';
	import { Button } from '#lib/components/ui/button/index.ts';
	import { display } from '#lib/state/settings.svelte.ts';
	import { bringEdgeToFront, listMonitors } from '#lib/tauri/edge-window.ts';
	import { identifyScreens, prepareIdentify } from '#lib/tauri/identify.ts';

	const AUTO = 'auto';

	let monitors = $state.raw<Monitor[]>([]);
	listMonitors().then((found) => (monitors = found));

	// Prepare the boxes befor the click, so they show at once.
	$effect(() => prepareIdentify());

	const choices = $derived([
		{ value: AUTO, label: 'Automatic (first secondary display)' },
		...monitors
			.filter((m) => m.name)
			.map((m) => ({ value: m.name!, label: `${m.name} · ${m.size.width} × ${m.size.height}` }))
	]);
</script>

<PageHeader
	title="Display"
	description="The Edge window is made for the Corsair Xeneon Edge and works on any monitor: pick the one it shows on."
>
	{#snippet actions()}
		<Button variant="outline" onclick={() => bringEdgeToFront().catch(console.error)}>
			<BringToFront />Bring to front
		</Button>
	{/snippet}
</PageHeader>

<div class="grid gap-8">
	<SettingsSection title="Monitor">
		<div class="flex flex-wrap gap-2">
			<ChoiceSelect
				class="w-96"
				{choices}
				value={display.current.monitor ?? AUTO}
				onchange={(value) => (display.current = { monitor: value === AUTO ? null : value })}
			/>
			<Button variant="outline" onclick={() => identifyScreens().catch(console.error)}>
				<ScanEye />Identify screens
			</Button>
		</div>
	</SettingsSection>
</div>
