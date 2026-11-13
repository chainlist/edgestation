<script lang="ts">
	import AppWindow from '@lucide/svelte/icons/app-window';
	import CirclePlay from '@lucide/svelte/icons/circle-play';
	import Cpu from '@lucide/svelte/icons/cpu';
	import Globe from '@lucide/svelte/icons/globe';
	import PanelsTopLeft from '@lucide/svelte/icons/panels-top-left';
	import Volume2 from '@lucide/svelte/icons/volume-2';
	import { Badge } from '#lib/components/ui/badge/index.ts';
	import { PERMISSIONS, shownPermissions } from '#lib/widgets/permissions.ts';
	import type { InstalledWidget, WidgetPermission } from '#lib/widgets/types.ts';

	const PERMISSION_ICONS: Record<WidgetPermission, typeof Cpu> = {
		system: Cpu,
		media: CirclePlay,
		apps: AppWindow,
		audio: Volume2,
		fetch: Globe,
		webview: PanelsTopLeft
	};

	let { widget }: { widget: InstalledWidget } = $props();

	const permissions = $derived(shownPermissions(widget));
</script>

{#if permissions.length}
	<span class="inline-flex flex-wrap items-center gap-1">
		Asks for
		{#each permissions as permission (permission)}
			{@const Icon = PERMISSION_ICONS[permission]}
			<Badge variant="outline" class="relative" title={PERMISSIONS[permission].allows}>
				<Icon />{PERMISSIONS[permission].label}
			</Badge>
		{/each}
	</span>
{/if}
