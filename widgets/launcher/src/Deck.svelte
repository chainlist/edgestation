<script lang="ts" module>
	/** The time in ms that the user holds a key before it opens the app picker. */
	const HOLD_MS = 500;
	/** The finger movment in px that cancels a hold. */
	const SLOP_PX = 10;

	const SHAKE: Keyframe[] = [
		{ transform: 'translateX(0)' },
		{ transform: 'translateX(-6%)' },
		{ transform: 'translateX(5%)' },
		{ transform: 'translateX(-3%)' },
		{ transform: 'translateX(0)' }
	];
</script>

<script lang="ts">
	import type { AppInfo, WidgetApi } from '@edgestation/sdk';
	import type { Snippet } from 'svelte';
	import Glyph from './Glyph.svelte';
	import type { Keys } from './keys.ts';

	let {
		api,
		deck,
		rows,
		slots,
		labels,
		editable,
		keyIcon,
		appIcon,
		onedit
	}: {
		api: WidgetApi;
		deck: { side: number; gap: number; cols: number; count: number };
		rows: number;
		slots: Keys;
		labels: boolean;
		editable: boolean;
		keyIcon: number;
		appIcon: Snippet<[AppInfo, number]>;
		onedit: (index: number, event: Event) => void;
	} = $props();

	/** Rings `key` in the accent color, or in red with a shake if its app did not start. */
	function flash(key: HTMLElement, failed: boolean) {
		const ring = key.querySelector<HTMLElement>('.ring');
		if (!ring) return;
		ring.style.borderColor = failed ? 'var(--destructive)' : '';
		ring.animate(
			[
				{ opacity: 1, transform: 'scale(1)' },
				{ opacity: 0, transform: 'scale(1.08)' }
			],
			{ duration: failed ? 900 : 500, easing: 'ease-out' }
		);
		if (failed) key.animate(SHAKE, { duration: 400 });
	}

	function launch(app: AppInfo, key: HTMLElement) {
		flash(key, false);
		api.apps.launch(app.id).catch((error) => {
			console.warn(`Launcher: ${app.name} did not start:`, error);
			flash(key, true);
		});
	}

	/** The key that the user holds. It opens the picker after `HOLD_MS`. */
	let hold: { x: number; y: number; cancel: () => void } | undefined;
	/** True if a hold opened the picker. Then the click at the end of the hold opens no app. */
	let held = false;

	function press(index: number, event: PointerEvent) {
		release();
		held = false;
		if (!editable || !event.isPrimary || event.button !== 0) return;
		const cancel = api.after(HOLD_MS, () => {
			hold = undefined;
			held = true;
			edit(index, event);
		});
		hold = { x: event.clientX, y: event.clientY, cancel };
	}

	// A finger that moves is not a hold.
	function move(event: PointerEvent) {
		if (hold && Math.hypot(event.clientX - hold.x, event.clientY - hold.y) > SLOP_PX) release();
	}

	function release() {
		hold?.cancel();
		hold = undefined;
	}

	function edit(index: number, event: Event) {
		release();
		onedit(index, event);
	}

	function tap(index: number, event: MouseEvent) {
		if (held) {
			held = false;
			return;
		}
		const app = slots[index];
		if (app) launch(app, event.currentTarget as HTMLElement);
		else if (editable) edit(index, event);
	}
</script>

<div
	class="deck"
	style:--side="{deck.side}px"
	style:--gap="{deck.gap}px"
	style:grid-template-columns="repeat({deck.cols}, var(--side))"
	style:grid-template-rows="repeat({rows}, var(--side))"
>
	{#each slots as app, index (index)}
		{#if app}
			<button
				class="key"
				class:labelled={labels}
				aria-label="Open {app.name}"
				title={app.name}
				onpointerdown={(event) => press(index, event)}
				onpointermove={move}
				onpointerup={release}
				onpointercancel={release}
				onpointerleave={release}
				onclick={(event) => tap(index, event)}
				oncontextmenu={(event) => edit(index, event)}
			>
				{@render appIcon(app, keyIcon)}
				{#if labels}<span class="label">{app.name}</span>{/if}
				<span class="ring"></span>
			</button>
		{:else if editable}
			<button
				class="key empty"
				aria-label="Pick an app for this key"
				onclick={(event) => tap(index, event)}
			>
				<Glyph name="add" />
			</button>
		{:else}
			<div class="key empty"></div>
		{/if}
	{/each}
</div>

<style>
	/* Square keys. The script sets the size (`--side`), so that only full keys show. */
	.deck {
		display: grid;
		place-content: center;
		gap: var(--gap);
		height: 100%;
	}

	.key {
		--icon-width: 60%;
		--initial-size: calc(var(--side) * 0.25);
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: calc(var(--side) * 0.06);
		min-width: 0;
		padding: calc(var(--side) * 0.08);
		border-radius: min(calc(var(--radius) * 1.5), calc(var(--side) * 0.22));
		background: color-mix(in oklch, var(--card-foreground) 8%, transparent);
		color: var(--card-foreground);
		box-shadow:
			inset 0 1px 0 color-mix(in oklch, var(--foreground) 10%, transparent),
			0 1px 3px rgb(0 0 0 / 0.25);
		transition:
			transform 0.1s,
			filter 0.1s;
	}

	button.key:active {
		transform: scale(0.93);
		filter: brightness(1.2);
	}

	.key.labelled {
		--icon-width: 50%;
	}

	.label {
		max-width: 100%;
		overflow: hidden;
		font-size: max(10px, calc(var(--side) * 0.115));
		line-height: 1.2;
		white-space: nowrap;
		text-overflow: ellipsis;
	}

	/* This flashes when the user tapps the key (see `flash`). */
	.ring {
		position: absolute;
		inset: 0;
		border: max(2px, calc(var(--side) * 0.03)) solid var(--primary);
		border-radius: inherit;
		opacity: 0;
		pointer-events: none;
	}

	.empty {
		background: color-mix(in oklch, var(--card-foreground) 3%, transparent);
		box-shadow: none;
		color: var(--muted-foreground);
	}

	.empty :global(svg) {
		width: 30%;
		opacity: 0.7;
	}
</style>
