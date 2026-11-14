<script lang="ts">
	import Glyph from './Glyph.svelte';

	let {
		name,
		date,
		isToday,
		earlier,
		later,
		today,
		onshow
	}: {
		name: string;
		date?: string;
		isToday: boolean;
		earlier?: string;
		later?: string;
		today: string;
		onshow: (day: string) => void;
	} = $props();
</script>

<header>
	<button
		class="step"
		aria-label="Previous day"
		title="Previous day"
		disabled={!earlier}
		onclick={() => earlier && onshow(earlier)}
	>
		<Glyph name="previous" />
	</button>
	<button
		class="day"
		title={isToday ? undefined : 'Back to today'}
		disabled={isToday}
		onclick={() => onshow(today)}
	>
		<span class="name">{name}</span>
		{#if date}<span class="date">{date}</span>{/if}
	</button>
	<button
		class="step"
		aria-label="Next day"
		title="Next day"
		disabled={!later}
		onclick={() => later && onshow(later)}
	>
		<Glyph name="next" />
	</button>
</header>

<style>
	/* The day selector has arrows on the two sides of the day name. */
	header {
		display: flex;
		align-items: center;
		gap: 0.25em;
	}

	.step {
		display: grid;
		flex: none;
		place-items: center;
		width: 2.2em;
		height: 2.2em;
		border-radius: 50%;
	}

	.step :global(svg) {
		width: 55%;
	}

	.step:disabled {
		opacity: 0.3;
	}

	.step:active:not(:disabled),
	.day:active:not(:disabled) {
		background: var(--muted);
	}

	/* A narrow cell cuts the date first, then the name. The header dose not wrap. */
	.day {
		display: flex;
		flex: 1;
		align-items: baseline;
		justify-content: center;
		gap: 0.5em;
		min-width: 0;
		height: 2.2em;
		padding: 0 0.6em;
		border-radius: 999px;
		line-height: 2.2em;
		white-space: nowrap;
	}

	.name,
	.date {
		min-width: 0;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.name {
		flex: none;
		max-width: 100%;
		font-weight: 600;
	}

	.date {
		font-size: 0.8em;
		color: var(--muted-foreground);
	}
</style>
