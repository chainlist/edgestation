import { describe, expect, it } from 'vitest';
import { addedPermissions, declaredPermissions } from './permissions';
import type { WidgetPermission } from './types';

describe('declaredPermissions', () => {
	it('lists them in display order, without repeats or unknown names', () => {
		const permissions = ['fetch', 'camera', 'media', 'fetch'] as WidgetPermission[];
		expect(declaredPermissions({ permissions })).toEqual(['media', 'fetch']);
	});

	it('is empty for a manifest without permissions', () => {
		expect(declaredPermissions({})).toEqual([]);
	});
});

describe('addedPermissions', () => {
	const media = { permissions: ['media'] as WidgetPermission[] };

	it('lists only what the new version asks for in addition', () => {
		const latest = { permissions: ['fetch', 'media'] as WidgetPermission[] };
		expect(addedPermissions(media, latest)).toEqual(['fetch']);
		expect(addedPermissions({}, media)).toEqual(['media']);
	});

	it('is empty when the new version asks for the same or less', () => {
		expect(addedPermissions(media, media)).toEqual([]);
		expect(addedPermissions(media, {})).toEqual([]);
	});
});
