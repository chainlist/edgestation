<script lang="ts">
	import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';
	import PageHeader from '#lib/components/settings/PageHeader.svelte';
	import SettingsGroup from '#lib/components/settings/SettingsGroup.svelte';
	import SettingsRow from '#lib/components/settings/SettingsRow.svelte';
	import { Button } from '#lib/components/ui/button/index.ts';
	import { Switch } from '#lib/components/ui/switch/index.ts';
	import { createAppUpdate } from '#lib/state/app-update.svelte.ts';
	import { freezeOnFullscreen } from '#lib/state/settings.svelte.ts';
	import { appVersion } from '#lib/tauri/app-version.ts';

	const id = $props.id();

	/**
	 * True if the system opens the app when you sign in. It is `undefined` until the read is
	 * done. The system keeps this value, not the store, so a change in Task Manager (Windows) or
	 * in the autostart folder (Linux) also shows here.
	 */
	let openAtStartup = $state<boolean>();
	isEnabled().then((on) => (openAtStartup = on), console.error);

	async function setOpenAtStartup(on: boolean) {
		openAtStartup = on;
		try {
			await (on ? enable() : disable());
		} catch (e) {
			console.error(e);
			openAtStartup = !on;
		}
	}

	let version = $state<string>();
	appVersion().then((v) => (version = v), console.error);

	/** The newer release that Check for updates found. It installs only when the user says so. */
	const offer = createAppUpdate();
	/** The result of Check for updates. It shows in place of the row description. */
	let updateStatus = $state<string>();

	async function checkForUpdates() {
		updateStatus = 'Looking for a newer release…';
		const found = await offer.check();
		updateStatus = offer.error
			? undefined
			: found
				? `EdgeStation ${found.version} is available. You have ${found.currentVersion}.`
				: 'You have the latest release.';
	}

	async function install() {
		if (!offer.update) return;
		const shown = updateStatus;
		updateStatus = `Updating to ${offer.update.version}. EdgeStation closes and opens again once it's done.`;
		if (!(await offer.install())) updateStatus = shown;
	}
</script>

<PageHeader
	title="General"
	description="How EdgeStation runs: whether it opens when you sign in, whether the Edge rests while another app is full screen, and how it updates."
/>

<div class="grid gap-8">
	<SettingsGroup title="Startup">
		<SettingsRow
			label="Open at startup"
			description="Opens EdgeStation on the Edge when you sign in. To open Settings, use the tray icon or start EdgeStation again."
			for="{id}-startup"
		>
			<Switch
				id="{id}-startup"
				bind:checked={() => openAtStartup ?? false, setOpenAtStartup}
				disabled={openAtStartup === undefined}
			/>
		</SettingsRow>
	</SettingsGroup>

	<SettingsGroup title="While another app is full screen">
		<SettingsRow
			label="Pause widgets"
			description="A game or video covering a monitor pauses the widgets, so they stop polling and redrawing, and the Edge shows a Paused badge."
			for="{id}-pause"
		>
			<Switch
				id="{id}-pause"
				bind:checked={() => freezeOnFullscreen.current, (on) => (freezeOnFullscreen.current = on)}
			/>
		</SettingsRow>
		{#snippet note()}
			Switching to another app, or leaving full screen, resumes them; touching the Edge doesn't.
		{/snippet}
	</SettingsGroup>

	<SettingsGroup title="Updates">
		<SettingsRow
			label={version ? `Version ${version}` : 'Version'}
			description={updateStatus ??
				'EdgeStation looks for a newer release each time it opens, and asks on the Edge before it installs it.'}
			error={offer.error}
		>
			{#if offer.update}
				<Button onclick={install} disabled={offer.busy}>
					{offer.installLabel}
				</Button>
			{:else}
				<Button variant="outline" onclick={checkForUpdates} disabled={offer.busy}>
					Check for updates
				</Button>
			{/if}
		</SettingsRow>
	</SettingsGroup>
</div>
