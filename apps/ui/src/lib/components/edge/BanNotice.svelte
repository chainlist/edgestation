<!--
	Warns on the Edge that the app removed marketplace widgets that the registry banned. The Edge asks
	the host to remove them when it opens. Settings does it at each read of the marketplace.
-->
<script lang="ts">
	import ShieldAlert from '@lucide/svelte/icons/shield-alert';
	import { invoke } from '@tauri-apps/api/core';
	import { Button } from '#lib/components/ui/button/index.ts';
	import { onWidgetsBanned } from '#lib/tauri/events.ts';
	import type { BannedWidget } from '#lib/widgets/types.ts';
	import { overlay } from '#lib/widgets/webview/motion.ts';
	import EdgeDialog from './EdgeDialog.svelte';

	/** A large button, for a finger on the Edge. */
	const BUTTON = 'h-14 rounded-2xl bg-destructive px-8 text-xl text-white hover:bg-destructive/90';

	let removed = $state.raw<BannedWidget[]>([]);
	const add = (widgets: BannedWidget[]) => (removed = [...removed, ...widgets]);

	$effect(() => onWidgetsBanned(add));

	// Offline, the check fails and the the next one tries again.
	$effect(() => {
		invoke<BannedWidget[]>('marketplace_remove_banned').then(add, console.error);
	});
</script>

{#if removed.length}
	<!-- The card is tinted from inside. Its own `bg-card` class would fight a second background
	     class. -->
	<EdgeDialog
		backdropClass="fixed z-50 items-center"
		attach={overlay}
		role="alertdialog"
		aria-labelledby="ban-title"
		aria-describedby="ban-text"
		class="w-[min(44rem,100%)] overflow-hidden border-destructive ring-4 ring-destructive/40"
	>
		<div class="flex flex-col gap-6 bg-destructive/15 p-8">
			<div class="flex items-start gap-5">
				<ShieldAlert class="size-12 shrink-0 text-destructive" />
				<div class="grid gap-3">
					<h2 id="ban-title" class="text-3xl font-semibold text-destructive">
						{removed.length === 1 ? 'Widget removed' : 'Widgets removed'}
					</h2>
					<div id="ban-text" class="grid gap-3 text-xl">
						{#each removed as widget, index (index)}
							<div>
								<p>
									<strong>{widget.name}</strong> was uninstalled because the marketplace banned it.
								</p>
								{#if widget.reason}
									<p class="text-muted-foreground">Reason: {widget.reason}</p>
								{/if}
							</div>
						{/each}
						<p class="text-muted-foreground">
							{removed.length === 1 ? 'Its' : 'Their'} tiles and saved data were removed too.
						</p>
					</div>
				</div>
			</div>
			<div class="flex justify-end">
				<Button class={BUTTON} onclick={() => (removed = [])}>OK</Button>
			</div>
		</div>
	</EdgeDialog>
{/if}
