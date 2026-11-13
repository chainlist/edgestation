import { describe, expect, it } from 'vitest';
import { cellStyle, clampRect, dragRect, findSpot, isFree, overlaps, swapRects } from './grid';

const grid = { cols: 4, rows: 2 };

describe('overlaps', () => {
	it('detects shared cells', () => {
		expect(overlaps({ x: 0, y: 0, w: 2, h: 1 }, { x: 1, y: 0, w: 1, h: 1 })).toBe(true);
	});
	it('treats touching edges as free', () => {
		expect(overlaps({ x: 0, y: 0, w: 2, h: 1 }, { x: 2, y: 0, w: 1, h: 1 })).toBe(false);
	});
});

describe('clampRect', () => {
	it('pulls a block back inside the grid', () => {
		expect(clampRect(grid, { x: 3, y: 1, w: 2, h: 2 })).toEqual({ x: 2, y: 0, w: 2, h: 2 });
	});
	it('shrinks a block larger than the grid', () => {
		expect(clampRect(grid, { x: 0, y: 0, w: 9, h: 9 })).toEqual({ x: 0, y: 0, w: 4, h: 2 });
	});
});

describe('findSpot', () => {
	it('skips taken cells row by row', () => {
		const taken = [{ x: 0, y: 0, w: 3, h: 1 }];
		expect(findSpot(grid, taken, 2, 1)).toEqual({ x: 0, y: 1, w: 2, h: 1 });
	});
	it('covers the fewest taken cells when no spot is free', () => {
		// Only the cell at x 3, y 1 is free.
		const taken = [
			{ x: 0, y: 0, w: 4, h: 1 },
			{ x: 0, y: 1, w: 3, h: 1 }
		];
		expect(findSpot(grid, taken, 2, 1)).toEqual({ x: 2, y: 1, w: 2, h: 1 });
	});
	it('takes the first spot when all spots cover the same number of taken cells', () => {
		expect(findSpot(grid, [{ x: 0, y: 0, w: 4, h: 2 }], 1, 1)).toEqual({ x: 0, y: 0, w: 1, h: 1 });
	});
	it('shrinks a block larger than the grid', () => {
		expect(findSpot(grid, [], 6, 3)).toEqual({ x: 0, y: 0, w: 4, h: 2 });
	});
});

describe('dragRect', () => {
	const rect = { x: 1, y: 0, w: 2, h: 1 };

	it('moves by whole cells and stops at the grid edge', () => {
		expect(dragRect(grid, rect, 'move', 1, 1)).toEqual({ x: 2, y: 1, w: 2, h: 1 });
		expect(dragRect(grid, rect, 'move', 5, 0)).toEqual({ x: 2, y: 0, w: 2, h: 1 });
	});
	it('resizes from a fixed top-left corner', () => {
		expect(dragRect(grid, rect, 'se', 5, 5)).toEqual({ x: 1, y: 0, w: 3, h: 2 });
	});
	it('keeps the opposite corner fixed when dragging the other corners', () => {
		expect(dragRect(grid, rect, 'nw', -5, 0)).toEqual({ x: 0, y: 0, w: 3, h: 1 });
		expect(dragRect(grid, rect, 'sw', -1, 1)).toEqual({ x: 0, y: 0, w: 3, h: 2 });
		const low = { x: 1, y: 1, w: 2, h: 1 };
		expect(dragRect(grid, low, 'ne', 5, -5)).toEqual({ x: 1, y: 0, w: 3, h: 2 });
	});
	it('never shrinks below one cell', () => {
		expect(dragRect(grid, rect, 'se', -5, -5)).toEqual({ x: 1, y: 0, w: 1, h: 1 });
		expect(dragRect(grid, rect, 'nw', 5, 5)).toEqual({ x: 2, y: 0, w: 1, h: 1 });
	});
});

describe('swapRects', () => {
	const block = (id: string, x: number, y: number, w = 1, h = 1) => ({ id, x, y, w, h });

	it('only moves the dragged block onto free cells', () => {
		const blocks = [block('a', 0, 0), block('b', 3, 1)];
		expect(swapRects(grid, blocks, 'a', { x: 1, y: 0, w: 1, h: 1 })).toEqual({
			a: { x: 1, y: 0, w: 1, h: 1 }
		});
	});
	it('swaps two blocks of the same size', () => {
		const blocks = [block('a', 0, 0, 2), block('b', 2, 0, 2)];
		expect(swapRects(grid, blocks, 'a', { x: 2, y: 0, w: 2, h: 1 })).toEqual({
			a: { x: 2, y: 0, w: 2, h: 1 },
			b: { x: 0, y: 0, w: 2, h: 1 }
		});
	});
	it('moves every covered block into the space left, keeping their arrangement', () => {
		const blocks = [
			block('a', 0, 0, 2, 2),
			block('b', 2, 0, 1, 2),
			block('c', 3, 0),
			block('d', 3, 1)
		];
		expect(swapRects(grid, blocks, 'a', { x: 2, y: 0, w: 2, h: 2 })).toEqual({
			a: { x: 2, y: 0, w: 2, h: 2 },
			b: { x: 0, y: 0, w: 1, h: 2 },
			c: { x: 1, y: 0, w: 1, h: 1 },
			d: { x: 1, y: 1, w: 1, h: 1 }
		});
	});
	it('slides a smaller block to the nearest spot in the space left when a straight swap overlaps', () => {
		// a moves one cell right onto b. A straight swap puts b under a again.
		const blocks = [block('a', 0, 0, 2), block('b', 2, 0)];
		expect(swapRects(grid, blocks, 'a', { x: 1, y: 0, w: 2, h: 1 })).toEqual({
			a: { x: 1, y: 0, w: 2, h: 1 },
			b: { x: 0, y: 0, w: 1, h: 1 }
		});
	});
	it('leaves blocks it does not cover where they are', () => {
		const blocks = [block('a', 0, 0), block('b', 1, 0), block('c', 2, 0)];
		expect(swapRects(grid, blocks, 'a', { x: 1, y: 0, w: 1, h: 1 })).toEqual({
			a: { x: 1, y: 0, w: 1, h: 1 },
			b: { x: 0, y: 0, w: 1, h: 1 }
		});
	});
	it('returns null when the covered blocks cannot take the space left', () => {
		// b is 2 cells wide, but only the one cell of a becomes free next to it.
		const blocks = [block('a', 0, 0), block('b', 1, 0, 2), block('c', 0, 1, 4), block('d', 3, 0)];
		expect(swapRects(grid, blocks, 'a', { x: 1, y: 0, w: 1, h: 1 })).toBeNull();
	});
	it('never sends the covered blocks away from the space left', () => {
		// b fits in the free bottom row, but a was not there.
		const blocks = [block('a', 0, 0), block('b', 1, 0, 3)];
		expect(swapRects(grid, blocks, 'a', { x: 2, y: 0, w: 1, h: 1 })).toBeNull();
	});
});

it('isFree rejects a spot that overlaps a taken block', () => {
	const taken = [{ x: 0, y: 0, w: 2, h: 1 }];
	expect(isFree(taken, { x: 1, y: 0, w: 1, h: 1 })).toBe(false);
	expect(isFree(taken, { x: 2, y: 0, w: 1, h: 1 })).toBe(true);
});

it('cellStyle converts 0-based cells to 1-based grid lines', () => {
	expect(cellStyle({ x: 1, y: 0, w: 2, h: 1 })).toBe('grid-area:1 / 2 / span 1 / span 2');
});
