import { describe, expect, it } from 'vitest';
import { settingsTransition } from './transition';

const sections = [
	{ href: '/settings/pages' },
	{ href: '/settings/widgets' },
	{ href: '/settings/appearance' }
];

describe('settingsTransition', () => {
	it('follows the sidebar between sections', () => {
		expect(settingsTransition('/settings/pages', '/settings/appearance', sections)).toBe('down');
		expect(settingsTransition('/settings/appearance', '/settings/widgets', sections)).toBe('up');
	});
	it('goes forward into a sub-page and back out of it', () => {
		expect(settingsTransition('/settings/pages', '/settings/pages/home', sections)).toBe('forward');
		expect(settingsTransition('/settings/pages/home', '/settings/pages', sections)).toBe('back');
	});
	it('leaves a sub-page for another section along the sidebar', () => {
		expect(settingsTransition('/settings/pages/home', '/settings/widgets', sections)).toBe('down');
	});
	it('stays still on the same page', () => {
		expect(settingsTransition('/settings/appearance', '/settings/appearance', sections)).toBeNull();
	});
});
