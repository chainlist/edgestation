import type { GridSize } from './types';

/** The distance in pixels from one cell to the next in a rendred grid, gap included. */
export function gridPitch(el: HTMLElement, { cols, rows }: GridSize) {
	const style = getComputedStyle(el);
	const px = (value: string) => parseFloat(value) || 0;
	const width = el.clientWidth - px(style.paddingLeft) - px(style.paddingRight);
	const height = el.clientHeight - px(style.paddingTop) - px(style.paddingBottom);
	return { x: (width + px(style.columnGap)) / cols, y: (height + px(style.rowGap)) / rows };
}

/**
 * Follows the pointer drag that `event` starts. When the number of whole cells that the pointer
 * moved changes, `onmove` gets the new numbers. `onend` gets `false` if the drag is cancelled.
 */
export function trackCellDrag(
	event: PointerEvent,
	pitch: { x: number; y: number },
	onmove: (dx: number, dy: number) => void,
	onend: (completed: boolean) => void
) {
	const el = event.currentTarget as HTMLElement;
	el.setPointerCapture(event.pointerId);
	event.preventDefault();

	let lastX = 0;
	let lastY = 0;
	const move = (e: PointerEvent) => {
		const dx = Math.round((e.clientX - event.clientX) / pitch.x);
		const dy = Math.round((e.clientY - event.clientY) / pitch.y);
		if (dx === lastX && dy === lastY) return;
		lastX = dx;
		lastY = dy;
		onmove(dx, dy);
	};
	const end = (e: PointerEvent) => {
		el.removeEventListener('pointermove', move);
		el.removeEventListener('pointerup', end);
		el.removeEventListener('pointercancel', end);
		onend(e.type === 'pointerup');
	};
	el.addEventListener('pointermove', move);
	el.addEventListener('pointerup', end);
	el.addEventListener('pointercancel', end);
}
