/** Helpers for lists of items with an `id`, such as pages, placements and profiles. */
type WithId = { id: string };

export const updateIn = <T extends WithId>(list: T[], id: string, fn: (item: T) => T) =>
	list.map((item) => (item.id === id ? fn(item) : item));

export const removeIn = <T extends WithId>(list: T[], id: string) =>
	list.filter((item) => item.id !== id);

/** Moves item `id` by `delta` positions. A move past either end changes nothing. */
export function moveIn<T extends WithId>(list: T[], id: string, delta: number): T[] {
	const from = list.findIndex((item) => item.id === id);
	const to = from + delta;
	if (from < 0 || to < 0 || to >= list.length) return list;
	const next = [...list];
	next.splice(to, 0, ...next.splice(from, 1));
	return next;
}

export const newId = () => crypto.randomUUID().slice(0, 8);
