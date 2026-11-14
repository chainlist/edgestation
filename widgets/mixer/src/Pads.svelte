<script lang="ts">
	import type { AudioTarget } from '@edgestation/sdk';
	import ChannelIcon from './ChannelIcon.svelte';
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
				<ChannelIcon {target} {src} {fallback} {muted} bind:broken />
			</PadBody>
		</button>
	{/each}
</PadGrid>

<style>
	.pad.muted :global(img),
	.pad.off :global(img) {
		opacity: 0.5;
		filter: grayscale(1);
	}
</style>
