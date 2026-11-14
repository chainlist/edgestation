<script lang="ts">
	import type { MediaAction, MediaInfo } from '@edgestation/sdk';
	import Glyph from './Glyph.svelte';

	let {
		now,
		playing,
		send
	}: { now: MediaInfo | null; playing: boolean; send: (action: MediaAction) => void } = $props();
</script>

<div class="controls" class:idle={!now}>
	<button aria-label="Previous" disabled={!now?.can.previous} onclick={() => send('previous')}>
		<Glyph name="previous" />
	</button>
	<button
		class="toggle"
		aria-label={playing ? 'Pause' : 'Play'}
		disabled={!(playing ? now?.can.pause : now?.can.play)}
		onclick={() => send(playing ? 'pause' : 'play')}
	>
		<Glyph name={playing ? 'pause' : 'play'} />
	</button>
	<button aria-label="Next" disabled={!now?.can.next} onclick={() => send('next')}>
		<Glyph name="next" />
	</button>
</div>

<style>
	.controls {
		display: flex;
		align-items: center;
		gap: min(4cqh, 2.5cqw);
		margin-top: min(2cqh, 1.5cqw);
	}

	button {
		display: grid;
		place-items: center;
		width: min(20cqh, 11cqw);
		aspect-ratio: 1;
		border-radius: 50%;
	}

	button :global(svg) {
		width: 55%;
	}

	.toggle {
		width: min(26cqh, 14cqw);
		background: var(--primary);
		color: var(--primary-foreground);
	}

	button:disabled {
		opacity: 0.35;
		cursor: default;
	}

	.controls.idle {
		display: none;
	}

	@container (aspect-ratio < 1.4) {
		.controls {
			justify-content: center;
		}

		button {
			width: min(13cqh, 16cqw);
		}

		.toggle {
			width: min(17cqh, 21cqw);
		}
	}
</style>
