import { describe, expect, it } from 'vitest';
import { ids, installedWidget as installed, listedWidget as listed } from '#lib/test-helpers.ts';
import { listingStatus, paginate, repoOwner, sortListing, verifiedOwner } from './listing';

describe('sortListing', () => {
	const widgets = [
		listed('clock', { downloads: 10, released: '2026-01-01T00:00:00Z' }),
		listed('weather', { downloads: 500, released: '2025-06-01T00:00:00Z' }),
		listed('new'),
		listed('alarm', { downloads: 10, released: '2026-03-01T00:00:00Z' })
	];

	it('puts the most downloaded first, ties by name, unknown counts last', () => {
		expect(ids(sortListing(widgets, 'downloads'))).toEqual(['weather', 'alarm', 'clock', 'new']);
	});

	it('puts the most recently released first, unknown dates last', () => {
		expect(ids(sortListing(widgets, 'released'))).toEqual(['alarm', 'clock', 'weather', 'new']);
	});

	it('sorts by name', () => {
		expect(ids(sortListing(widgets, 'name'))).toEqual(['alarm', 'clock', 'new', 'weather']);
	});

	it('leaves its input alone', () => {
		sortListing(widgets, 'name');
		expect(ids(widgets)).toEqual(['clock', 'weather', 'new', 'alarm']);
	});
});

describe('listingStatus', () => {
	it('offers to install a widget that is not installed', () => {
		expect(listingStatus(listed('w'), [], '0.3.0')).toEqual({ kind: 'install' });
	});

	it('offers a newer release of an installed widget', () => {
		const widget = listed('w', { manifest: { name: 'W', version: '1.1.0' } });
		expect(listingStatus(widget, [installed('w')], '0.3.0')).toEqual({
			kind: 'update',
			version: '1.1.0'
		});
		expect(listingStatus(listed('w'), [installed('w')], '0.3.0')).toEqual({ kind: 'installed' });
	});

	it("doesn't offer a release this app is too old for", () => {
		const widget = listed('w', {
			manifest: { name: 'W', version: '2.0.0', minAppVersion: '0.9.0' }
		});
		expect(listingStatus(widget, [], '0.3.0')).toEqual({
			kind: 'unavailable',
			reason: 'Needs EdgeStation 0.9.0 or later.'
		});
		expect(listingStatus(widget, [installed('w')], '0.3.0')).toEqual({ kind: 'installed' });
	});

	it('refuses an id one of your own widgets uses', () => {
		expect(listingStatus(listed('w'), [installed('w', { source: 'local' })], '0.3.0')).toEqual({
			kind: 'unavailable',
			reason: 'One of your own widgets uses this id.'
		});
	});

	it("says why a release can't be installed", () => {
		const widget = listed('w', { manifest: null, error: 'it answered 404' });
		expect(listingStatus(widget, [], '0.3.0')).toEqual({
			kind: 'unavailable',
			reason: "Its latest release can't be read."
		});
	});
});

describe('paginate', () => {
	const items = Array.from({ length: 57 }, (_, i) => i + 1);

	it('cuts a page out of the list', () => {
		expect(paginate(items, 1, 20)).toEqual({
			items: items.slice(20, 40),
			index: 1,
			count: 3,
			first: 21,
			last: 40
		});
	});

	it('gives a short last page', () => {
		const page = paginate(items, 2, 20);
		expect([page.first, page.last, page.items.length]).toEqual([41, 57, 17]);
	});

	it('gives the last page for an index past it', () => {
		expect(paginate(items, 9, 20).index).toBe(2);
		expect(paginate(items.slice(0, 5), 2, 20)).toMatchObject({ index: 0, first: 1, last: 5 });
	});

	it('has one empty page for an empty list', () => {
		expect(paginate([], 0, 20)).toEqual({ items: [], index: 0, count: 1, first: 1, last: 0 });
	});
});

describe('repoOwner', () => {
	const listing = [listed('jira', { repo: 'ChainList/edgestation-jira' })];

	it('reads the owner of the repo of a marketplace widget', () => {
		expect(repoOwner({ id: 'jira', source: 'marketplace' }, listing)).toBe('ChainList');
	});

	it('has none for other widgets, or for widgets missing from the listing', () => {
		expect(repoOwner({ id: 'jira', source: 'local' }, listing)).toBeUndefined();
		expect(repoOwner({ id: 'jira', source: 'builtin' }, listing)).toBeUndefined();
		expect(repoOwner({ id: 'todo', source: 'marketplace' }, listing)).toBeUndefined();
		expect(repoOwner({ id: 'jira', source: 'marketplace' }, undefined)).toBeUndefined();
	});
});

describe('verifiedOwner', () => {
	it('trusts chainlist in any case, and no one else', () => {
		expect(verifiedOwner('ChainList')).toBe(true);
		expect(verifiedOwner('someone')).toBe(false);
		expect(verifiedOwner('constructor')).toBe(false);
	});
});
