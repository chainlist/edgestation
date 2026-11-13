<script lang="ts">
	import type {
		AudioApp,
		AudioLevels,
		AudioMixer,
		AudioOutput,
		AudioTarget,
		WidgetApi
	} from '@edgestation/sdk';
	import Faders from './Faders.svelte';
	import type { Icon } from './icons.ts';
	import { FALL, STEPS, meter, sameLevels } from './levels.ts';
	import Outputs from './Outputs.svelte';
	import Pads from './Pads.svelte';
	import { ICON_PX, type Strip } from './strip.ts';

	let props: { api: WidgetApi } = $props();
	// `api` does not change. A changed setting mounts the widget again.
	// svelte-ignore state_referenced_locally
	const { api } = props;

	/**
	 * The chanel of this placement: `all`, `output`, `input`, `system`, `outputs`, or the program
	 * of an app. A program is the file name of its executable, as the `audioApps` source saves it.
	 */
	const channel = api.settings.channel || 'all';
	/** Only mute buttons, one for each channel, with no faders and no meters. */
	const buttons = api.settings.display === 'buttons';
	/** One button for each output device, which makes it the default. Show does not apply. */
	const selector = channel === 'outputs';
	const single = channel !== 'all' && !selector;

	/** `undefined` until the first read. */
	let mixer = $state.raw<AudioMixer>();
	/** The output devices. Only the selector reads them. */
	let outputs = $state.raw<AudioOutput[]>([]);
	/** Meter heights from 0 to 1 for each target (`output`, `input` or the id of an app). */
	let levels = $state.raw<Record<string, number>>({});
	/** Apps with an icon that did not load. They show the generic icon. */
	let broken = $state.raw<Record<string, boolean>>({});
	let width = $state(0);
	let height = $state(0);
	/**
	 * True for horizontal faders, one in each row. All channels use rows in tall cells. One
	 * channel uses a row in very wide cells.
	 */
	const rows = $derived(single ? width > height * 2.5 : width < height);

	const appStrip = (app: AudioApp): Strip => ({
		target: app.id,
		name: app.name,
		volume: app.volume,
		muted: app.muted,
		src: app.path && api.audio.icon(app.path, ICON_PX),
		fallback: app.path ? 'app' : 'bell',
		live: app.active
	});

	const strips = $derived.by((): Strip[] => {
		if (!mixer || selector) return [];
		const { output, input, apps } = mixer;
		const master: Strip | null = output && {
			target: 'output',
			name: 'Master',
			volume: output.volume,
			muted: output.muted,
			src: null,
			fallback: 'speaker',
			live: apps.some((app) => app.active)
		};
		// Windows does not tell wether an app records, so the microphone meter always reads.
		const mic: Strip | null = input && {
			target: 'input',
			name: 'Microphone',
			volume: input.volume,
			muted: input.muted,
			src: null,
			fallback: 'mic',
			live: true
		};
		const missing = (name: string, fallback: Icon): Strip[] => [
			{ target: '', name, volume: 0, muted: false, src: null, fallback, off: true }
		];
		switch (channel) {
			case 'all':
				// The faders do not show the microphone. The button in their header controls it.
				return [
					...(master ? [master] : []),
					...(buttons && mic ? [mic] : []),
					...apps.map(appStrip)
				];
			case 'output':
				return master ? [master] : missing('No sound output', 'speaker');
			case 'input':
				return mic ? [mic] : missing('No microphone', 'mic');
			case 'system': {
				const system = apps.filter((app) => app.program === null);
				return system.length ? system.map(appStrip) : missing('System sounds', 'bell');
			}
			default: {
				const program = channel.toLowerCase();
				const found = apps.filter((app) => app.program?.toLowerCase() === program);
				return found.length ? found.map(appStrip) : missing(channel.replace(/\.exe$/i, ''), 'app');
			}
		}
	});

	const read = async () => {
		// A device that is plugged in or out also calls `onChange`, so the selector reads the list
		// agian each time.
		if (selector) [mixer, outputs] = await Promise.all([api.audio.mixer(), api.audio.outputs()]);
		else mixer = await api.audio.mixer();
	};
	const refresh = api.guard(read);
	// Read again when Windows reports a change. Nothing polls between the changes.
	api.audio.onChange(refresh);
	refresh();

	/**
	 * The meters get levels only while a channel that they show has an open stream. Other
	 * channels cannot make sound. Buttons have no meters.
	 */
	const metering = $derived(!buttons && strips.some((strip) => strip.live && !strip.off));

	function showLevels(peaks: AudioLevels) {
		const fall = (id: string, peak: number) =>
			Math.round(Math.max(meter(peak), (levels[id] ?? 0) - FALL) * STEPS) / STEPS;
		const next: Record<string, number> = {
			output: fall('output', peaks.output),
			input: fall('input', peaks.input)
		};
		for (const [id, peak] of Object.entries(peaks.apps)) next[id] = fall(id, peak);
		// Keep the old object if the lights are the same, so that nothing draws agian.
		if (!sameLevels(levels, next)) levels = next;
	}

	$effect(() => {
		if (!metering) {
			levels = {};
			return;
		}
		return api.audio.onLevels(showLevels);
	});

	const toggle = api.guard(async (target: AudioTarget, muted: boolean) => {
		await api.audio.setMuted(target, !muted);
		await read();
	});

	const setOutput = api.guard(async (id: string) => {
		if (id === mixer?.output?.id) return;
		await api.audio.setOutput(id);
		await read();
	});
