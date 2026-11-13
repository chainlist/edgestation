<script lang="ts">
	import type { AudioTarget } from '@edgestation/sdk';
	import Glyph from './Glyph.svelte';
	import PadBody from './PadBody.svelte';
	import PadGrid from './PadGrid.svelte';
	import type { Strip } from './strip.ts';

	let {
		strips,
		toggle,
		broken = $bindable()
	}: {
		strips: Strip[];
		toggle: (target: AudioTarget, muted: boolean) => void;
		broken: Record<string, boolean>;
	} = $props();
</script>

<PadGrid count={strips.length}>
	{#each strips as { target, name, muted, src, fallback, off } (target || name)}
		<button
			class="pad"
			class:muted
			class:off
			disabled={off}
			aria-label={muted ? `Unmute ${name}` : `Mute ${name}`}
			aria-pressed={muted}
			onclick={() => toggle(target, muted)}
		>
			<PadBody {name} detail={off ? 'Not running' : muted ? 'Muted' : 'On'}>
				{#if src && !broken[target]}
					<img alt="" {src} onerror={() => (broken = { ...broken, [target]: true })} />
				{:else}
					<Glyph name={muted ? (fallback === 'mic' ? 'micOff' : 'speakerOff') : fallback} />
				{/if}
			</PadBody>
		</button>
	{/each}
</PadGrid>

<style>
	.pad.muted img,
	.pad.off img {
		opacity: 0.5;
		filter: grayscale(1);
	}
</style>
