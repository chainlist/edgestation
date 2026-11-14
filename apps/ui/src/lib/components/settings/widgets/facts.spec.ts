import { describe, expect, it } from 'vitest';
import { installedWidget as installed, listedWidget as listed } from '#lib/test-helpers.ts';
import { publisherFacts } from './facts';

describe('publisherFacts', () => {
	it('verifies the author of a built-in widget only', () => {
		expect(publisherFacts({ id: 'clock', source: 'builtin', author: 'EdgeStation' }, [])).toEqual([
			{ text: 'EdgeStation', verified: true }
		]);
		expect(publisherFacts({ id: 'mine', source: 'local', author: 'Me' }, [])).toEqual([
			{ text: 'Me', verified: false }
		]);
	});

	it('adds the GitHub owner of a marketplace widget', () => {
		const listing = [listed('jira', { repo: 'ChainList/edgestation-jira' })];
		expect(publisherFacts(installed('jira'), listing)).toEqual([
			{ text: '@ChainList', title: 'Owner of its GitHub repository', verified: true }
		]);
	});
});
