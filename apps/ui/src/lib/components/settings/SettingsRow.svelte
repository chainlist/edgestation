<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Label } from '#lib/components/ui/label/index.ts';

	let {
		label,
		description,
		error,
		for: control,
		children
	}: {
		label: string;
		description?: string;
		/** Shows below the description, as an error. */
		error?: string;
		/** The id of the control that the label names. */
		for?: string;
		children: Snippet;
	} = $props();
</script>

<!-- One setting in a SettingsGroup. Its description is on the left and its control is on the right.
     In a narrow window, the control goes below. -->
<div class="flex flex-wrap items-center justify-between gap-x-8 gap-y-3 px-4 py-3.5">
	<div class="grid min-w-48 flex-1 gap-1">
		{#if control}
			<Label for={control}>{label}</Label>
		{:else}
			<span class="text-sm leading-none font-medium">{label}</span>
		{/if}
		{#if description}
			<p class="text-muted-foreground">{description}</p>
		{/if}
		{#if error}
			<p class="text-destructive">{error}</p>
		{/if}
	</div>
	<div class="flex shrink-0 flex-wrap items-center gap-2">{@render children()}</div>
</div>
