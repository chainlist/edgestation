<script lang="ts">
	import { getVersion } from '@tauri-apps/api/app';
	import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart';
	import { check, type Update } from '@tauri-apps/plugin-updater';
	import PageHeader from '#lib/components/settings/PageHeader.svelte';
	import SettingsGroup from '#lib/components/settings/SettingsGroup.svelte';
	import SettingsRow from '#lib/components/settings/SettingsRow.svelte';
	import { Button } from '#lib/components/ui/button/index.ts';
	import { Switch } from '#lib/components/ui/switch/index.ts';
	import { freezeOnFullscreen } from '#lib/state/settings.svelte.ts';
	import { installUpdate } from '#lib/tauri/updater.ts';

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
	getVersion().then((v) => (version = v), console.error);

	/** The result of Check for updates. It shows in place of the row description. */
	let updateStatus = $state<string>();
	let updateError = $state<string>();
	/** True while the app looks for a release or instals one. */
	let busy = $state(false);
	/** The newer release that Check for updates found. It installs only when the user says so. */
	let update = $state<Update>();
	/** The part of the download that is done, from 0 to 100. */
	let percent = $state(0);

	async function checkForUpdates() {
		busy = true;
		updateStatus = 'Looking for a newer release…';
		updateError = undefined;
		try {
			update = (await check()) ?? undefined;
			updateStatus = update
				? `EdgeStation ${update.version} is available. You have ${update.currentVersion}.`
				: 'You have the latest release.';
		} catch (e) {
			console.error(e);
			updateStatus = undefined;
			updateError = `Couldn't update: ${e}`;
		} finally {
			busy = false;
		}
	}

	async function install() {
		if (!update) return;
		const offer = updateStatus;
		busy = true;
		percent = 0;
		updateError = undefined;
		updateStatus = `Updating to ${update.version}. EdgeStation closes and opens again once it's done.`;
		try {
			await installUpdate(update, (p) => (percent = p));
		} catch (e) {
			console.error(e);
			updateStatus = offer;
			updateError = `Couldn't update: ${e}`;
		} finally {
			busy = false;
		}
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
			error={updateError}
		>
			{#if update}
				<Button onclick={install} disabled={busy}>
					{busy ? `Downloading… ${percent}%` : 'Download & install'}
				</Button>
			{:else}
				<Button variant="outline" onclick={checkForUpdates} disabled={busy}>
					Check for updates
				</Button>
			{/if}
		</SettingsRow>
	</SettingsGroup>
</div>
