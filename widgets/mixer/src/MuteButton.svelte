<script lang="ts">
	import type { AudioTarget } from '@edgestation/sdk';
	import ChannelIcon from './ChannelIcon.svelte';
	import Glyph from './Glyph.svelte';
	import { offIcon, type Icon } from './icons.ts';

	let {
		target,
		name,
		muted,
		src,
		fallback,
		off,
		toggle,
		broken = $bindable()
	}: {
		target: AudioTarget;
		name: string;
		muted: boolean;
		src: string | null;
		fallback: Icon;
		off: boolean | undefined;
		toggle: (target: AudioTarget, muted: boolean) => void;
		broken: Record<string, boolean>;
	} = $props();
</script>

<button
	class="mute"
	class:muted
	disabled={off}
	aria-label={muted ? `Unmute ${name}` : `Mute ${name}`}
	aria-pressed={muted}
	onclick={() => toggle(target, muted)}
>
	<ChannelIcon {target} {src} {fallback} bind:broken />
	{#if muted}
		<span class="badge"><Glyph name={offIcon(fallback)} /></span>
	{/if}
</button>

<style>
	.mute {
		position: relative;
		display: grid;
		grid-area: mute;
		place-items: center;
		width: 1.9em;
		height: 1.9em;
		border-radius: 0.45em;
		transition: transform 0.1s;
	}

	.mute:not(:disabled):active {
		transform: scale(0.9);
	}

	.mute:disabled {
		cursor: default;
		opacity: 0.4;
	}

	.mute > :global(img) {
		width: 100%;
		height: 100%;
		object-fit: contain;
	}

	.mute > :global(svg) {
		width: 75%;
		height: 75%;
		color: var(--muted-foreground);
	}

	.mute.muted > :global(img),
	.mute.muted > :global(svg) {
		opacity: 0.4;
	}

	.badge {
		position: absolute;
		right: -0.25em;
		bottom: -0.25em;
		display: grid;
		place-items: center;
		width: 1em;
		height: 1em;
		border-radius: 50%;
		background: var(--destructive);
		color: white;
	}

	.badge :global(svg) {
		width: 0.65em;
		height: 0.65em;
	}
</style>
