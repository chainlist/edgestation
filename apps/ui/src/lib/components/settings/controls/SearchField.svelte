<script lang="ts">
	import Search from '@lucide/svelte/icons/search';
	import { Input } from '#lib/components/ui/input/index.ts';

	let {
		value = $bindable(''),
		label,
		oninput,
		class: className
	}: {
		value?: string;
		/** The placeholder, which screen readers also read as the name of the field. */
		label: string;
		oninput?: () => void;
		class?: string;
	} = $props();
</script>

<!-- A search field with a magnifier at its start. -->
<div class={['relative', className]}>
	<Search
		class="pointer-events-none absolute top-1/2 left-2.5 size-4 -translate-y-1/2 text-muted-foreground"
	/>
	<Input type="search" bind:value {oninput} placeholder={label} aria-label={label} class="pl-8" />
</div>
