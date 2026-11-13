<script lang="ts">
	import type { LoadingLook } from '#lib/widgets/api/loading.ts';
	import { overlay } from '#lib/widgets/webview/motion.ts';
	import { fade } from 'svelte/transition';

	let { look }: { look: LoadingLook } = $props();

	/**
	 * Web views draw above the card, so a view under the bar leaves a hole for it (see `overlay`).
	 * The hole opens when the bar shows, after its wait. Thus a quick load does not cut the page.
	 */
	function holeInViews(bar: HTMLElement) {
		let close: (() => void) | undefined;
		const shown = (event: AnimationEvent) => {
			// The sweep of `::before` starts too, at once.
			if (!event.pseudoElement) close ??= overlay(bar);
		};
		bar.addEventListener('animationstart', shown);
		return () => {
			bar.removeEventListener('animationstart', shown);
			close?.();
		};
	}
</script>

<!-- Both indicators wait a moment before they show, so a quick load does not flash them. The
     card clips the bar to its round corners. -->
{#if look === 'dots'}
	<div class="dots" out:fade={{ duration: 150 }}>
		<span></span><span></span><span></span>
	</div>
{:else if look === 'bar'}
	<div class="bar" out:fade={{ duration: 300 }} {@attach holeInViews}></div>
{/if}

<style>
	.dots,
	.bar {
		position: absolute;
		pointer-events: none;
	}

	/* The dots scale with the card, like a widget does with cqh and cqw. The units in `--dot`
	   resolve on the dots, so `.dots` cannot use it for a gap. */
	.dots {
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		container-type: size;
		--dot: clamp(4px, min(3.5cqh, 2.5cqw), 10px);
	}

	.dots span {
		width: var(--dot);
		height: var(--dot);
		margin: 0 calc(var(--dot) / 2);
		border-radius: 50%;
		background: var(--muted-foreground);
		animation: blink 1.2s ease-in-out 200ms infinite both;
	}

	/* Each dot fades in 200 ms after the one before it. The first delay is the wait of all the
	   indicators. */
	.dots span:nth-child(2) {
		animation-delay: 400ms;
	}

	.dots span:nth-child(3) {
		animation-delay: 600ms;
	}

	@keyframes blink {
		0%,
		80%,
		100% {
			opacity: 0;
		}
		20%,
		60% {
			opacity: 1;
		}
	}

	.bar {
		inset: 0 0 auto;
		height: 2px;
		background: color-mix(in oklch, var(--primary) 15%, transparent);
		animation: appear 200ms 200ms both;
	}

	@keyframes appear {
		from {
			opacity: 0;
		}
	}

	/* A streak of the accent color with a light head. It glows only where it is opaque. */
	.bar::before {
		content: '';
		position: absolute;
		inset: 0 auto 0 0;
		width: 40%;
		background: linear-gradient(
			90deg,
			transparent,
			var(--primary) 70%,
			color-mix(in oklch, var(--primary) 55%, white) 94%,
			transparent
		);
		filter: drop-shadow(0 0 3px var(--primary));
		animation: sweep 1.4s cubic-bezier(0.65, 0, 0.35, 1) infinite;
	}

	/* From just out of the left edge to just out of the right edge, in widths of the streak. */
	@keyframes sweep {
		from {
			transform: translateX(-100%);
		}
		to {
			transform: translateX(250%);
		}
	}

	/* The Edge is frozen while an other app is full screen. */
	:global(.frozen) .dots span,
	:global(.frozen) .bar::before {
		animation-play-state: paused;
	}
</style>
