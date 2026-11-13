<script lang="ts">
	import type { Snippet } from 'svelte';

	let { count, children }: { count: number; children: Snippet } = $props();

	let width = $state(0);
	let height = $state(0);
	/** The number of columns that gives the largest sqaure pads in the cell. */
	const columns = $derived.by(() => {
		const total = Math.max(1, count);
		let best = { columns: 1, side: 0 };
		for (let columns = 1; columns <= total; columns++) {
			const side = Math.min(width / columns, height / Math.ceil(total / columns));
			if (side > best.side) best = { columns, side };
		}
		return best.columns;
	});
</script>

<div
	class="pads"
	style:grid-template-columns="repeat({columns}, minmax(0, 1fr))"
	bind:clientWidth={width}
	bind:clientHeight={height}
>
	{@render children()}
</div>

<style>
	/* A grid of pads that fills the cell. */
	.pads {
		display: grid;
		grid-auto-rows: minmax(0, 1fr);
		gap: 0.4em;
		height: 100%;
	}

	/* Each pad is the size container of its content, so the content sizes use cqh and cqw of
	   the pad. */
	.pads > :global(.pad) {
		min-width: 0;
		border-radius: 0.6em;
		container-type: size;
	}
</style>
