<script lang="ts" module>
	/** Hour options for each value of the `hours` setting. `auto` uses the language format. */
	const hourOptions: Record<string, Intl.DateTimeFormatOptions> = {
		auto: { hour: 'numeric' },
		'12': { hour: 'numeric', hourCycle: 'h12' },
		// Two digits, as 24-hour clocks usualy show the hour: 09:05.
		'24': { hour: '2-digit', hourCycle: 'h23' }
	};

	/** Options for each `date` style that `Intl` formats. */
	const dateOptions: Record<string, Intl.DateTimeFormatOptions> = {
		long: { weekday: 'long', day: 'numeric', month: 'long' },
		full: { weekday: 'long', day: 'numeric', month: 'long', year: 'numeric' },
		short: { weekday: 'short', day: 'numeric', month: 'short' },
		dayMonth: { day: 'numeric', month: 'long' },
		weekday: { weekday: 'long' },
		numeric: { day: '2-digit', month: '2-digit', year: 'numeric' }
	};

	const pad = (n: number) => String(n).padStart(2, '0');

	/** Returns a formatter for `style` of the `date` setting, or `undefined` to hide the date. */
	function dateFormatter(style: string): ((date: Date) => string) | undefined {
		if (style === 'none') return undefined;
		if (style === 'iso') {
			return (d) => `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
		}
		const format = new Intl.DateTimeFormat(undefined, dateOptions[style]);
		return (d) => format.format(d);
	}
</script>

<script lang="ts">
	import type { WidgetApi } from '@edgestation/sdk';

	let props: { api: WidgetApi } = $props();
	// `api` does not change. A changed setting mounts the widget again.
	// svelte-ignore state_referenced_locally
	const { api } = props;
	const timeFormat = new Intl.DateTimeFormat(undefined, {
		...hourOptions[api.settings.hours],
		minute: '2-digit'
	});
	const formatDate = dateFormatter(api.settings.date);

	let now = $state(new Date());
	api.every(1000, () => (now = new Date()));

	// Use parts, so that AM/PM can be smaller than the digits. Some languages put AM/PM first.
	const timeParts = $derived(timeFormat.formatToParts(now));
	// AM/PM and the space next to it. At the size of the digits, the space is to wide.
	const small = (part: Intl.DateTimeFormatPart) => part.type === 'dayPeriod' || !part.value.trim();
</script>

<div class="clock">
	<time class="time">
		{#each timeParts as part, i (i)}
			<span class:period={small(part)}>{part.value}</span>
		{/each}
	</time>
	{#if formatDate}
		<p class="date">{formatDate(now)}</p>
	{/if}
</div>

<style>
	/* cqh and cqw are relative to the cell of the widget, so the clock grows with the cell. */
	.clock {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 3cqh;
		height: 100%;
	}

	.time {
		font-size: min(40cqh, 17cqw);
		white-space: nowrap;
		font-weight: 600;
		line-height: 1;
		font-variant-numeric: tabular-nums;
		color: var(--primary);
	}

	.period {
		font-size: 0.4em;
		font-weight: 500;
	}

	.date {
		margin: 0;
		font-size: min(11cqh, 7cqw);
		color: var(--muted-foreground);
	}
</style>
