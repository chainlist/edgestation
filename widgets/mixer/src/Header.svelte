<script lang="ts">
	import type { AudioMixer, AudioTarget } from '@edgestation/sdk';
	import Glyph from './Glyph.svelte';

	let {
		mixer,
		channel,
		openOutputs,
		toggle
	}: {
		mixer: AudioMixer;
		channel: string;
		openOutputs: () => void;
		toggle: (target: AudioTarget, muted: boolean) => void;
	} = $props();
</script>

<header>
	<button
		class="pill device"
		title={mixer.output?.name}
		disabled={!mixer.output}
		onclick={openOutputs}
	>
		<Glyph name="speaker" />
		<span>{mixer.output?.name ?? 'No sound output'}</span>
		{#if mixer.output}<Glyph name="chevron" />{/if}
	</button>
	{#if channel === 'all' && mixer.input}
		{@const muted = mixer.input.muted}
		<button
			class="pad mic"
			class:muted
			aria-pressed={muted}
			title={mixer.input.name}
			onclick={() => toggle('input', muted)}
		>
			<Glyph name={muted ? 'micOff' : 'mic'} />
			<span>{muted ? 'Mic muted' : 'Mic on'}</span>
		</button>
	{/if}
</header>

<style>
	/* In a narrow cell, the mic button goes below the pill. */
	header {
		display: flex;
		flex: none;
		flex-wrap: wrap;
		justify-content: space-between;
		gap: 0.5em;
		font-size: var(--controls);
	}

	.pill {
		display: flex;
		align-items: center;
		gap: 0.45em;
		min-width: 0;
		height: 2.1em;
		padding: 0 0.9em;
		border-radius: 999px;
		background: var(--secondary);
		color: var(--secondary-foreground);
		transition: transform 0.1s;
	}

	.pill span {
		overflow: hidden;
		white-space: nowrap;
		text-overflow: ellipsis;
	}

	.pill:not(:disabled):active {
		transform: scale(0.96);
	}

	.pill:disabled {
		cursor: default;
	}

	.device :global(svg:last-child) {
		opacity: 0.6;
	}

	/* The pad of the microphone in the header of the faders. */
	header .pad {
		display: flex;
		flex: none;
		align-items: center;
		gap: 0.45em;
		height: 2.1em;
		padding: 0 0.9em;
		font-weight: 600;
	}
</style>
