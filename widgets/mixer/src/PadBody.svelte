<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		name,
		detail,
		children
	}: {
		name: string;
		/** The small line below the name. */
		detail: string;
		/** The icon. */
		children: Snippet;
	} = $props();
</script>

<span class="pad-body">
	<span class="pad-icon">{@render children()}</span>
	<span class="pad-name">{name}</span>
	{#if detail}<span class="pad-state">{detail}</span>{/if}
</span>

<style>
	.pad-body {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 4cqh;
		height: 100%;
		padding: 6cqh 6cqw;
	}

	.pad-icon {
		display: grid;
		place-items: center;
		width: min(34cqh, 40cqw);
		height: min(34cqh, 40cqw);
	}

	.pad-icon > :global(*) {
		width: 100%;
		height: 100%;
		object-fit: contain;
	}

	.pad-name,
	.pad-state {
		max-width: 100%;
		overflow: hidden;
		white-space: nowrap;
		text-overflow: ellipsis;
	}

	.pad-name {
		font-size: min(13cqh, 9cqw);
		font-weight: 700;
		letter-spacing: 0.06em;
		text-transform: uppercase;
	}

	.pad-state {
		font-size: min(10cqh, 7.5cqw);
		opacity: 0.85;
	}
</style>
