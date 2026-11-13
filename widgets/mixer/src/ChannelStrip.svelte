<script lang="ts">
	import type { AudioTarget } from '@edgestation/sdk';
	import Fader from './Fader.svelte';
	import Meter from './Meter.svelte';
	import MuteButton from './MuteButton.svelte';
	import type { Strip } from './strip.ts';

	let {
		target,
		name,
		volume,
		muted,
		src,
		fallback,
		off,
		held,
		level,
		single,
		rows,
		press,
		move,
		release,
		key,
		toggle,
		broken = $bindable()
	}: Strip & {
		held: number | undefined;
		level: number | undefined;
		single: boolean;
		rows: boolean;
		press: (event: PointerEvent, target: AudioTarget, volume: number, muted: boolean) => void;
		move: (event: PointerEvent) => void;
		release: (event: PointerEvent) => void;
		key: (event: KeyboardEvent, target: AudioTarget, volume: number) => void;
		toggle: (target: AudioTarget, muted: boolean) => void;
		broken: Record<string, boolean>;
	} = $props();

	const shown = $derived(held ?? volume);
</script>

<div
	class="strip"
	class:muted
	class:off
	class:master={target === 'output' && !single}
	class:single
	class:rows
>
	<span class="name" title={name}>{name}</span>
	<span class="value">{off ? 'Not running' : Math.round(shown * 100)}</span>
	<div class="channel">
		<Fader {target} {name} {shown} {muted} {off} {rows} {press} {move} {release} {key} />
		<Meter level={off ? 0 : (level ?? 0)} {rows} />
	</div>
	<MuteButton {target} {name} {muted} {src} {fallback} {off} {toggle} bind:broken />
</div>

<style>
	/* One channel, with its name and value above the fader and meter, and its mute button below. */
	.strip {
		display: grid;
		flex: 1 0 3.4em;
		grid-template-rows: auto auto minmax(0, 1fr) auto;
		grid-template-areas: 'name' 'value' 'channel' 'mute';
		justify-items: center;
		gap: 0.25em;
		max-width: 6em;
		min-width: 0;
	}

	.strip.single {
		max-width: none;
	}

	.strip.master {
		margin-right: 0.3em;
		padding-right: 0.6em;
		border-right: 1px solid var(--border);
	}

	.name {
		grid-area: name;
		max-width: 100%;
		overflow: hidden;
		color: var(--muted-foreground);
		font-size: 0.55em;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		white-space: nowrap;
		text-overflow: ellipsis;
	}

	.value {
		grid-area: value;
		font-size: 0.7em;
		font-variant-numeric: tabular-nums;
	}

	.off .value {
		color: var(--muted-foreground);
		font-size: 0.55em;
	}

	/* One channel has room for larger labels. */
	.single .value {
		font-size: 0.95em;
	}

	.single .name,
	.single.off .value {
		font-size: 0.75em;
	}

	.channel {
		display: flex;
		grid-area: channel;
		gap: 0.25em;
		height: 100%;
		min-height: 0;
	}

	.strip.rows {
		flex: none;
		grid-template-columns: auto minmax(0, 1fr) auto;
		grid-template-rows: auto auto;
		grid-template-areas: 'mute name value' 'mute channel channel';
		align-items: center;
		justify-items: stretch;
		column-gap: 0.5em;
		row-gap: 0.1em;
		max-width: none;
		padding: 0.15em 0;
	}

	.strip.rows.master {
		margin: 0 0 0.3em;
		padding: 0 0 0.4em;
		border-right: none;
		border-bottom: 1px solid var(--border);
	}

	.rows .value {
		text-align: right;
	}

	.rows .channel {
		flex-direction: column;
		height: auto;
		gap: 0.15em;
	}
</style>
