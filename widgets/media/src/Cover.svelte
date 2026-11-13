<script lang="ts">
	import Glyph from './Glyph.svelte';

	let {
		art,
		count,
		index,
		onstep
	}: {
		art: string | null;
		/** The number of sessions, and the index of the shown one. */
		count: number;
		index: number;
		/** Shows the next session. */
		onstep: () => void;
	} = $props();
</script>

<div class="cover">
	<div class="art">
		<Glyph name="note" />
		{#if art}<img alt="" src={art} />{/if}
		<!-- With more sessions, a tap on the art shows the next one. The dots tell wich one shows. -->
		{#if count > 1}
			<button class="step" aria-label="Next session ({index + 1} of {count})" onclick={onstep}>
				<span class="dots">
					{#each { length: count }, i (i)}
						<span class="dot" class:active={i === index}></span>
					{/each}
				</span>
			</button>
		{/if}
	</div>
</div>

<style>
	/* The space for the art. The art is the largest square wich fits in it. */
	.cover {
		display: grid;
		flex: none;
		place-items: center;
		height: 100%;
		aspect-ratio: 1;
		container-type: size;
	}

	.art {
		position: relative;
		display: grid;
		place-items: center;
		width: min(100cqw, 100cqh);
		height: min(100cqw, 100cqh);
		overflow: hidden;
		border-radius: calc(var(--radius) * 0.75);
		background: var(--muted);
		color: var(--muted-foreground);
		transition: transform 0.1s;
	}

	.art :global(svg) {
		width: 40%;
		fill: currentColor;
		opacity: 0.6;
	}

	.art img {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.art:has(.step:active) {
		transform: scale(0.95);
	}

	.step {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: flex-end;
		justify-content: center;
		padding: 0 0 8% 0;
		border: none;
		background: transparent;
		cursor: pointer;
	}

	/* The art under the dots can be of any color, so they sit on a small pill. */
	.dots {
		display: flex;
		gap: max(3px, 3cqmin);
		padding: max(3px, 3cqmin) max(5px, 5cqmin);
		border-radius: 999px;
		background: color-mix(in oklch, var(--card) 70%, transparent);
		backdrop-filter: blur(6px);
	}

	.dot {
		width: max(4px, 5cqmin);
		aspect-ratio: 1;
		border-radius: 50%;
		background: color-mix(in oklch, var(--card-foreground) 40%, transparent);
	}

	.dot.active {
		background: var(--primary);
	}

	/* In square and tall cells, the art fills the height that the text does not use. */
	@container (aspect-ratio < 1.4) {
		.cover {
			flex: 1 1 0;
			height: auto;
			min-height: 0;
			aspect-ratio: auto;
		}
	}
</style>
