<script lang="ts">
	import type { HTMLAttributes } from 'svelte/elements';
	import { prefersReducedMotion } from 'svelte/motion';
	import { scale } from 'svelte/transition';
	import { cellStyle } from '#lib/layout/grid.ts';
	import type { Rect } from '#lib/layout/types.ts';
	import { widgetName } from '#lib/widgets/installed.svelte.ts';

	let {
		widget,
		rect,
		invalid = false,
		lifted = false,
		class: className,
		children,
		...rest
	}: HTMLAttributes<HTMLDivElement> & {
		widget: string;
		rect: Rect;
		invalid?: boolean;
		/** Drawn above the others with a shadow, for example during a drag. */
		lifted?: boolean;
	} = $props();

	let el = $state<HTMLDivElement>();
	const area = $derived(cellStyle(rect));

	// When the widget changes cells after a drag or a swap, it glides from its old position. It
	// does not jump. Measure it before and after the DOM update (FLIP).
	let drawn: DOMRect | undefined;
	let glide: Animation | undefined;
	$effect.pre(() => {
		void area;
		// This includes a glide that still runs, so a new glide starts from the position on screen.
		drawn = el?.getBoundingClientRect();
	});
	$effect(() => {
		void area;
		if (!el || !drawn) return;
		glide?.cancel();
		const now = el.getBoundingClientRect();
		const dx = drawn.left - now.left;
		const dy = drawn.top - now.top;
		if ((!dx && !dy) || prefersReducedMotion.current) return;
		glide = el.animate([{ transform: `translate(${dx}px, ${dy}px)` }, { transform: 'none' }], {
			duration: 220,
			easing: 'cubic-bezier(0.2, 0, 0, 1)'
		});
	});
</script>

<!-- The colors are mixed with the backround, so the cells below do not show through. The item
     scales in and out when a widget is added to or removed from the page. -->
<div
	bind:this={el}
	transition:scale={{ start: 0.8, duration: prefersReducedMotion.current ? 0 : 150 }}
	class={[
		'relative flex items-center justify-center truncate rounded-sm border px-1 text-xs transition-shadow select-none',
		lifted ? 'z-20 shadow-lg shadow-black/40' : 'z-10',
		invalid
			? 'border-destructive bg-[color-mix(in_oklab,var(--destructive)_20%,var(--background))]'
			: 'border-primary/50 bg-[color-mix(in_oklab,var(--primary)_15%,var(--background))]',
		className
	]}
	style={area}
	{...rest}
>
	{widgetName(widget)}
	{@render children?.()}
</div>
