<script lang="ts">
	let {
		count,
		active,
		vertical,
		onselect,
		open = $bindable(false)
	}: {
		count: number;
		active: number;
		vertical: boolean;
		onselect: (index: number) => void;
		/** True while a dot is pressed. Then the bar and the big page number show. */
		open?: boolean;
	} = $props();

	/** At rest, the distance between dots (one button) and the size of one dot. */
	const PITCH = 20;
	const DOT = 8;
	/**
	 * When pressed, the bar is large enough to see around a finger. Each page gets up to `STEP`
	 * pixels of its length, or less if the bar does not fit the screen. `STEP` is also the
	 * finger travel for one page.
	 */
	const STEP = 120;
	const THICK = 72;
	const INSET = 6;
	/** When pressed, the size of the dots and the distance from the bar to the screen edges. */
	const MARKER = 12;
	const MARGIN = 16;
	/** When pressed, the bar moves this far from the dots, so it is clear of the screen edge. */
	const RISE = THICK / 2 + MARGIN - PITCH / 2;
	/** The time for the bar to open. It agrees with `duration-150` in `spring`. */
	const OPEN_MS = 150;
	/**
	 * Wheel travel over the dots for one page. One notch of an mouse wheel turns one page at
	 * most. The bar shows while the wheel turns, and closes this long after it stops.
	 */
	const WHEEL_PX = 50;
	const WHEEL_IDLE_MS = 600;
	const spring =
		'transition-[transform,width,height,opacity] duration-150 ease-[cubic-bezier(0.34,1.4,0.64,1)]';
	/** All parts of the bar sit on the middle of the dots and spring to their places. */
	const morph = `pointer-events-none absolute top-1/2 left-1/2 ${spring}`;

	let frame: HTMLElement;
	/** The scrub pointer, its start position along the axis, and the page that it pressed. */
	let scrub: { id: number; start: number; from: number } | null = null;
	const middle = $derived((count - 1) / 2);
	/** The size of the nav. Along the axis it spans the screen. This limits the lenght of the bar. */
	let width = $state(0);
	let height = $state(0);
	const step = $derived(Math.min(STEP, ((vertical ? height : width) - 2 * MARGIN) / count));

	/** The position of page `i` along the axis, from the middle of the dots and of the bar. */
	function place(i: number) {
		return (i - middle) * (open ? step : PITCH);
	}

	/** Centers an element on the middle of the dots, then moves it along and across the axis. */
	function shift(along: number) {
		const across = open ? -RISE : 0;
		const [x, y] = vertical ? [across, along] : [along, across];
		return `translate(-50%, -50%) translate(${x}px, ${y}px)`;
	}

	/** The width and height of a box with this length along the axis and this depth across it. */
	function box(along: number, across: number) {
		return vertical ? { w: `${across}px`, h: `${along}px` } : { w: `${along}px`, h: `${across}px` };
	}

	const track = $derived(open ? box(count * step, THICK) : box((count - 1) * PITCH + DOT, DOT));
	const thumb = $derived(open ? box(step - 2 * INSET, THICK - 2 * INSET) : box(DOT, DOT));
	const marker = $derived(`${open ? MARKER : DOT}px`);

	const coord = (e: PointerEvent) => (vertical ? e.clientY : e.clientX);

	function press(e: PointerEvent, i: number) {
		if (scrub || !e.isPrimary || e.button !== 0) return;
		// The dots own this pointer. The carousel must not start a swipe from it.
		e.stopPropagation();
		frame.setPointerCapture(e.pointerId);
		scrub = { id: e.pointerId, start: coord(e), from: i };
		open = true;
		// The bar opens first, becuase the new page can take time to draw and stop the screen.
		// If a scrub changed the page before then, nothing changes here.
		const shown = active;
		if (i !== active) setTimeout(() => active === shown && onselect(i), OPEN_MS);
	}

	function move(e: PointerEvent) {
		if (e.pointerId !== scrub?.id) return;
		const steps = Math.round((coord(e) - scrub.start) / step);
		const page = Math.max(0, Math.min(count - 1, scrub.from + steps));
		if (page !== active) onselect(page);
	}

	function release(e: PointerEvent) {
		if (e.pointerId !== scrub?.id) return;
		scrub = null;
		open = false;
	}

	/** Wheel travel that did not turn a page yet. It is negative for the previous page. */
	let wheeled = 0;
	let wheelIdle: ReturnType<typeof setTimeout> | undefined;
	$effect(() => () => clearTimeout(wheelIdle));

	function wheel(e: WheelEvent) {
		if (scrub) return;
		// Use the larger axis. A sideways wheel is as good for a row of dots as a normal wheel.
		const delta = Math.abs(e.deltaX) > Math.abs(e.deltaY) ? e.deltaX : e.deltaY;
		// A wheel that counts lines or pages turns one page for each event.
		const px = e.deltaMode === WheelEvent.DOM_DELTA_PIXEL ? delta : Math.sign(delta) * WHEEL_PX;
		if (Math.sign(px) !== Math.sign(wheeled)) wheeled = 0;
		wheeled += px;
		open = true;
		clearTimeout(wheelIdle);
		wheelIdle = setTimeout(() => {
			wheeled = 0;
			if (!scrub) open = false;
		}, WHEEL_IDLE_MS);
		if (Math.abs(wheeled) < WHEEL_PX) return;
		const page = Math.max(0, Math.min(count - 1, active + Math.sign(wheeled)));
		wheeled = 0;
		if (page !== active) onselect(page);
	}
</script>

<!-- While a dot is pressed, the page number shows big in the middle of the screen. -->
<div class="pointer-events-none absolute inset-0 grid place-items-center">
	<div
		class={[
			spring,
			'rounded-3xl border bg-card/80 px-10 py-4 text-7xl font-semibold tabular-nums shadow-lg backdrop-blur-md'
		]}
		style:opacity={open ? 1 : 0}
		style:transform="scale({open ? 1 : 0.8})"
		aria-hidden="true"
	>
		{active + 1}
	</div>
</div>

<nav
	bind:clientWidth={width}
	bind:clientHeight={height}
	class={[
		'absolute flex justify-center',
		vertical ? 'inset-y-0 right-0 flex-col' : 'inset-x-0 bottom-0'
	]}
>
	<!-- The scrub follows the captured pointer here, and the wheel turns pages over the dots.
	     The buttons are the accesible controls. -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		bind:this={frame}
		class={['relative flex touch-none', vertical && 'flex-col']}
		onpointermove={move}
		onpointerup={release}
		onpointercancel={release}
		onwheel={wheel}
	>
		<!-- A press opens the bar and selects the page. The click is only for the keyboard. -->
		{#each { length: count }, i (i)}
			<button
				class="size-5"
				aria-label="Page {i + 1}"
				onpointerdown={(e) => press(e, i)}
				onclick={(e) => e.detail === 0 && onselect(i)}
			></button>
		{/each}
		<div
			class={[morph, 'rounded-full border bg-card/80 shadow-lg backdrop-blur-md']}
			style:width={track.w}
			style:height={track.h}
			style:opacity={open ? 1 : 0}
			style:transform={shift(0)}
		></div>
		{#each { length: count }, i (i)}
			<span
				class={[morph, 'rounded-full bg-muted-foreground/40']}
				style:width={marker}
				style:height={marker}
				style:transform={shift(place(i))}
			></span>
		{/each}
		<!-- The active dot, which grows into the bar's thumb. -->
		<div
			class={[morph, 'rounded-full bg-primary']}
			style:width={thumb.w}
			style:height={thumb.h}
			style:transform={shift(place(active))}
		></div>
	</div>
</nav>
