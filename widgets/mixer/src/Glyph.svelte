<script lang="ts">
	import { ICONS, type Icon } from './icons.ts';

	let { name }: { name: Icon } = $props();
</script>

<svg viewBox="0 0 24 24" aria-hidden="true">
	{#each ICONS[name] as d (d)}<path {d} />{/each}
</svg>

<style>
	svg {
		width: 1em;
		height: 1em;
		flex: none;
		fill: none;
		stroke: currentColor;
		stroke-width: 2;
		stroke-linecap: round;
		stroke-linejoin: round;
	}
</style>
