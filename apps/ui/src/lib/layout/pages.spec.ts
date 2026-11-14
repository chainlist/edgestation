import { describe, expect, it } from 'vitest';
import {
	findPages,
	leaveProfile,
	movePlacements,
	newPlacement,
	placeWaiting,
	profileUsers,
	resizePage,
	setPlacementProfile,
	setPlacementSetting
} from './pages';
import type { Page } from './types';

const page = (): Page => ({
	id: 'p',
	name: 'Page',
	cols: 4,
	rows: 2,
	items: [{ id: 'a', widget: 'clock', x: 2, y: 0, w: 2, h: 1 }]
});

it('resizePage keeps existing widgets inside the new grid', () => {
	expect(resizePage(page(), 2, 2).items[0]).toMatchObject({ x: 0, w: 2 });
});

describe('newPlacement', () => {
	it('places the widget in the first free spot', () => {
		const item = newPlacement(page(), 'notes', { w: 2, h: 1 });
		expect(item).toMatchObject({ widget: 'notes', x: 0, y: 0, w: 2, h: 1 });
	});
	it('keeps the widget size when no spot is free', () => {
		expect(newPlacement(page(), 'big', { w: 4, h: 2 })).toMatchObject({ x: 0, y: 0, w: 4, h: 2 });
	});
});

describe('placeWaiting', () => {
	const item = (id: string, x: number, y: number, w = 2, h = 1) => ({ id, widget: id, x, y, w, h });

	it('saves the waiting placements that fit and keeps the others', () => {
		const next = placeWaiting(page(), [item('over', 1, 0), item('free', 0, 1)]);
		expect(next.page.items.map((i) => i.id)).toEqual(['a', 'free']);
		expect(next.waiting.map((i) => i.id)).toEqual(['over']);
	});
	it('checks each placement against the ones saved before it', () => {
		const next = placeWaiting(page(), [item('first', 0, 1), item('second', 1, 1)]);
		expect(next.page.items.map((i) => i.id)).toEqual(['a', 'first']);
		expect(next.waiting.map((i) => i.id)).toEqual(['second']);
	});
	it('returns the same page when nothing fits', () => {
		const before = page();
		expect(placeWaiting(before, [item('over', 2, 0)]).page).toBe(before);
	});
	it('moves waiting placements back inside the grid', () => {
		const next = placeWaiting(page(), [item('out', 3, 0)]);
		expect(next.waiting[0]).toMatchObject({ x: 2, y: 0 });
	});
});

it('movePlacements moves each listed placement, clamped to the grid', () => {
	const two: Page = {
		...page(),
		items: [...page().items, { id: 'b', widget: 'notes', x: 0, y: 0, w: 1, h: 1 }]
	};
	const next = movePlacements(two, { a: { x: 3, y: 1, w: 2, h: 1 } });
	expect(next.items[0]).toMatchObject({ x: 2, y: 1, w: 2 });
	expect(next.items[1]).toBe(two.items[1]);
});

it('setPlacementSetting merges one value into the placement settings', () => {
	const once = setPlacementSetting(page(), 'a', 'metric', 'net');
	const twice = setPlacementSetting(once, 'a', 'period', 'avg5m');
	expect(twice.items[0].settings).toEqual({ metric: 'net', period: 'avg5m' });
});

it('setPlacementProfile sets the profile, or clears it for Default', () => {
	const work = setPlacementProfile(page(), 'a', 'work');
	expect(work.items[0].profile).toBe('work');
	expect(setPlacementProfile(work, 'a', undefined).items[0].profile).toBeUndefined();
});

describe('profileUsers', () => {
	const pages: Page[] = [
		{
			...page(),
			id: 'p1',
			items: [
				{ id: 'b', widget: 'url', x: 2, y: 1, w: 2, h: 1 },
				{ id: 'a', widget: 'url', x: 0, y: 1, w: 2, h: 1 },
				{ id: 'c', widget: 'clock', x: 0, y: 0, w: 2, h: 1 }
			]
		},
		{
			...page(),
			id: 'p2',
			items: [
				{ id: 'd', widget: 'youtube', x: 0, y: 0, w: 2, h: 1 },
				{ id: 'e', widget: 'url', profile: 'work', x: 2, y: 0, w: 2, h: 1 }
			]
		}
	];
	const webviews = new Set(['url', 'youtube']);
	const ids = (profile?: string) =>
		profileUsers(pages, profile, webviews).map(({ page, item }) => `${page.id}/${item.id}`);

	it('lists the web view widgets in Default by page, then in reading order', () => {
		expect(ids()).toEqual(['p1/a', 'p1/b', 'p2/d']);
	});
	it('lists only the placements set to that profile', () => {
		expect(ids('work')).toEqual(['p2/e']);
		expect(ids('home')).toEqual([]);
	});
});

it('leaveProfile moves only the placements in that profile to Default', () => {
	const two: Page = {
		...page(),
		items: [
			{ id: 'a', widget: 'url', profile: 'work', x: 0, y: 0, w: 2, h: 1 },
			{ id: 'b', widget: 'url', profile: 'home', x: 2, y: 0, w: 2, h: 1 }
		]
	};
	const [left] = leaveProfile([two], 'work');
	expect(left.items.map((item) => item.profile)).toEqual([undefined, 'home']);
});

describe('findPages', () => {
	const named = (...names: string[]) => names.map((name) => ({ ...page(), name }));
	const pages = named('Home', 'Media', 'Été', 'Work mail', 'Gaming');

	it('lists every page for an empty query', () => {
		expect(findPages(pages, '  ')).toEqual([0, 1, 2, 3, 4]);
	});
	it('ignores case and accents', () => {
		expect(findPages(pages, 'ETE')).toEqual([2]);
		expect(findPages(named('Ete'), 'été')).toEqual([0]);
	});
	it('puts names or words starting with the query before names holding it', () => {
		expect(findPages(pages, 'm')).toEqual([1, 3, 0, 4]);
		expect(findPages(pages, 'work m')).toEqual([3]);
		expect(findPages(pages, 'ing')).toEqual([4]);
	});
	it('finds nothing when no name holds the query', () => {
		expect(findPages(pages, 'zz')).toEqual([]);
	});
});
