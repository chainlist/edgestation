<script lang="ts">
	import type { WidgetApi } from '@edgestation/sdk';
	import Band from './Band.svelte';
	import Now from './Now.svelte';
	import Today from './Today.svelte';
	import { findPlace, loadForecast, type Forecast, type Place } from './forecast.ts';
	import { hourOptions } from './format.ts';

	let props: { api: WidgetApi } = $props();
	// `api` does not change. A changed setting mounts the widget again.
	// svelte-ignore state_referenced_locally
	const { api } = props;
	const { show } = api.settings;
	const query = api.settings.city.trim();
	const imperial = api.settings.units === 'imperial';
	const hourStyle = hourOptions[api.settings.hours];

	/** After this age, the next tick gets a new forecast. Open-Meteo updates every 15 minutes. */
	const STALE = 15 * 60_000;

	/** `undefined` until the lookup ends. `null` if Open-Meteo does not know the city. */
	let place = $state.raw<Place | null>();
	let forecast = $state.raw<Forecast>();
	/** The last attempt failed. A forecast from before stays on the screen. */
	let offline = $state(false);
	let fetchedAt = 0;
	let busy = false;

	// Ticks are cheap. A tick gets the forecast only when it is old, or again after a failiure.
	const refresh = api.guard(async () => {
		if (!query || place === null || busy || Date.now() - fetchedAt < STALE) return;
		busy = true;
		try {
			await api.loading(update());
			offline = false;
		} catch (error) {
			console.warn('Weather:', error);
			offline = true;
		} finally {
			busy = false;
		}
	});
	api.every(60_000, refresh);
	refresh();

	// One promise for the lookup and the forecast. Thus the dots of the first load stay untill both
	// end.
	async function update() {
		place ??= await findPlace(api, query);
		if (place) {
			forecast = await loadForecast(api, place, imperial);
			fetchedAt = Date.now();
		}
	}
</script>

<!-- `el` is the size container, so its padding cannot use cqh or cqw. Put the layout in it. -->
{#if !query}
	<div class="message">
		<p class="title">No city yet</p>
		<p class="hint">Set its city in Settings → Pages, by clicking this widget.</p>
	</div>
{:else if place === null}
	<div class="message">
		<p class="title">City not found</p>
		<p class="hint">{query}</p>
	</div>
{:else if !forecast}
	<!-- Until the first forecast comes, the app shows dots for `api.loading`. -->
	{#if offline}
		<div class="message">
			<p class="title">No weather</p>
			<p class="hint">Open-Meteo can't be reached. Trying again every minute.</p>
		</div>
	{/if}
{:else if show === 'today'}
	<Today {forecast} {imperial} {hourStyle} />
{:else if show === 'current'}
	<div class="current">
		<Now current={forecast.current} />
		<p class="place">{forecast.place}</p>
	</div>
{:else}
	<Band {forecast} {hourStyle} />
{/if}

<style>
	.message,
	.current {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 3cqh;
		height: 100%;
		padding: 0 6cqw;
		text-align: center;
	}

	.message p,
	.place {
		max-width: 100%;
		margin: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.title {
		font-size: min(8cqh, 6cqw);
		font-weight: 600;
		color: var(--muted-foreground);
	}

	.message .hint {
		font-size: min(5cqh, 4cqw);
		white-space: normal;
		color: var(--muted-foreground);
	}

	/* The icon is above the temperature in a square or tall cell, and next to it in a wide cell. */
	.current {
		--now-direction: column;
		--now-gap: 2cqh;
		--now-icon: min(36cqh, 40cqw);
		--now-text: min(30cqh, 30cqw);
	}

	.place {
		font-size: min(8cqh, 7cqw);
		color: var(--muted-foreground);
	}

	@container (aspect-ratio > 4 / 3) {
		.current {
			--now-direction: row;
			--now-gap: 4cqw;
			--now-icon: min(48cqh, 24cqw);
			--now-text: min(44cqh, 20cqw);
		}
	}
</style>
