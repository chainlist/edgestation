<script lang="ts">
	import type { ProgramUsage, SystemInfo, WidgetApi } from '@edgestation/sdk';
	import { HISTORY, numeric } from './charts.ts';
	import LineChart from './LineChart.svelte';
	import Programs from './Programs.svelte';

	let props: { api: WidgetApi } = $props();
	// `api` does not change. A changed setting mounts the widget again.
	// svelte-ignore state_referenced_locally
	const { api } = props;
	const { metric } = api.settings;
	const chart = numeric(api.settings);

	/** Readings after the widget mounted, one each second, newest last. */
	let readings = $state.raw<SystemInfo[]>([]);
	let programs = $state.raw<ProgramUsage[]>([]);
	const latest = $derived(readings.at(-1));
	/** The values of each series, in the same order as `readings`. */
	const history = $derived(chart?.series.map((s) => readings.map(s.read)) ?? []);
	const max = $derived(
		chart?.max ?? Math.max(chart?.min ?? 0, ...history.flat().map((v) => v ?? 0)) * 1.15
	);

	const refresh = api.guard(async () => {
		const [reading, top] = await Promise.all([
			api.system(),
			chart?.top ? api.topPrograms(chart.top) : []
		]);
		readings = [...readings.slice(1 - HISTORY), reading];
		programs = top;
	});
	api.every(1000, refresh);
	refresh();
</script>

<!-- `el` is the size container, so its padding cannot use cqh or cqw. Put the layout in it. -->
{#if chart}
	<div class="body" class:net={metric === 'net'} class:both={chart.series.length > 1}>
		<p class="label">{chart.label}</p>
		<div class="values">
			{#each chart.series as series, i}
				{@const value = history[i].at(-1)}
				<p class="value s{i}">
					{#if series.icon}<span class="icon">{series.icon}</span>{/if}<span
						class="number"
						class:unavailable={latest && value == null}
						>{!latest ? '…' : value == null ? 'Unavailable' : chart.format(value)}</span
					>
				</p>
			{/each}
		</div>
		<LineChart {history} {max} />
		{#if chart.top}
			<Programs top={chart.top} {programs} />
		{/if}
	</div>
{:else}
	<div class="body text">
		<p class="label">{metric === 'wifi' ? 'Wi-Fi' : 'Machine name'}</p>
		<p class="value">
			{!latest ? '…' : metric === 'wifi' ? (latest.wifi ?? 'Not connected') : latest.hostname}
		</p>
	</div>
{/if}

<style>
	/* cqh and cqw are relative to the cell of the widget, so all parts grow with the cell. */
	.body {
		display: grid;
		grid-template-rows: auto auto minmax(0, 1fr);
		gap: 3cqh;
		height: 100%;
		padding: min(7cqh, 7cqw);
	}

	.label {
		margin: 0;
		overflow: hidden;
		font-size: min(8cqh, 7cqw);
		white-space: nowrap;
		text-overflow: ellipsis;
		color: var(--muted-foreground);
	}

	.values {
		display: flex;
		flex-wrap: wrap;
		column-gap: 1em;
	}

	.value {
		margin: 0;
		font-size: min(26cqh, 21cqw);
		font-weight: 600;
		line-height: 1.05;
		white-space: nowrap;
		font-variant-numeric: tabular-nums;
	}

	/* Speeds have long units ("Mbit/s" in some languages). A smaler font lets them fit in a
	   1×1 cell. */
	.net .value {
		font-size: min(20cqh, 14cqw);
	}

	.both .value {
		font-size: min(14cqh, 10.5cqw);
	}

	.icon {
		margin-right: 0.15em;
	}

	.unavailable {
		font-size: 0.5em;
		font-weight: 500;
		color: var(--muted-foreground);
	}

	/* Series colors. The chart uses the same colors, so each arrow has the color of its line. */
	:where(.body) :global(.s0) {
		color: var(--chart-1);
	}

	:where(.body) :global(.s1) {
		color: var(--chart-2);
	}

	.number {
		color: var(--card-foreground);
	}

	/* Wi-Fi network and machine name show only text, in the vertical center. */
	.text {
		grid-template-rows: auto auto;
		align-content: center;
	}

	.text .value {
		display: -webkit-box;
		overflow: hidden;
		font-size: min(14cqh, 11cqw);
		white-space: normal;
		overflow-wrap: anywhere;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
	}
</style>
