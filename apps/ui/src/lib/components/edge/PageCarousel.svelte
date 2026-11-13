<script lang="ts">
	import type { Page } from '#lib/layout/types.ts';
	import type { PageDirection, PageTransition } from '#lib/state/settings.svelte.ts';
	import {
		aboutToMove,
		layoutChanged,
		setMoving,
		viewsPlaced
	} from '#lib/widgets/webview/motion.ts';
	import { tick } from 'svelte';
	import GridPage from './GridPage.svelte';
	import PageDots from './PageDots.svelte';
	import PageSwitcher from './PageSwitcher.svelte';

	let {
		pages,
		direction,
		transition
	}: { pages: Page[]; direction: PageDirection; transition: PageTransition } = $props();

	/** Time and easing for a page to snap into place or to fade in, after a swipe or a dot tap. */
	const SNAP_MS = 250;
	const SNAP_EASING = 'cubic-bezier(0.2, 0, 0, 1)';
	/** The two fingers must move this many pixels along the page axis before the pages follow. */
	const SLOP_PX = 10;
	/**
	 * A release changes the page if the fingers moved more than this part of the page size, or
	 * faster than this speed.
	 */
	const COMMIT_RATIO = 0.25;
	const FLICK_PX_PER_MS = 0.4;
	/** Past the first or the last page, the drag moves only this part of the finger travel. */
	const EDGE_RESISTANCE = 0.3;
	/** Maximum wait for native views to hide. After it, the page change starts anyway. */
	const HOLD_MS = 150;

	let viewport: HTMLElement;
	let index = $state(0);
	const active = $derived(Math.min(index, pages.length - 1));
	const vertical = $derived(direction === 'vertical');
	const fade = $derived(transition === 'fade');

	/**
	 * Fingers on the page by pointer id, at their last position. Two fingers make a swipe. One
	 * finger, a mouse or a pen allways stays with the widget under it.
	 */
	const touches = new Map<number, { x: number; y: number }>();
	/** The two fingers of a swipe, and the point between them and the time when they landed. */
	interface Swipe {
		ids: [number, number];
		x: number;
		y: number;
		t: number;
		dragging: boolean;
	}
	let gesture: Swipe | null = null;
	let offset = $state(0);
	let dragging = $state(false);
	/** The page that comes into view in a swipe. Its widgets start agian when it shows. */
	const incoming = $derived(dragging ? active - Math.sign(offset) : active);

	let width = $state(0);
	let height = $state(0);
	/** Part of the page that the fingers moved along the swipe axis, from 0 to 1. */
	const swiped = $derived(Math.min(1, Math.abs(offset) / ((vertical ? height : width) || 1)));
	/**
	 * Opacity of page `i` in fade mode. The open page shows less as a swipe brings in the next
	 * page. The next page shows as much as the fingers moved.
	 */
	const opacity = (i: number) => (i === active ? 1 - swiped : i === incoming ? swiped : 0);

	let track: HTMLElement;
	/** True while the track moves into place after a swipe, a dot tap or a page change. */
	let snapping = $state(false);
	let scrubbing = $state(false);
	let holding = $state(false);
	/** The Go to page box is open. */
	let finding = $state(false);
	// Native views (`api.webview`) cannot slide with the pages, and they would cover the page
	// number and the Go to page box. Thus they hide in all these states (see `motion.ts`).
	$effect(() => setMoving(dragging || snapping || scrubbing || holding || finding));
	$effect(() => {
		void pages;
		void direction;
		void transition;
		layoutChanged();
	});

	/** True for a transition of the track or of a page, not of a widget. */
	const own = (e: TransitionEvent) =>
		e.target === track || (e.target as Element).parentElement === track;
	const turning = () => [track, ...track.children].some((el) => el.getAnimations().length > 0);

	/**
	 * Sets `snapping` before the transition starts, so a native view never shows at the place
	 * that the finger left. It stays true until the transition ends. If the pages did not move,
	 * it becomes false at once.
	 */
	function snap() {
		snapping = true;
		holding = false;
		tick().then(() => {
			if (!turning()) snapping = false;
		});
	}

	/**
	 * Hides native views before a page change. A view that still shows stays in place while the
	 * pages slide, untill its picture is ready. Waits for the views to hide, but not more than
	 * `HOLD_MS`.
	 */
	async function hold() {
		holding = true;
		setMoving(true);
		await Promise.race([viewsPlaced(), new Promise((done) => setTimeout(done, HOLD_MS))]);
	}

	async function go(i: () => number) {
		await hold();
		index = i();
		snap();
	}

	/**
	 * Turns `by` pages for a shortcut. After the last page it goes back to the first, and the
	 * other way. Thus one shortcut can go through all the pages.
	 */
	export function turn(by: number) {
		if (pages.length < 2) return;
		// Count from the page that shows after the views hide. Thus a shortcut that is held down
		// turns one page after the other.
		return go(() => {
			const count = pages.length;
			return count < 2 ? index : (((active + by) % count) + count) % count;
		});
	}

	/** Turns to page `i`, if it is not the open page. */
	const goTo = (i: number) => i !== active && go(() => i);

	/** Turns to the page `id`, for exemple the page that the user picks in Settings. */
	export function show(id: string) {
		const i = pages.findIndex((page) => page.id === id);
		if (i >= 0) goTo(i);
	}

	let switcher: ReturnType<typeof PageSwitcher>;

	/** Opens the Go to page box for its shortcut, or closes it if it is open. */
	export const find = () => switcher.toggle();

	/** Sets `snapping` to false only when the last page stops. In fade mode, two pages fade. */
	function settled(e: TransitionEvent) {
		if (own(e) && !turning()) snapping = false;
	}

	/** The point halfway between the two fingers of the swipe. */
	function middle([a, b]: [number, number]) {
		const p = touches.get(a)!;
		const q = touches.get(b)!;
		return { x: (p.x + q.x) / 2, y: (p.y + q.y) / 2 };
	}

	function travel(from: Swipe) {
		const at = middle(from.ids);
		const dx = at.x - from.x;
		const dy = at.y - from.y;
		return vertical ? { along: dy, across: dx } : { along: dx, across: dy };
	}

	/** Keeps the last position of finger `e`. Tells if it is one of the two fingers of the swipe. */
	function follow(e: PointerEvent) {
		const touch = touches.get(e.pointerId);
		if (!touch) return false;
		touch.x = e.clientX;
		touch.y = e.clientY;
		return gesture?.ids.includes(e.pointerId) ?? false;
	}

	function down(e: PointerEvent) {
		if (e.pointerType !== 'touch') return;
		touches.set(e.pointerId, { x: e.clientX, y: e.clientY });
		if (gesture || touches.size !== 2) return;
		const ids = [...touches.keys()] as [number, number];
		// From the second finger, the page owns the two touches. The widgets under them loose them
		// and get no click.
		for (const id of ids) viewport.setPointerCapture(id);
		gesture = { ids, ...middle(ids), t: e.timeStamp, dragging: false };
		aboutToMove();
	}

	function move(e: PointerEvent) {
		if (!follow(e) || !gesture) return;
		const { along, across } = travel(gesture);
		if (!gesture.dragging) {
			if (Math.abs(across) > SLOP_PX && Math.abs(across) >= Math.abs(along)) gesture = null;
			if (!gesture || Math.abs(along) <= SLOP_PX) return;
			gesture.dragging = dragging = true;
		}
		const atEdge = (along > 0 && active === 0) || (along < 0 && active === pages.length - 1);
		offset = atEdge ? along * EDGE_RESISTANCE : along;
	}

	/** The swipe ends when one of its two fingers lifts. */
	function up(e: PointerEvent) {
		if (follow(e) && gesture) {
			if (gesture.dragging && e.type === 'pointerup') {
				const { along } = travel(gesture);
				const size = vertical ? viewport.clientHeight : viewport.clientWidth;
				const speed = Math.abs(along) / Math.max(1, e.timeStamp - gesture.t);
				if (Math.abs(along) > size * COMMIT_RATIO || speed > FLICK_PX_PER_MS) {
					index = Math.max(0, Math.min(pages.length - 1, active - Math.sign(along)));
				}
			}
			if (gesture.dragging) snap();
			gesture = null;
			dragging = false;
			offset = 0;
		}
		touches.delete(e.pointerId);
	}
