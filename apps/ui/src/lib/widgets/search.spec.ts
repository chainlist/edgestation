import { describe, expect, it } from 'vitest';
import { ids, installedWidget, listedWidget as listed } from '#lib/test-helpers.ts';
import { filterInstalled, filterListing, matchesQuery } from './search';

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
	const widgets = [
		installedWidget('clock', { name: 'Clock', author: 'EdgeStation', source: 'builtin' }),
		installedWidget('jira', {
			name: 'Jira board',
			description: 'The active sprint',
			source: 'marketplace'
		})
	];

	it('matches the id, name, description and author', () => {
		expect(ids(filterInstalled(widgets, 'sprint'))).toEqual(['jira']);
		expect(ids(filterInstalled(widgets, 'edgestation'))).toEqual(['clock']);
		expect(ids(filterInstalled(widgets, ''))).toEqual(['clock', 'jira']);
	});
});

describe('filterListing', () => {
	const widgets = [
		listed('weather', { manifest: { name: 'Weather', description: 'Forecast for your city' } }),
		listed('clock', { manifest: { name: 'Clock', author: 'Jane' } })
	];

	it('matches every word against id, name, description and author', () => {
		expect(ids(filterListing(widgets, 'city forecast'))).toEqual(['weather']);
		expect(ids(filterListing(widgets, 'JANE'))).toEqual(['clock']);
		expect(ids(filterListing(widgets, 'clock jane weather'))).toEqual([]);
	});

	it('keeps everything for an empty query', () => {
		expect(ids(filterListing(widgets, '  '))).toEqual(['weather', 'clock']);
	});
});
