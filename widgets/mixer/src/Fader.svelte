<script lang="ts">
	import type { AudioTarget } from '@edgestation/sdk';

	let {
		target,
		name,
		shown,
		muted,
		off,
		rows,
		press,
		move,
		release,
		key
	}: {
		target: AudioTarget;
		name: string;
		shown: number;
		muted: boolean;
		off: boolean | undefined;
		rows: boolean;
		press: (event: PointerEvent, target: AudioTarget, volume: number, muted: boolean) => void;
		move: (event: PointerEvent) => void;
		release: (event: PointerEvent) => void;
		key: (event: KeyboardEvent, target: AudioTarget, volume: number) => void;
	} = $props();
</script>

<div
	class="fader"
	class:muted
	class:off
	class:rows
	role="slider"
	tabindex={off ? -1 : 0}
	aria-label="{name} volume"
	aria-valuemin={0}
	aria-valuemax={100}
	aria-valuenow={Math.round(shown * 100)}
	aria-disabled={off}
	style:--volume={shown}
	onpointerdown={off ? undefined : (event) => press(event, target, shown, muted)}
	onpointermove={move}
	onpointerup={release}
	onpointercancel={release}
	onlostpointercapture={release}
	onkeydown={off ? undefined : (event) => key(event, target, shown)}
>
	<div class="ticks"></div>
	<div class="groove"><div class="fill"></div></div>
	<div class="knob"></div>
</div>

<style>
	/* The fader is a groove between tick marks. The groove has a fill up to the knob. */
	.fader {
		position: relative;
		width: var(--knob-across);
		/* All drags that start here go to the fader. The browser does not scrol for them. */
		touch-action: none;
		cursor: ns-resize;
	}

	.fader:focus-visible {
		border-radius: 0.3em;
		outline: 2px solid var(--ring);
		outline-offset: 2px;
	}

	.fader.off {
		cursor: default;
		opacity: 0.4;
	}

	/* The groove and the ticks go from the center of the knob at 0 to its center at 100. The
	   meter does the same. */
	.ticks,
	.groove {
		position: absolute;
		top: calc(var(--knob-along) / 2);
		bottom: calc(var(--knob-along) / 2);
	}

	/* A tick at each tenth, on the two sides of the groove. */
	.ticks {
		left: 10%;
		right: 10%;
		border-bottom: 1px solid var(--border);
		background: linear-gradient(to bottom, var(--border) 0 1px, transparent 1px) 0 0 / 100% 10%;
	}

	.groove {
		left: 50%;
		width: 0.3em;
		overflow: hidden;
		border-radius: 999px;
		background: var(--muted);
		transform: translateX(-50%);
	}

	.fill {
		position: absolute;
		inset: 0;
		background: var(--primary);
		transform: scaleY(var(--volume));
		transform-origin: bottom;
	}

	.knob {
		position: absolute;
		bottom: calc(var(--volume) * (100% - var(--knob-along)));
		left: 0;
		width: var(--knob-across);
		height: var(--knob-along);
		border-radius: 0.25em;
		background: var(--primary);
		box-shadow:
			0 0 0.7em var(--glow),
			0 0.1em 0.25em rgb(0 0 0 / 0.35);
	}

	/* The grip line across the knob. */
	.knob::after {
		position: absolute;
		top: 50%;
		left: 18%;
		right: 18%;
		height: 2px;
		border-radius: 1px;
		background: var(--primary-foreground);
		opacity: 0.75;
		content: '';
		transform: translateY(-50%);
	}

	.muted .fill {
		opacity: 0.35;
	}

	.muted .knob {
		background: var(--muted-foreground);
		box-shadow: 0 0.1em 0.25em rgb(0 0 0 / 0.35);
	}

	.fader.rows {
		width: auto;
		height: var(--knob-across);
		cursor: ew-resize;
	}

	.rows .ticks,
	.rows .groove {
		top: auto;
		bottom: auto;
		left: calc(var(--knob-along) / 2);
		right: calc(var(--knob-along) / 2);
	}

	.rows .ticks {
		top: 15%;
		bottom: 15%;
		border-bottom: none;
		border-right: 1px solid var(--border);
		background: linear-gradient(to right, var(--border) 0 1px, transparent 1px) 0 0 / 10% 100%;
	}

	.rows .groove {
		top: 50%;
		width: auto;
		height: 0.3em;
		transform: translateY(-50%);
	}

	.rows .fill {
		transform: scaleX(var(--volume));
		transform-origin: left;
	}

	.rows .knob {
		top: 0;
		bottom: auto;
		left: calc(var(--volume) * (100% - var(--knob-along)));
		width: var(--knob-along);
		height: var(--knob-across);
	}

	.rows .knob::after {
		top: 18%;
		bottom: 18%;
		left: 50%;
		right: auto;
		width: 2px;
		height: auto;
		transform: translateX(-50%);
	}
</style>
