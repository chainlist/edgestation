<script lang="ts">
	let { level, rows }: { level: number; rows: boolean } = $props();
</script>

<div class="meter" class:rows style:--level={level}>
	<div class="window"><div class="lit"></div></div>
</div>

<style>
	/* The meter is a column of lights in the accent color, with red lights at the top. The lit
	   lights are drawn once and then only move. A window slides over them when the level changes,
	   and they slide back inside it to stay in place. The compositor does the moves, so the meter
	   does not repaint. */
	.meter {
		position: relative;
		width: 0.4em;
		margin: calc(var(--knob-along) / 2) 0;
		overflow: hidden;
		--lights: repeating-linear-gradient(to top, #000 0 0.3em, transparent 0.3em 0.42em);
	}

	.meter::before,
	.window,
	.lit {
		position: absolute;
		inset: 0;
	}

	.meter::before,
	.lit {
		mask: var(--lights);
	}

	.meter::before {
		background: var(--muted);
		content: '';
	}

	.window {
		overflow: hidden;
		transform: translateY(calc((1 - var(--level)) * 100%));
		will-change: transform;
	}

	.lit {
		background: linear-gradient(to top, var(--primary) 0 80%, var(--destructive) 80%);
		transform: translateY(calc((var(--level) - 1) * 100%));
		will-change: transform;
	}

	.meter.rows {
		width: auto;
		height: 0.35em;
		margin: 0 calc(var(--knob-along) / 2);
		--lights: repeating-linear-gradient(to right, #000 0 0.3em, transparent 0.3em 0.42em);
	}

	.rows .window {
		transform: translateX(calc((var(--level) - 1) * 100%));
	}

	.rows .lit {
		background: linear-gradient(to right, var(--primary) 0 80%, var(--destructive) 80%);
		transform: translateX(calc((1 - var(--level)) * 100%));
	}
</style>
