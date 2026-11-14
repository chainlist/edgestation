<script lang="ts" module>
	/**
	 * Returns the color of a detail icon with hue `hue`. The mix contains 25 % of the text color.
	 * So the color is lighter on a dark theme and darker on a ligth theme, and is clear on both.
	 */
	const tint = (hue: number) =>
		`color-mix(in oklch, var(--card-foreground) 25%, oklch(0.7 0.18 ${hue}))`;

	/** The hues of the WHO colors for each UV index level (green, yellow, orange, red, violet). */
	const UV_HUES: Record<string, number> = {
		Low: 145,
		Moderate: 95,
		High: 55,
		'Very high': 25,
		Extreme: 320
	};
</script>

<script lang="ts">
	import Icon from './Icon.svelte';
	import Now from './Now.svelte';
	import type { Forecast } from './forecast.ts';
	import { compass, degrees, unit, uvLevel } from './format.ts';
	import { ICONS } from './icons.ts';
	import { condition } from './weather-codes.ts';

	let {
		forecast,
		imperial,
		hourStyle
	}: {
		forecast: Forecast;
		imperial: boolean;
		/** The hour format of the `hours` setting. */
		hourStyle: Intl.DateTimeFormatOptions;
	} = $props();

	const { current, today } = $derived(forecast);
	const now = $derived(condition(current.code, current.day));
	const time = $derived(
		new Intl.DateTimeFormat(undefined, {
			...hourStyle,
			minute: '2-digit',
			timeZone: forecast.timeZone
		})
	);
	// svelte-ignore state_referenced_locally
	const speed = unit(imperial ? 'mile-per-hour' : 'kilometer-per-hour');
	const percent = unit('percent');

	const uv = $derived(today.uv == null ? undefined : uvLevel(today.uv));

	// Each detail shows its icon in its own color. `label` gives its name to screen readers.
	const details = $derived([
		{
			label: 'Feels like',
			icon: ICONS.thermometer,
			color: tint(45),
			value: degrees(current.feelsLike)
		},
		{
			label: 'Wind',
			icon: ICONS.wind,
			color: tint(190),
			value: speed.format(current.windSpeed),
			note: compass(current.windDirection)
		},
		{
			label: 'Humidity',
			icon: ICONS.droplet,
			color: tint(240),
			value: percent.format(current.humidity)
		},
		{
			label: 'UV index',
			icon: ICONS.sun,
			color: uv && tint(UV_HUES[uv]),
			value: today.uv == null ? 'Unavailable' : String(Math.round(today.uv)),
			note: uv
		},
		{
			label: 'Sunrise',
			icon: ICONS.sunrise,
			color: tint(80),
			value: today.sunrise == null ? 'None' : time.format(today.sunrise)
		},
		{
			label: 'Sunset',
			icon: ICONS.sunset,
			color: tint(20),
			value: today.sunset == null ? 'None' : time.format(today.sunset)
		}
	]);
</script>

<div class="today">
	<div class="summary">
		<div class="lead"><Now {current} /></div>
		<p class="place">{forecast.place}</p>
		<p class="condition">{now.label}</p>
	</div>
	<dl class="details">
		{#each details as { label, icon, color, value, note } (label)}
			<div class="detail">
				<dt style:color><Icon paths={icon} {label} /></dt>
				<dd>
					{value}{#if note}<span class="note">{note}</span>{/if}
				</dd>
			</div>
		{/each}
	</dl>
</div>

<style>
	/* The summary is above the details in a square or tall cell, and next to them in a wide one. */
	.today {
		display: grid;
		grid-template-rows: auto minmax(0, 1fr);
		gap: 4cqh;
		height: 100%;
		padding: min(6cqh, 6cqw);
	}

	.summary {
		--now-gap: 2cqw;
		--now-icon: min(16cqh, 14cqw);
		--now-text: min(16cqh, 14cqw);
		display: grid;
		grid-template-columns: auto minmax(0, 1fr);
		column-gap: 4cqw;
		align-items: center;
	}

	.lead {
		grid-row: span 2;
	}

	p {
		margin: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.place {
		font-size: min(7cqh, 6cqw);
		font-weight: 600;
	}

	.condition {
		font-size: min(6cqh, 5cqw);
		color: var(--muted-foreground);
	}

	.details {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		grid-auto-rows: 1fr;
		gap: 3cqh 4cqw;
		margin: 0;
	}

	.detail {
		display: flex;
		flex-direction: column;
		justify-content: center;
		min-width: 0;
	}

	/* The `dt` holds the icon, not a text label. */
	dt {
		width: min(9cqh, 8cqw);
		height: min(9cqh, 8cqw);
		margin-bottom: 1.5cqh;
		color: var(--muted-foreground);
	}

	dd {
		margin: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		font-size: min(8cqh, 7cqw);
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	.note {
		margin-left: 0.4em;
		font-size: 0.7em;
		font-weight: 500;
		color: var(--muted-foreground);
	}

	@container (aspect-ratio > 3 / 2) {
		.today {
			grid-template-rows: none;
			grid-template-columns: minmax(0, 1fr) minmax(0, 2fr);
			gap: 4cqw;
		}

		.summary {
			--now-icon: min(26cqh, 10cqw);
			--now-text: min(26cqh, 10cqw);
			grid-template-columns: minmax(0, 1fr);
			align-content: center;
			row-gap: 2cqh;
		}

		.lead {
			grid-row: auto;
			margin-bottom: 2cqh;
		}

		.place {
			font-size: min(9cqh, 3.6cqw);
		}

		.condition {
			font-size: min(7cqh, 2.8cqw);
		}

		.details {
			grid-template-columns: repeat(3, minmax(0, 1fr));
			column-gap: 3cqw;
		}

		dt {
			width: min(12cqh, 4cqw);
			height: min(12cqh, 4cqw);
		}

		dd {
			font-size: min(10cqh, 3.6cqw);
		}
	}
</style>
