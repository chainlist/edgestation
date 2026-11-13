<script lang="ts">
	import Glyph from './Glyph.svelte';

	let { art }: { art: string | null } = $props();
</script>

<div class="cover">
	<div class="art">
		<Glyph name="note" />
		{#if art}<img alt="" src={art} />{/if}
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
