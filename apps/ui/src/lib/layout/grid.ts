import { clamp } from '#lib/clamp.ts';
import type { GridSize, Rect } from './types';

/**
 * The Xeneon Edge panel in pixels. EdgeStation is made for this display. The app uses this size
 * until it knows the size of the Edge window, on any display.
 */
export const XENEON_EDGE = { width: 2560, height: 720 };

export const EDGE_ASPECT = XENEON_EDGE.width / XENEON_EDGE.height;

export const MAX_COLS = 16;
export const MAX_ROWS = 8;

export const gridStyle = ({ cols, rows }: GridSize) =>
	`grid-template-columns:repeat(${cols},minmax(0,1fr));grid-template-rows:repeat(${rows},minmax(0,1fr))`;

export const cellStyle = ({ x, y, w, h }: Rect) =>
	`grid-area:${y + 1} / ${x + 1} / span ${h} / span ${w}`;

export const overlaps = (a: Rect, b: Rect) =>
	a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h;

export const isFree = (taken: Rect[], rect: Rect) => !taken.some((other) => overlaps(other, rect));

export function clampRect(grid: GridSize, rect: Rect): Rect {
	const w = clamp(rect.w, 1, grid.cols);
	const h = clamp(rect.h, 1, grid.rows);
	return { w, h, x: clamp(rect.x, 0, grid.cols - w), y: clamp(rect.y, 0, grid.rows - h) };
}

/** The number of cells that `a` and `b` share. */
const sharedCells = (a: Rect, b: Rect) =>
	Math.max(0, Math.min(a.x + a.w, b.x + b.w) - Math.max(a.x, b.x)) *
	Math.max(0, Math.min(a.y + a.h, b.y + b.h) - Math.max(a.y, b.y));

/**
 * Finds the spot for a `w`×`h` block that covers the fewest taken cells, row by row. Thus it is
 * the first free spot when there is one. A block larger than the grid gets the size of the grid.
 */
export function findSpot(grid: GridSize, taken: Rect[], w: number, h: number): Rect {
	const size = clampRect(grid, { x: 0, y: 0, w, h });
	let best = size;
	let least = Infinity;
	for (let y = 0; y + size.h <= grid.rows; y++) {
		for (let x = 0; x + size.w <= grid.cols; x++) {
			const rect = { ...size, x, y };
			const cells = taken.reduce((sum, other) => sum + sharedCells(rect, other), 0);
			if (cells === 0) return rect;
			if (cells < least) {
				best = rect;
				least = cells;
			}
		}
	}
	return best;
}

/**
 * Moves block `id` to `to`. The blocks under `to` move into the space that block `id` leaves.
 * They move as one group and keep their arrangment, as near as possible to a straight swap.
 * They must cover part of that space, so they cannot jump to another part of the grid. Returns
 * the new rect of each block that moves, or `null` if the blocks do not fit.
 */
export function swapRects(
	grid: GridSize,
	blocks: (Rect & { id: string })[],
	id: string,
	to: Rect
): Record<string, Rect> | null {
	const from = blocks.find((block) => block.id === id);
	if (!from) return null;
	const moves: Record<string, Rect> = { [id]: to };
	const others = blocks.filter((block) => block.id !== id);
	const hit = others.filter((block) => overlaps(block, to));
	if (hit.length === 0) return moves;

	const taken = [to, ...others.filter((block) => !overlaps(block, to))];
	const left = Math.min(...hit.map((block) => block.x));
	const top = Math.min(...hit.map((block) => block.y));
	const right = Math.max(...hit.map((block) => block.x + block.w));
	const bottom = Math.max(...hit.map((block) => block.y + block.h));
	// A straight swap moves the group back by the distance that the dragged block moved.
	const ideal = { x: from.x - to.x, y: from.y - to.y };
	let best: { dx: number; dy: number; cost: number } | null = null;
	for (let dy = -top; bottom + dy <= grid.rows; dy++) {
		for (let dx = -left; right + dx <= grid.cols; dx++) {
			const cost = Math.abs(dx - ideal.x) + Math.abs(dy - ideal.y);
			if (best && cost >= best.cost) continue;
			const shifted = hit.map((block) => ({ ...block, x: block.x + dx, y: block.y + dy }));
			if (!shifted.some((block) => overlaps(block, from))) continue;
			if (shifted.every((block) => isFree(taken, block))) best = { dx, dy, cost };
		}
	}
	if (!best) return null;
	for (const { id, x, y, w, h } of hit) moves[id] = { x: x + best.dx, y: y + best.dy, w, h };
	return moves;
}

export type Corner = 'nw' | 'ne' | 'sw' | 'se';
export type DragMode = 'move' | Corner;

/** Gives the new `rect` after a drag of `dx`×`dy` cells in `mode`, inside the grid. */
export function dragRect(grid: GridSize, rect: Rect, mode: DragMode, dx: number, dy: number): Rect {
	if (mode === 'move') return clampRect(grid, { ...rect, x: rect.x + dx, y: rect.y + dy });
	// A resize moves one corner. The opposite corner stays, and the block keeps at least one cell.
	const west = mode.endsWith('w');
	const north = mode.startsWith('n');
	const right = rect.x + rect.w;
	const bottom = rect.y + rect.h;
	const x = west ? clamp(rect.x + dx, 0, right - 1) : rect.x;
	const y = north ? clamp(rect.y + dy, 0, bottom - 1) : rect.y;
	const w = west ? right - x : clamp(rect.w + dx, 1, grid.cols - x);
	const h = north ? bottom - y : clamp(rect.h + dy, 1, grid.rows - y);
	return { x, y, w, h };
}
