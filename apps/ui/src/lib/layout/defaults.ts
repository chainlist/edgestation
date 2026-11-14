import type { Page } from './types';

/** The size for a widget whose manifest gives no size. */
export const DEFAULT_WIDGET_SIZE = { w: 2, h: 1 };

/** The size of a widget when it is added to a page. */
export const widgetSize = (widget: { size?: { w: number; h: number } }) =>
	widget.size ?? DEFAULT_WIDGET_SIZE;

export const DEFAULT_PAGES: Page[] = [
	{
		id: 'home',
		name: 'Home',
		cols: 8,
		rows: 2,
		items: [{ id: 'clock', widget: 'clock', x: 0, y: 0, w: 2, h: 1 }]
	}
];
