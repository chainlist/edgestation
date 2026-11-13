<script lang="ts">
	import type { ProgramUsage } from '@edgestation/sdk';
	import { percent } from './format.ts';

	let { top, programs }: { top: number; programs: ProgramUsage[] } = $props();
</script>

<!-- If the computer is idle, there can be fewer programs then rows. The other rows stay empty. -->
<ol class="programs">
	{#each Array.from({ length: top }, (_, i) => programs[i]) as program}
		<li>
			<span class="name">{program?.name.replace(/\.exe$/i, '') ?? ''}</span>
			<span>{program ? percent.format(program.cpu) : ''}</span>
		</li>
	{/each}
</ol>

<style>
	/* The busiest programs show over the top of the chart, because CPU load rarely gets there. */
	.programs {
		grid-area: 3 / 1;
		margin: 0;
		padding: 0;
		overflow: hidden;
		font-size: min(8cqh, 7cqw);
		line-height: 1.25;
		list-style: none;
		/* A halo in the card color keeps the text readable where the line goes behind it. */
		text-shadow:
			0 0 0.2em var(--card),
			0 0 0.2em var(--card);
	}

	.programs li {
		display: flex;
		gap: 0.5em;
		justify-content: space-between;
		font-variant-numeric: tabular-nums;
		white-space: nowrap;
	}

	.programs .name {
		min-width: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		color: var(--muted-foreground);
	}
</style>
