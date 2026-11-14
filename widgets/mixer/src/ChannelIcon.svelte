<script lang="ts">
	import type { AudioTarget } from '@edgestation/sdk';
	import Glyph from './Glyph.svelte';
	import { offIcon, type Icon } from './icons.ts';

	let {
		target,
		src,
		fallback,
		muted = false,
		broken = $bindable()
	}: {
		target: AudioTarget;
		src: string | null;
		fallback: Icon;
		/** Shows the crossed out glyph in place of `fallback`. */
		muted?: boolean;
		broken: Record<string, boolean>;
	} = $props();
</script>

<!-- The icon of the app, or a glyph if the app has none or its icon fals to load. -->
{#if src && !broken[target]}
	<img alt="" {src} onerror={() => (broken = { ...broken, [target]: true })} />
{:else}
	<Glyph name={muted ? offIcon(fallback) : fallback} />
{/if}
