<script lang="ts">
	import Check from '@lucide/svelte/icons/check';
	import Download from '@lucide/svelte/icons/download';
	import LoaderCircle from '@lucide/svelte/icons/loader-circle';
	import Trash from '@lucide/svelte/icons/trash';
	import * as AlertDialog from '#lib/components/ui/alert-dialog/index.ts';
	import { Button } from '#lib/components/ui/button/index.ts';
	import { pagesShowing } from '#lib/state/settings.svelte.ts';
	import { installed } from '#lib/widgets/installed.svelte.ts';
	import { listingStatus } from '#lib/widgets/marketplace/listing.ts';
	import { addedPermissions, declaredPermissions, PERMISSIONS } from '#lib/widgets/permissions.ts';
	import {
		installWidget,
		marketplace,
		uninstallWidget
	} from '#lib/widgets/marketplace/state.svelte.ts';

	let {
		id,
		removable = false,
		compact = false,
		updatable = false
	}: {
		id: string;
		removable?: boolean;
		/** Uninstall is an icon button, for a widget row. */
		compact?: boolean;
		/** Offers a newer release. Only the Installed cards do, so elswhere it shows as installed. */
		updatable?: boolean;
	} = $props();

	const listed = $derived(marketplace.listing?.find((widget) => widget.id === id));
	const latest = $derived(
		listed && listingStatus(listed, installed.current, marketplace.appVersion)
	);
	const status = $derived(
		latest?.kind === 'update' && !updatable ? ({ kind: 'installed' } as const) : latest
	);
	const mine = $derived(installed.current.find((w) => w.id === id && w.source === 'marketplace'));
	const busy = $derived(marketplace.busy(id));
	/** The pages that show the widget. They lose it when it is uninstalled. */
	const usedOn = $derived(pagesShowing(id));

	let confirming = $state(false);
	/** The dialog is open. It tells what the widget can do before an install or an update. */
	let asking = $state(false);
	const name = $derived(listed?.manifest?.name ?? mine?.name ?? id);
	/** The version that an update installs, `undefined` for an install. */
	const updating = $derived(status?.kind === 'update' ? status.version : undefined);
	/** What the dialog lists. An update lists only the permissions that it adds. */
	const permissions = $derived(
		!listed?.manifest
			? []
			: updating && mine
				? addedPermissions(mine, listed.manifest)
				: declaredPermissions(listed.manifest)
	);

	/** Asks first only if the widget asks for permissions, or an update adds some. */
	function install() {
		if (permissions.length) asking = true;
		else void installWidget(id);
	}

	function uninstall() {
		if (usedOn.length) confirming = true;
		else void uninstallWidget(id);
	}
</script>

<!-- The Install, Update or Uninstall button for marketplace widget `id`, as applicable. Uninstall
     shows only if `removable`. An uninstall of a widget that is on pages asks first. -->
{#if status?.kind === 'unavailable'}
	<span class="max-w-48 text-right text-xs text-muted-foreground">{status.reason}</span>
{:else if status?.kind === 'install' || status?.kind === 'update'}
	<Button size="sm" disabled={!!busy} onclick={install}>
		{#if busy === 'install'}<LoaderCircle class="animate-spin" />{:else}<Download />{/if}
		{status.kind === 'install' ? 'Install' : `Update to ${status.version}`}
	</Button>
{:else if status?.kind === 'installed' && !removable}
	<span class="flex items-center gap-1 text-xs text-muted-foreground">
		<Check class="size-3.5" />Installed
	</span>
{/if}
{#if removable && mine}
	<Button
		size={compact ? 'icon-sm' : 'sm'}
		variant={compact ? 'ghost' : 'outline'}
		class={compact ? 'text-muted-foreground hover:text-destructive' : undefined}
		title={compact ? 'Uninstall' : undefined}
		aria-label={compact ? `Uninstall ${name}` : undefined}
		disabled={!!busy}
		onclick={uninstall}
	>
		{#if busy === 'uninstall'}<LoaderCircle class="animate-spin" />{:else}<Trash />{/if}
		{#if !compact}Uninstall{/if}
	</Button>
{/if}

<AlertDialog.Root bind:open={confirming}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Uninstall {name}?</AlertDialog.Title>
			<AlertDialog.Description>
				It's on {usedOn.map((page) => page.name).join(', ')}. Its tiles there show an error until
				you remove them or install it again.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action
				variant="destructive"
				onclick={() => {
					confirming = false;
					void uninstallWidget(id);
				}}
			>
				<Trash />Uninstall
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

<AlertDialog.Root bind:open={asking}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>{updating ? 'Update' : 'Install'} {name}?</AlertDialog.Title>
			<AlertDialog.Description>
				{updating ? `${name} ${updating} will also be able to:` : `${name} will be able to:`}
			</AlertDialog.Description>
		</AlertDialog.Header>
		<ul class="grid list-disc gap-1.5 pl-5 text-sm">
			{#each permissions as permission (permission)}
				<li>{PERMISSIONS[permission].allows}</li>
			{/each}
		</ul>
		<p class="text-xs text-muted-foreground">
			Widgets run inside EdgeStation, so install only the ones you trust.
		</p>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action
				onclick={() => {
					asking = false;
					void installWidget(id);
				}}
			>
				<Download />{updating ? 'Update' : 'Install'}
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
