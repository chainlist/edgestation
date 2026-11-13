<script lang="ts" module>
	/** Icon paths on a 24×24 box. The icons use a stroke, not a fill. */
	const GLYPHS = {
		add: 'M12 5v14M5 12h14',
		back: 'M15 5l-7 7 7 7',
		remove: 'M4 7h16M10 7V4h4v3M6 7l1 13h10l1-13'
	};
</script>

<script lang="ts">
	let { name }: { name: keyof typeof GLYPHS } = $props();
</script>

<svg viewBox="0 0 24 24" aria-hidden="true"><path d={GLYPHS[name]} /></svg>

<style>
	svg {
		fill: none;
		stroke: currentColor;
		stroke-width: 2.5;
		stroke-linecap: round;
		stroke-linejoin: round;
	}
</style>
