/**
 * Holes in a native view (`api.webview`) show the page under it. The view draws above the page,
 * so page elements over the view, such as its round buttons, need a hole to show.
 */

/** A box in CSS pixels from the top left of a view. `radius` rounds its corners. */
export interface Hole {
	x: number;
	y: number;
	width: number;
	height: number;
	radius: number;
}

/** The diameter of the round view buttons, and their space from the view edges and each other. */
export const BUTTON_SIZE = 48;
const SPACE = 8;

/**
 * Returns the position of button `slot` in a view of `width` by `height` CSS pixels. Slots count
 * up from the bottom right corner. The position is from the top left corner of the view.
 */
export const buttonSpot = (width: number, height: number, slot: number) => ({
	x: width - SPACE - BUTTON_SIZE,
	y: height - (SPACE + BUTTON_SIZE) * (slot + 1)
});

/**
 * Returns the hole that a view of `width` by `height` CSS pixels leaves for its button `slot`.
 * The hole is one pixel inside the button on all sides. The cut edge of the view is not smooth,
 * so it covers the smoth edge of the button.
 */
export function buttonHole(width: number, height: number, slot: number): Hole {
	const { x, y } = buttonSpot(width, height, slot);
	const side = BUTTON_SIZE - 2;
	return { x: x + 1, y: y + 1, width: side, height: side, radius: side / 2 };
}
