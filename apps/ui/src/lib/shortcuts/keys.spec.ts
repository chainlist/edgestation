import { describe, expect, it } from 'vitest';
import { keyLabels, record, type KeyPress } from './keys';

const press = (code: string, keyCode: number, held: Partial<KeyPress> = {}): KeyPress => ({
	code,
	keyCode,
	ctrlKey: false,
	altKey: false,
	shiftKey: false,
	metaKey: false,
	...held
});

describe('record', () => {
	it('records modifiers then the key, in the plugin format', () => {
		expect(record(press('PageDown', 34, { ctrlKey: true, altKey: true }))).toEqual({
			shortcut: 'Control+Alt+PageDown'
		});
		expect(record(press('ArrowLeft', 37, { metaKey: true, shiftKey: true }))).toEqual({
			shortcut: 'Shift+Super+ArrowLeft'
		});
	});

	it('names a letter by what it types, not where it sits', () => {
		// The A key of an AZERTY keyboard is where a US keyboard has Q.
		expect(record(press('KeyQ', 65, { ctrlKey: true }))).toEqual({ shortcut: 'Control+KeyA' });
	});

	it('keeps waiting while only modifiers are down', () => {
		expect(record(press('ControlLeft', 17, { ctrlKey: true }))).toEqual({ held: ['Ctrl'] });
		expect(record(press('AltRight', 18, { ctrlKey: true, altKey: true }))).toEqual({
			held: ['Ctrl', 'Alt']
		});
	});

	it('needs Ctrl, Alt or Win, except for keys no app uses', () => {
		expect(record(press('PageDown', 34))).toHaveProperty('error');
		expect(record(press('KeyA', 65, { shiftKey: true }))).toHaveProperty('error');
		expect(record(press('F13', 124))).toEqual({ shortcut: 'F13' });
	});

	it('refuses keys that depend on the layout', () => {
		expect(record(press('Minus', 189, { ctrlKey: true }))).toHaveProperty('error');
	});
});

describe('keyLabels', () => {
	it('prints keys as on a keyboard', () => {
		expect(keyLabels('Control+Alt+PageDown')).toEqual(['Ctrl', 'Alt', 'Page Down']);
		expect(keyLabels('Super+KeyA')).toEqual(['Win', 'A']);
		expect(keyLabels('Shift+Digit1')).toEqual(['Shift', '1']);
		expect(keyLabels('Control+Numpad1')).toEqual(['Ctrl', 'Numpad 1']);
		expect(keyLabels('Alt+ArrowLeft')).toEqual(['Alt', 'Left']);
		expect(keyLabels('Control+MediaTrackNext')).toEqual(['Ctrl', 'Track Next']);
		expect(keyLabels('F13')).toEqual(['F13']);
	});
});
