<!--
	Asks on the Edge whether to install a newer release of EdgeStation. The Edge page looks for one
	when it opens. Nothing downloads until the user touches Download & install.
-->
<script lang="ts">
	import { check, type Update } from '@tauri-apps/plugin-updater';
	import { fade, scale } from 'svelte/transition';
	import { Button } from '#lib/components/ui/button/index.ts';
	import { installUpdate } from '#lib/tauri/updater.ts';
	import { overlay } from '#lib/widgets/webview/motion.ts';

	/** Large buttons, for a finger on the Edge. */
	const BUTTON = 'h-14 rounded-2xl px-6 text-xl';

	let update = $state<Update>();
	let installing = $state(false);
	/** The part of the download that is done, from 0 to 100. */
	let percent = $state(0);
	let error = $state<string>();

	// Debug builds skip the check, so `tauri dev` does not offer to install a release over the
	// installed app.
	$effect(() => {
		if (import.meta.env.DEV) return;
		check().then((found) => (update = found ?? undefined), console.error);
	});

	function later() {
		update?.close().catch(console.error);
		update = undefined;
	}

	async function install() {
		if (!update) return;
		installing = true;
		percent = 0;
		error = undefined;
		try {
			await installUpdate(update, (p) => (percent = p));
			later();
		} catch (e) {
			console.error(e);
			error = `Couldn't update: ${e}`;
		} finally {
			installing = false;
		}
	}
</script>

{#if update}
	<!-- Web views draw above the page, so they leave a hole for the backdrop (see `overlay`). It
	     covers the whole Edge, so no widget takes a touch untill the user answers. -->
	<div
		{@attach overlay}
		class="fixed inset-0 z-50 flex items-center justify-center bg-background/40 p-6"
		transition:fade={{ duration: 150 }}
	>
		<div
			role="alertdialog"
			aria-labelledby="update-title"
			aria-describedby="update-text"
			class="flex w-[min(40rem,100%)] flex-col gap-6 rounded-3xl border bg-card/90 p-8 shadow-lg backdrop-blur-md"
			in:scale={{ start: 0.96, duration: 150 }}
		>
			<div class="grid gap-2">
				<h2 id="update-title" class="text-3xl font-semibold">Update available</h2>
				<p id="update-text" class="text-xl text-muted-foreground">
					EdgeStation {update.version} is ready to download. You have {update.currentVersion}.
				</p>
				{#if error}
					<p class="text-xl text-destructive">{error}</p>
				{/if}
			</div>
			<div class="flex justify-end gap-3">
				<Button variant="outline" class={BUTTON} disabled={installing} onclick={later}>
					Later
				</Button>
				<Button class={BUTTON} disabled={installing} onclick={install}>
					{installing ? `Downloading… ${percent}%` : 'Download & install'}
				</Button>
			</div>
		</div>
	</div>
{/if}
