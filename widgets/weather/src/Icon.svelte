<script lang="ts">
	/** The paths of the icon on a 24×24 box, and a label for screen readers. */
	let { paths, label }: { paths: string[]; label: string } = $props();
</script>

<!-- The icon fills its parent. The parent sets the size. -->
<svg viewBox="0 0 24 24" role="img" aria-label={label}>
	{#each paths as d}
		<path {d} />
	{/each}
</svg>

<style>
	svg {
		display: block;
		width: 100%;
		height: 100%;
		fill: none;
		stroke: currentColor;
		stroke-width: 1.5;
		stroke-linecap: round;
		stroke-linejoin: round;
	}
</style>
