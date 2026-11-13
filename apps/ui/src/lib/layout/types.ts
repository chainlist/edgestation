/** A block of grid cells. `x` and `y` start at 0. `w` and `h` are numbers of cells. */
export interface Rect {
	x: number;
	y: number;
	w: number;
	h: number;
}

/** A widget instance placed on a page. */
export interface Placement extends Rect {
	/** The instance id. It does not change during the life of the placement. */
	id: string;
	/** The widget id, which is the name of it's folder. */
	widget: string;
	/** The values of the widget settings for this placement, by key. */
	settings?: Record<string, string>;
	/** The id of the browser profile for its `api.webview` views. Not set for Default. */
	profile?: string;
}

/** One swipeable page of the Edge display. */
export interface Page {
	id: string;
	name: string;
	cols: number;
	rows: number;
	items: Placement[];
}

export type GridSize = Pick<Page, 'cols' | 'rows'>;
