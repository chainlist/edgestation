<script lang="ts">
	import ArrowLeft from '@lucide/svelte/icons/arrow-left';
	import type { Snippet } from 'svelte';
	import { Button } from '#lib/components/ui/button/index.ts';

	let {
		title,
		description,
		back,
		actions
	}: {
		title: string;
		description?: string;
		/** The page that the back button opens. Without it, there is no back button. */
		back?: string;
		actions?: Snippet;
	} = $props();
</script>

<!-- In a narrow window, the actions go below the title, so the title keeps its width. -->
<header class="mb-6 flex flex-wrap items-start justify-between gap-x-4 gap-y-3">
	<div class="flex min-w-72 flex-1 items-start gap-2">
		{#if back}
			<Button variant="ghost" size="icon" href={back} aria-label="Back" title="Back">
				<ArrowLeft />
			</Button>
		{/if}
		<div class={['grid min-w-0 flex-1 gap-1', back && 'pt-1']}>
			<h1 class="text-xl font-semibold">{title}</h1>
			{#if description}
				<p class="text-sm text-muted-foreground">{description}</p>
			{/if}
		</div>
	</div>
	{#if actions}
		<div class="flex shrink-0 gap-2">{@render actions()}</div>
	{/if}
</header>
