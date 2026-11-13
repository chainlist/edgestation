import type { InstalledWidget } from './types';

/** Tells if each word of `query` is in one of `fields`. Case does not matter. */
export function matchesQuery(query: string, fields: (string | null | undefined)[]): boolean {
	const text = fields.join(' ').toLowerCase();
	return query
		.toLowerCase()
		.split(/\s+/)
		.every((word) => text.includes(word));
}

/** The installed widgets whose id, name, description or author match `query`. */
export const filterInstalled = (widgets: InstalledWidget[], query: string) =>
	widgets.filter((widget) =>
		matchesQuery(query, [widget.id, widget.name, widget.description, widget.author])
	);
