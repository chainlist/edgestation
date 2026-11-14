<!--
	Asks on the Edge whether to install a newer release of EdgeStation. The Edge page looks for one
	when it opens. Nothing downloads until the user touches Download & install.
-->
<script lang="ts">
	import { Button } from '#lib/components/ui/button/index.ts';
	import { createAppUpdate } from '#lib/state/app-update.svelte.ts';
	import { overlay } from '#lib/widgets/webview/motion.ts';
	import EdgeDialog from './EdgeDialog.svelte';

	/** Large buttons, for a finger on the Edge. */
	const BUTTON = 'h-14 rounded-2xl px-6 text-xl';

	const offer = createAppUpdate();

	// Debug builds skip the check, so `tauri dev` does not offer to install a release over the
	// installed app.
	$effect(() => {
		if (import.meta.env.DEV) return;
		void offer.check();
	});

	async function install() {
		if (await offer.install()) offer.dismiss();
	}
</script>

{#if offer.update}
	{@const update = offer.update}
	<!-- Web views draw above the page, so they leave a hole for the backdrop (see `overlay`). It
	     covers the whole Edge, so no widget takes a touch untill the user answers. -->
	<EdgeDialog
		backdropClass="fixed z-50 items-center"
		attach={overlay}
		role="alertdialog"
		aria-labelledby="update-title"
		aria-describedby="update-text"
		class="flex w-[min(40rem,100%)] flex-col gap-6 p-8"
	>
		<div class="grid gap-2">
			<h2 id="update-title" class="text-3xl font-semibold">Update available</h2>
			<p id="update-text" class="text-xl text-muted-foreground">
				EdgeStation {update.version} is ready to download. You have {update.currentVersion}.
			</p>
			{#if offer.error}
				<p class="text-xl text-destructive">{offer.error}</p>
			{/if}
		</div>
		<div class="flex justify-end gap-3">
			<Button variant="outline" class={BUTTON} disabled={offer.busy} onclick={offer.dismiss}>
				Later
			</Button>
			<Button class={BUTTON} disabled={offer.busy} onclick={install}>
				{offer.installLabel}
			</Button>
		</div>
	</EdgeDialog>
{/if}