</script>

<!-- `el` is the size container, so its padding cannot use cqh or cqw. Put the layout in it. -->
<div
	class="mixer"
	class:single
	class:rows
	class:buttons
	bind:clientWidth={width}
	bind:clientHeight={height}
>
	{#if mixer && selector}
		<Outputs {outputs} current={mixer.output?.id} pick={setOutput} />
	{:else if mixer && buttons}
		<Pads {strips} {toggle} bind:broken />
	{:else if mixer}
		<Faders
			{api}
			{mixer}
			{strips}
			{levels}
			{channel}
			{single}
			{rows}
			{read}
			{toggle}
			{setOutput}
			bind:broken
		/>
	{/if}
</div>

<style>
	/* All sizes are in em of one font size, which follows the cell (cqh and cqw). The faders are
	   vertical and side by side, or horizontal in rows (`.rows`, from the shape of the cell). */
	.mixer {
		/* The size of the knob along the fader and across the fader. */
		--knob-along: 0.8em;
		--knob-across: 1.9em;
		--glow: color-mix(in oklch, var(--primary) 55%, transparent);
		/* The font size of the output pill and the output list. It follows the cell width, not the
		   em. In a narrow cell the em is small, but the pill and the list can use the full width. */
		--controls: clamp(13px, 8cqw, 18px);

		position: relative;
		display: flex;
		flex-direction: column;
		gap: 0.5em;
		height: 100%;
		padding: 0.6em;
		font-size: min(5.5cqh, 4cqw);
		user-select: none;
	}

	.mixer.rows {
		font-size: min(4cqh, 6.5cqw);
	}

	.mixer.single {
		font-size: min(6.5cqh, 9cqw);
	}

	.mixer.single.rows {
		font-size: min(8cqh, 4cqw);
	}

	/* The child components use the rules that follow. `:where()` keeps their specificity low, so
	   a rule in a child component always overrides them. */
	:where(.mixer) :global(button) {
		padding: 0;
		border: none;
		background: transparent;
		color: inherit;
		font: inherit;
		cursor: pointer;
	}

	/* A pad, as on a DJ controller. It is lit in the accent color when its sound plays. It is
	   dark with a red outline when it is muted. */
	:where(.mixer) :global(.pad) {
		border: 1px solid color-mix(in oklch, var(--primary) 70%, white);
		border-radius: 0.45em;
		background: linear-gradient(
			to bottom,
			color-mix(in oklch, var(--primary) 80%, white),
			var(--primary)
		);
		color: var(--primary-foreground);
		box-shadow:
			0 0 0.9em var(--glow),
			inset 0 1px 0 rgb(255 255 255 / 0.3);
		transition:
			transform 0.1s,
			background 0.15s,
			box-shadow 0.15s;
	}

	:where(.mixer) :global(.pad.muted) {
		border-color: var(--destructive);
		background: color-mix(in oklch, var(--destructive) 12%, var(--card));
		color: var(--destructive);
		box-shadow: none;
	}

	:where(.mixer) :global(.pad.off) {
		border-color: var(--border);
		background: var(--muted);
		color: var(--muted-foreground);
		box-shadow: none;
		cursor: default;
	}

	:where(.mixer) :global(.pad:not(:disabled):active) {
		transform: scale(0.96);
	}
</style>
