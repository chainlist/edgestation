<script lang="ts" module>
	import type { AudioTarget } from '@edgestation/sdk';

	/** Finger movement in px before a drag sets the volume. */
	const SLOP_PX = 6;
	/** The volume step of the arrow keys. */
	const STEP = 0.05;

	interface Drag {
		pointer: number;
		target: AudioTarget;
		/** The finger position and the volume when the drag started to set the volume. */
		x: number;
		y: number;
		from: number;
		/** The fader length in px. A drag along the full lenght goes from 0 to 1. */
		length: number;
		vertical: boolean;
		muted: boolean;
		/** After the slop, the drag sets the volume. */
		owned: boolean;
	}
</script>

<script lang="ts">
	import type { AudioMixer, AudioOutput, WidgetApi } from '@edgestation/sdk';
	import ChannelStrip from './ChannelStrip.svelte';
	import Header from './Header.svelte';
	import { clamp } from './levels.ts';
	import OutputSheet from './OutputSheet.svelte';
	import type { Strip } from './strip.ts';

	let {
		api: widgetApi,
		mixer,
		strips,
		levels,
		channel,
		single,
		rows,
		read,
		toggle,
		setOutput,
		broken = $bindable()
	}: {
		api: WidgetApi;
		mixer: AudioMixer;
		strips: Strip[];
		levels: Record<string, number>;
		channel: string;
		single: boolean;
		rows: boolean;
		read: () => Promise<void>;
		toggle: (target: AudioTarget, muted: boolean) => void;
		setOutput: (id: string) => void;
		broken: Record<string, boolean>;
	} = $props();
	// `api` does not change. A changed setting mounts the widget again.
	// svelte-ignore state_referenced_locally
	const api = widgetApi;

	/**
	 * Volumes that the widget set, by target. They show in place of the last read until Windows
	 * has them.
	 */
	let held = $state.raw<Record<string, number>>({});
	/** The output device, while the list to select one is open. */
	let outputs = $state.raw<AudioOutput[] | null>(null);

	/** Throws the error again in a guard, so that the error card shows. */
	const fail = api.guard((error: unknown) => {
		throw error;
	});
	/**
	 * The latest volume to send, by target. While one volume is on its way, only the last one
	 * waits.
	 */
	const waiting = new Map<AudioTarget, number>();
	/** The sends in progress, one at a time for each target. */
	const sending = new Map<AudioTarget, Promise<void>>();

	function setVolume(target: AudioTarget, volume: number) {
		held = { ...held, [target]: volume };
		waiting.set(target, volume);
		if (sending.has(target)) return;
		const send = async () => {
			for (let next; (next = waiting.get(target)) !== undefined;) {
				waiting.delete(target);
				await api.audio.setVolume(target, next);
			}
		};
		sending.set(
			target,
			send()
				.catch(fail)
				.finally(() => sending.delete(target))
		);
	}

	/** After the widget sends its volumes, reads the volumes from Windows and shows them again. */
	const settle = api.guard(async (target: AudioTarget) => {
		await sending.get(target);
		await read();
		if (drag?.target === target) return;
		const { [target]: _, ...rest } = held;
		held = rest;
	});

	const openOutputs = api.guard(async () => {
		outputs = await api.audio.outputs();
	});

	const pick = (id: string) => {
		outputs = null;
		setOutput(id);
	};

	let drag: Drag | null = null;

	/** The volume and mute state of `target` from the latest read. */
	function current(target: AudioTarget) {
		const found =
			target === 'output'
				? mixer?.output
				: target === 'input'
					? mixer?.input
					: mixer?.apps.find((app) => app.id === target);
		return found && { volume: found.volume, muted: found.muted };
	}

	function press(event: PointerEvent, target: AudioTarget, volume: number, muted: boolean) {
		if (drag || !event.isPrimary || event.button !== 0) return;
		const fader = event.currentTarget as HTMLElement;
		const box = fader.getBoundingClientRect();
		const vertical = box.height >= box.width;
		drag = {
			pointer: event.pointerId,
			target,
			x: event.clientX,
			y: event.clientY,
			from: volume,
			length: (vertical ? box.height : box.width) || 1,
			vertical,
			muted,
			owned: false
		};
		fader.setPointerCapture(event.pointerId);
		// The latest read can be old, because timers wait while the Edge is frozen. This occurs
		// when a game is full screen, and then the mixer is most usefull. Read again. The result
		// comes before the finger moves enough to set a value, so the drag starts from Windows.
		const pointer = event.pointerId;
		read()
			.then(() => {
				const now = current(target);
				if (now && drag?.pointer === pointer && !drag.owned && held[target] === undefined) {
					drag = { ...drag, from: now.volume, muted: now.muted };
				}
			})
			.catch(fail);
	}

	function move(event: PointerEvent) {
		if (event.pointerId !== drag?.pointer) return;
		const dx = event.clientX - drag.x;
		const dy = event.clientY - drag.y;
		const along = drag.vertical ? -dy : dx;
		const across = drag.vertical ? dx : dy;
		if (!drag.owned) {
			// The movement is mostly across the fader, so it does not set the volume.
			if (Math.abs(across) > SLOP_PX && Math.abs(across) >= Math.abs(along)) {
				drag = null;
				return;
			}
			if (Math.abs(along) <= SLOP_PX) return;
			// From now on, the volume follows the finger from its current position.
			drag = { ...drag, x: event.clientX, y: event.clientY, owned: true };
			// A muted volume that goes up becomes unmuted, as in the Windows flyout. This does not
			// apply to the microphone, because a new level must not open it.
			if (drag.muted && drag.target !== 'input') toggle(drag.target, true);
			return;
		}
		setVolume(drag.target, clamp(drag.from + along / drag.length));
	}

	function release(event: PointerEvent) {
		if (event.pointerId !== drag?.pointer) return;
		const { target, owned } = drag;
		drag = null;
		if (owned) settle(target);
	}

	function key(event: KeyboardEvent, target: AudioTarget, volume: number) {
		const step = { ArrowUp: STEP, ArrowRight: STEP, ArrowDown: -STEP, ArrowLeft: -STEP }[event.key];
		if (step === undefined) return;
		event.preventDefault();
		setVolume(target, clamp(volume + step));
		settle(target);
	}
</script>

{#if outputs}
	<OutputSheet {api} {outputs} {mixer} close={() => (outputs = null)} {pick} />
{:else}
	{#if channel === 'all' || (channel === 'output' && mixer.output)}
		<Header {mixer} {channel} {openOutputs} {toggle} />
	{/if}

	<div class="strips-box">
		<div class="strips" class:single class:rows {@attach (strips) => api.scrollbars(strips)}>
			{#each strips as item (item.target || item.name)}
				<ChannelStrip
					{...item}
					held={held[item.target]}
					level={levels[item.target]}
					{single}
					{rows}
					{press}
					{move}
					{release}
					{key}
					{toggle}
					bind:broken
				/>
			{/each}
		</div>
	</div>
{/if}

<style>
	.strips-box {
		flex: 1;
		min-height: 0;
	}

	.strips {
		display: flex;
		gap: 0.3em;
		height: 100%;
		overscroll-behavior: contain;
	}

	.strips.single {
		justify-content: center;
	}

	/* In rows, the faders are horizontal, one below the other. */
	/* The list fills its box and scrolls if the rows do not fit. */
	.strips.rows {
		flex-direction: column;
		justify-content: flex-start;
	}

	.strips.rows.single {
		justify-content: center;
	}
</style>
