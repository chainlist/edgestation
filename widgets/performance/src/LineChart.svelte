<script lang="ts" module>
	import { HISTORY } from './charts.ts';

	/** The SVG path of `values` (one each second, newest last) on a 0..HISTORY-1 × 0..100 box. */
	function linePath(values: (number | undefined)[], max: number) {
		const offset = HISTORY - values.length;
		return values
			.map((v, i) => `${i ? 'L' : 'M'}${offset + i} ${100 - Math.min(1, (v ?? 0) / max) * 100}`)
			.join('');
	}
</script>

<script lang="ts">
	let { history, max }: { history: (number | undefined)[][]; max: number } = $props();
</script>

<svg class="chart" viewBox="0 0 {HISTORY - 1} 100" preserveAspectRatio="none" aria-hidden="true">
	{#each history as values, i}
		{@const line = values.at(-1) == null ? '' : linePath(values, max)}
		<path
			class="area s{i}"
			d={line && `${line}L${HISTORY - 1} 100L${HISTORY - values.length} 100Z`}
		/>
		<path class="line s{i}" d={line} />
	{/each}
</svg>

<style>
	.chart {
		width: 100%;
		height: 100%;
		overflow: visible;
		grid-area: 3 / 1;
	}

	.line {
		fill: none;
		stroke: currentColor;
		stroke-width: 2px;
		stroke-linejoin: round;
		vector-effect: non-scaling-stroke;
	}

	.area {
		fill: currentColor;
		opacity: 0.15;
	}
</style>
