<script lang="ts" module>
	/** Hour steps between two labels. The band uses the smallest that gives each label space. */
	const STEPS = [1, 2, 3, 4, 6, 8, 12];
	/** The space for one label, as a fraction of the band height. Label sizes are in cqh. */
	const LABEL_ROOM = 0.3;

	/** Positions of the curve and the bars on the 0..100 heigth of the plot, from the top. */
	const CURVE_TOP = 26;
	const CURVE_BOTTOM = 64;
	/** The bar height for a 100 % chance of rain, from the bottom. */
	const BARS = 30;
	/** The minimum chance of rain in percent that shows a label. */
	const RAIN_LABEL = 20;

	/**
	 * Returns an SVG path through `ys`, with one point for each x from 0. The path is a monotone
	 * cubic curve (the Fritsch-Carlson method). It is smooth, and between two points it does not
	 * go above or below them. So it shows no extremes that are not in the forecast.
	 */
	function curve(ys: number[]) {
		const n = ys.length;
		const slopes = ys.slice(1).map((y, i) => y - ys[i]);
		const tangents = ys.map((_, i) => {
			if (i === 0) return slopes[0];
			if (i === n - 1) return slopes[n - 2];
			return slopes[i - 1] * slopes[i] <= 0 ? 0 : (slopes[i - 1] + slopes[i]) / 2;
		});
		for (let i = 0; i < n - 1; i++) {
			if (slopes[i] === 0) {
				tangents[i] = tangents[i + 1] = 0;
				continue;
			}
			const a = tangents[i] / slopes[i];
			const b = tangents[i + 1] / slopes[i];
			const length = Math.hypot(a, b);
			if (length > 3) {
				tangents[i] = (3 / length) * a * slopes[i];
				tangents[i + 1] = (3 / length) * b * slopes[i];
			}
		}
		const f = (v: number) => +v.toFixed(2);
		let path = `M0 ${f(ys[0])}`;
		for (let i = 0; i < n - 1; i++) {
			path +=
				`C${f(i + 1 / 3)} ${f(ys[i] + tangents[i] / 3)} ` +
				`${f(i + 2 / 3)} ${f(ys[i + 1] - tangents[i + 1] / 3)} ${i + 1} ${f(ys[i + 1])}`;
		}
		return path;
	}
</script>

<script lang="ts">
	import Icon from './Icon.svelte';
	import type { Forecast } from './forecast.ts';
	import { degrees } from './format.ts';
	import { condition } from './weather-codes.ts';

	let {
		forecast,
		hourStyle
	}: {
		forecast: Forecast;
		/** The hour format of the `hours` setting. */
		hourStyle: Intl.DateTimeFormatOptions;
	} = $props();

	/** The width of the plot (the band without its side margins) and the height of the band. */
	let width = $state(0);
	let height = $state(0);

	const hours = $derived(forecast.hours);
	const last = $derived(hours.length - 1);
	const hourFormat = $derived(
		new Intl.DateTimeFormat(undefined, { ...hourStyle, timeZone: forecast.timeZone })
	);

	/**
	 * Temperatures at the top and the bottom of the curve. They are a minimum of 6 degrees apart,
	 * so an flat day looks flat.
	 */
	const range = $derived.by(() => {
		const temperatures = hours.map((hour) => hour.temperature);
		const [low, high] = [Math.min(...temperatures), Math.max(...temperatures)];
		const pad = Math.max(0, 6 - (high - low)) / 2;
		return { low: low - pad, high: high + pad };
	});
	const ys = $derived(
		hours.map(
			(hour) =>
				CURVE_BOTTOM -
				((hour.temperature - range.low) / (range.high - range.low)) * (CURVE_BOTTOM - CURVE_TOP)
		)
	);
	const line = $derived(curve(ys));

	const step = $derived(
		STEPS.find((hoursApart) => (hoursApart * width) / last >= height * LABEL_ROOM) ?? 12
	);
	const labelled = $derived(hours.flatMap((hour, i) => (i % step ? [] : [{ hour, i }])));
	const x = (i: number) => `${(i / last) * 100}%`;
</script>

<div class="band" bind:clientHeight={height}>
	<div class="row icons">
		{#each labelled as { hour, i } (hour.time)}
			<div class="label" style:left={x(i)}>
				<span class="icon"><Icon {...condition(hour.code, hour.day)} /></span>
				{#if hour.rain != null && hour.rain >= RAIN_LABEL}
					<span class="rain">{hour.rain}%</span>
				{/if}
			</div>
		{/each}
	</div>

	<div class="plot" bind:clientWidth={width}>
		<svg viewBox="0 0 {last} 100" preserveAspectRatio="none" aria-hidden="true">
			<defs>
				<linearGradient id="fade" x1="0" y1="0" x2="0" y2="1">
					<stop offset={CURVE_TOP / 100} stop-color="currentColor" stop-opacity="0.25" />
					<stop offset="1" stop-color="currentColor" stop-opacity="0" />
				</linearGradient>
			</defs>
			<path class="area" d="{line}L{last} 100L0 100Z" />
			{#each hours as hour, i (hour.time)}
				{#if hour.rain}
					<rect
						class="bar"
						x={i - 0.3}
						y={100 - (hour.rain * BARS) / 100}
						width="0.6"
						height={(hour.rain * BARS) / 100}
					/>
				{/if}
			{/each}
			<path class="line" d={line} />
		</svg>
		{#each labelled as { hour, i } (hour.time)}
			<span class="temperature" style:left={x(i)} style:top="{ys[i]}%">
				{degrees(hour.temperature)}
			</span>
		{/each}
	</div>

	<div class="row hours">
		{#each labelled as { hour, i } (hour.time)}
			<span class="label" style:left={x(i)}>{i ? hourFormat.format(hour.time) : 'Now'}</span>
		{/each}
	</div>
</div>

<style>
	/* cqh and cqw are relative to the cell of the widget, so the band grows with the cell. The
	   side margins give space for half a label at each end. */
	.band {
		display: grid;
		grid-template-rows: 22cqh minmax(0, 1fr) 9cqh;
		height: 100%;
		padding: 5cqh 12cqh 4cqh;
	}

	.row,
	.plot {
		position: relative;
	}

	.label,
	.temperature {
		position: absolute;
		white-space: nowrap;
		transform: translateX(-50%);
	}

	.icons .label {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.5cqh;
	}

	.icon {
		width: 13cqh;
		height: 13cqh;
	}

	.rain {
		font-size: 6cqh;
		font-weight: 600;
		color: var(--chart-2);
	}

	svg {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		overflow: visible;
		color: var(--chart-1);
	}

	.line {
		fill: none;
		stroke: currentColor;
		stroke-width: 3px;
		stroke-linejoin: round;
		vector-effect: non-scaling-stroke;
	}

	.area {
		fill: url(#fade);
	}

	.bar {
		fill: var(--chart-2);
		opacity: 0.7;
	}

	/* Put the temperature above its point on the curve. */
	.temperature {
		font-size: 8cqh;
		font-weight: 600;
		line-height: 1;
		font-variant-numeric: tabular-nums;
		transform: translate(-50%, calc(-100% - 2cqh));
	}

	.hours .label {
		bottom: 0;
		font-size: 6.5cqh;
		color: var(--muted-foreground);
	}
</style>
