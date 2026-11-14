<script lang="ts">
	import Icon from './Icon.svelte';
	import type { Forecast } from './forecast.ts';
	import { degrees } from './format.ts';
	import { condition } from './weather-codes.ts';

	let { current }: { current: Forecast['current'] } = $props();
</script>

<!-- The weather icon and the temperature now. The parent sets the direction, the gap and the sizes
     with `--now-direction`, `--now-gap`, `--now-icon` and `--now-text`. -->
<div class="now">
	<span class="icon"><Icon {...condition(current.code, current.day)} /></span>
	<span class="temperature">{degrees(current.temperature)}</span>
</div>

<style>
	.now {
		display: flex;
		flex-direction: var(--now-direction, row);
		align-items: center;
		gap: var(--now-gap);
	}

	.icon {
		flex: none;
		width: var(--now-icon);
		height: var(--now-icon);
		color: var(--primary);
	}

	.temperature {
		font-size: var(--now-text);
		font-weight: 600;
		line-height: 1;
		font-variant-numeric: tabular-nums;
	}
</style>
