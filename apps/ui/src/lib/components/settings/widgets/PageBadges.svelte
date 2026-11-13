<script lang="ts">
	import LayoutGrid from '@lucide/svelte/icons/layout-grid';
	import { Badge } from '#lib/components/ui/badge/index.ts';
	import type { Page } from '#lib/layout/types.ts';

	let { pages, class: className }: { pages: Page[]; class?: string } = $props();
</script>

<!-- Links to the pages that show a widget. It is `relative` to stay above a streched link. -->
{#if pages.length}
	<div class={['relative flex flex-wrap justify-end gap-1', className]}>
		{#each pages as page (page.id)}
			<Badge
				href="/settings/pages/{page.id}"
				variant="secondary"
				class="max-w-full"
				title="On the {page.name} page"
			>
				<LayoutGrid /><span class="truncate">{page.name}</span>
			</Badge>
		{/each}
	</div>
{/if}
