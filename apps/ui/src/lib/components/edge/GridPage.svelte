<script lang="ts">
	import { gridStyle } from '#lib/layout/grid.ts';
	import type { Page } from '#lib/layout/types.ts';
	import { createPause } from '#lib/widgets/api/timers.ts';
	import WidgetSlot from './WidgetSlot.svelte';

	let { page, paused }: { page: Page; paused: boolean } = $props();

	// Create it once, with the page state at mount. A pause or a resume never mounts the widgets
	// again.
	// svelte-ignore state_referenced_locally
	const pause = createPause(paused);
	$effect(() => pause.set(paused));

	// The widgets mount the first time the page shows, then stay. Thus a page that nobody looks
	// at loads nothing.
	let started = $state(false);
	$effect(() => {
		if (!paused) started = true;
	});
</script>

<section
	class="grid size-full shrink-0 gap-3 p-3 [content-visibility:auto]"
	style={gridStyle(page)}
	aria-label={page.name}
>
	{#each page.items as item (item.id)}
		<WidgetSlot {item} {pause} {started} />
	{/each}
</section>
