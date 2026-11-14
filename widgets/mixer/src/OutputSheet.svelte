<script lang="ts">
	import type { AudioMixer, AudioOutput, WidgetApi } from '@edgestation/sdk';
	import Glyph from './Glyph.svelte';

	let {
		api,
		outputs,
		mixer,
		close,
		pick
	}: {
		api: WidgetApi;
		outputs: AudioOutput[];
		mixer: AudioMixer;
		close: () => void;
		pick: (id: string) => void;
	} = $props();
</script>

<div class="sheet" role="dialog" aria-label="Sound output">
	<div class="sheet-head">
		<span>Sound output</span>
		<button class="close" aria-label="Close" onclick={close}>
			<Glyph name="close" />
		</button>
	</div>
	<div class="list">
		<ul {@attach (list) => api.scrollbars(list)}>
			{#each outputs as output (output.id)}
				<li>
					<button class:current={output.id === mixer.output?.id} onclick={() => pick(output.id)}>
						<Glyph name="check" />
						<span>{output.name}</span>
					</button>
				</li>
			{/each}
		</ul>
	</div>
</div>

<style>
	/* The sheet takes the place of the faders, so the glass of the card shows through it. */
	.sheet {
		display: flex;
		flex: 1;
		flex-direction: column;
		gap: 0.3em;
		min-height: 0;
		font-size: var(--controls);
	}

	.sheet-head {
		display: flex;
		flex: none;
		align-items: center;
		justify-content: space-between;
		padding-left: 0.4em;
		font-weight: 600;
	}

	.close {
		display: grid;
		place-items: center;
		width: 2.2em;
		height: 2.2em;
		border-radius: 50%;
		background: var(--secondary);
	}

	.list {
		flex: 1;
		min-height: 0;
	}

	ul {
		margin: 0;
		padding: 0;
		overscroll-behavior: contain;
		list-style: none;
	}

	li button {
		display: flex;
		align-items: center;
		gap: 0.5em;
		width: 100%;
		padding: 0.6em;
		border-radius: 0.5em;
		text-align: left;
	}

	li button:active {
		background: var(--secondary);
	}

	li button :global(svg) {
		visibility: hidden;
		color: var(--primary);
	}

	li button.current :global(svg) {
		visibility: visible;
	}

	/* Two lines, because many names differ only at the end, such as `Speakers (...)`. */
	li button span {
		display: -webkit-box;
		overflow: hidden;
		-webkit-box-orient: vertical;
		-webkit-line-clamp: 2;
		line-clamp: 2;
	}
</style>
