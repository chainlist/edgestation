import type { Page } from './types';

/** The size for a widget whose manifest gives no size. */
export const DEFAULT_WIDGET_SIZE = { w: 2, h: 1 };

export const DEFAULT_PAGES: Page[] = [
	{
		id: 'home',
		name: 'Home',
		cols: 8,
		rows: 2,
		items: [{ id: 'clock', widget: 'clock', x: 0, y: 0, w: 2, h: 1 }]
	}
];
