import { describe, expect, it } from 'vitest';
import { changeShortcut, resolveShortcuts, usedBy } from './actions';

describe('resolveShortcuts', () => {
	it('falls back to the defaults', () => {
		expect(resolveShortcuts({})).toEqual({
			previousPage: 'Control+Alt+PageUp',
			nextPage: 'Control+Alt+PageDown',
			goToPage: 'Control+Alt+KeyG',
			bringToFront: 'Control+Alt+Home',
			toggleEdge: 'Control+Alt+KeyT'
		});
	});

	it('keeps changed and turned off shortcuts', () => {
		expect(resolveShortcuts({ previousPage: 'Control+KeyA', nextPage: null })).toEqual({
			previousPage: 'Control+KeyA',
			nextPage: null,
			goToPage: 'Control+Alt+KeyG',
			bringToFront: 'Control+Alt+Home',
			toggleEdge: 'Control+Alt+KeyT'
		});
	});

	it('ignores actions that no longer exist', () => {
		expect(resolveShortcuts({ gone: 'F13' } as never)).not.toHaveProperty('gone');
	});
});

describe('changeShortcut', () => {
	it('saves a change', () => {
		expect(changeShortcut({}, 'nextPage', 'F13')).toEqual({ nextPage: 'F13' });
		expect(changeShortcut({}, 'nextPage', null)).toEqual({ nextPage: null });
	});

	it('forgets a change back to the default', () => {
		expect(changeShortcut({ nextPage: 'F13' }, 'nextPage', 'Control+Alt+PageDown')).toEqual({});
	});
});

describe('usedBy', () => {
	it('finds another action with the same shortcut', () => {
		const shortcuts = resolveShortcuts({});
		expect(usedBy(shortcuts, 'Control+Alt+PageUp', 'nextPage')).toBe('previousPage');
		expect(usedBy(shortcuts, 'Control+Alt+PageUp', 'previousPage')).toBeUndefined();
		expect(usedBy(shortcuts, 'F13', 'nextPage')).toBeUndefined();
	});
});