</script>

<!-- The swipe is a touch shortcut. The page dots are the accessible way to change the page. -->
<!-- Moves and releases come to the page before the widgets, so a widget cannot hide them. Presses
     come after, so that the dots and the Go to page box can keep theirs. -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	bind:this={viewport}
	bind:clientWidth={width}
	bind:clientHeight={height}
	class={['relative size-full overflow-hidden', vertical ? 'touch-pan-x' : 'touch-pan-y']}
	onpointerdown={down}
	onpointermovecapture={move}
	onpointerupcapture={up}
	onpointercancelcapture={up}
>
	<!-- In slide mode, the pages sit in a row and the track moves. A translate percentage is of
	     the track size, wich is one page. In fade mode, the pages lie in one cell and only the
	     open page shows. -->
	<div
		bind:this={track}
		class={['size-full', fade ? 'grid grid-cols-1 grid-rows-1' : ['flex', vertical && 'flex-col']]}
		ontransitionrun={(e) => own(e) && (snapping = true)}
		ontransitionend={settled}
		ontransitioncancel={settled}
		style:transform={fade
			? undefined
			: `${vertical ? 'translateY' : 'translateX'}(calc(${-active * 100}% + ${offset}px))`}
		style:transition={fade || dragging ? 'none' : `transform ${SNAP_MS}ms ${SNAP_EASING}`}
	>
		{#each pages as page, i (page.id)}
			{@const shows = fade ? opacity(i) : 1}
			<!-- Every page stays mounted. When a page is off screen or faded out, the timers of its
			     widgets stop. A faded out page is hidden, so its native views stay hidden too. -->
			<div
				class="col-start-1 row-start-1 size-full shrink-0"
				style:opacity={fade ? shows : undefined}
				style:visibility={shows === 0 ? 'hidden' : undefined}
				style:pointer-events={fade && i !== active ? 'none' : undefined}
				style:transition={fade && !dragging
					? `opacity ${SNAP_MS}ms ${SNAP_EASING}, visibility ${SNAP_MS}ms`
					: undefined}
			>
				<GridPage {page} paused={i !== active && i !== incoming} />
			</div>
		{/each}
	</div>
	{#if pages.length > 1}
		<PageDots
			bind:open={scrubbing}
			count={pages.length}
			{active}
			{vertical}
			onselect={(i) => go(() => i)}
		/>
	{/if}
	<PageSwitcher bind:this={switcher} bind:open={finding} {pages} {active} onselect={goTo} />
</div>
