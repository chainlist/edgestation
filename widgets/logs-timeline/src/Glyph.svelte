<script lang="ts" module>
	/** Icon paths on a 24×24 box. The icons use a stroke, not a fill. */
	const GLYPHS = {
		previous: 'M15 5l-7 7 7 7',
		next: 'M9 5l7 7-7 7',
		add: 'M12 5v14M5 12h14',
		save: 'M5 12.5l4.5 4.5L19 7.5',
		cancel: 'M6 6l12 12M18 6L6 18',
		edit: 'M4 20h4L19 9a2.83 2.83 0 0 0-4-4L4 16v4M13.5 6.5l4 4',
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
