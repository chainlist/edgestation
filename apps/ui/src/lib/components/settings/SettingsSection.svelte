<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		title,
		actions,
		children
	}: {
		title: string;
		/** Buttons on the right of the title. */
		actions?: Snippet;
		children: Snippet;
	} = $props();
</script>

{#snippet heading()}
	<h2 class="text-sm font-medium">{title}</h2>
{/snippet}

<section class="grid gap-3">
	{#if actions}
		<div class="flex min-h-8 items-center justify-between gap-2">
			{@render heading()}
			{@render actions()}
		</div>
	{:else}
		{@render heading()}
	{/if}
	{@render children()}
</section>
