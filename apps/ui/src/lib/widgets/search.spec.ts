import { describe, expect, it } from 'vitest';
import { filterInstalled, matchesQuery } from './search';
import type { InstalledWidget } from './types';

describe('matchesQuery', () => {
	it('needs every word in one of the fields, in any case', () => {
		expect(matchesQuery('JIRA board', ['jira', 'Jira board'])).toBe(true);
		expect(matchesQuery('jira clock', ['jira', 'Jira board'])).toBe(false);
	});

	it('skips missing fields and matches all for a blank query', () => {
		expect(matchesQuery('clock', [undefined, null, 'clock'])).toBe(true);
		expect(matchesQuery('  ', [undefined])).toBe(true);
	});
});

describe('filterInstalled', () => {
	const widget = (id: string, more: Partial<InstalledWidget> = {}): InstalledWidget => ({
		id,
		name: id,
		source: 'builtin',
		...more
	});
	const widgets = [
		widget('clock', { name: 'Clock', author: 'EdgeStation' }),
		widget('jira', { name: 'Jira board', description: 'The active sprint', source: 'marketplace' })
	];
	const ids = (list: InstalledWidget[]) => list.map((w) => w.id);

	it('matches the id, name, description and author', () => {
		expect(ids(filterInstalled(widgets, 'sprint'))).toEqual(['jira']);
		expect(ids(filterInstalled(widgets, 'edgestation'))).toEqual(['clock']);
		expect(ids(filterInstalled(widgets, ''))).toEqual(['clock', 'jira']);
	});
});
