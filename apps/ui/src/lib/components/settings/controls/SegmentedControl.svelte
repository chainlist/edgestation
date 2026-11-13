<script lang="ts" generics="T">
	import type { Snippet } from 'svelte';

	let {
		items,
		selected,
		onpick,
		label,
		name
	}: {
		items: readonly T[];
		selected: (item: T) => boolean;
		onpick: (item: T) => void;
		label: Snippet<[T]>;
		/** The subject of the choice, for screen readers. */
		name: string;
	} = $props();
</script>

<!-- A small set of exclusive choices side by side, for the control side of a SettingsRow. The
     selected choice is raised. -->
<div role="group" aria-label={name} class="inline-flex items-center rounded-lg bg-muted p-[3px]">
	{#each items as item, i (i)}
		{@const on = selected(item)}
		<button
			type="button"
			aria-pressed={on}
			class={[
				'inline-flex h-8 items-center gap-1.5 rounded-md border border-transparent px-3 text-sm font-medium whitespace-nowrap transition-[color,background-color,box-shadow] outline-none focus-visible:ring-3 focus-visible:ring-ring/50 [&_svg]:size-4 [&_svg]:shrink-0',
				on
					? 'bg-background text-foreground shadow-sm dark:border-input dark:bg-input/30'
					: 'text-muted-foreground hover:text-foreground'
			]}
			onclick={() => onpick(item)}
		>
			{@render label(item)}
		</button>
	{/each}
</div>
