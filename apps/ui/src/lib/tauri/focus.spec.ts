import { invoke } from '@tauri-apps/api/core';
import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { flush } from '#lib/test-helpers.ts';
import { isTextField, KEYBOARD_BUTTON, lendFocusToTextFields, lendFocusToView } from './focus';

/** True if the mock `focus_borrow` takes the focus from another window. */
let takes = true;
vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));
const host = vi.mocked(invoke);

let focusChanged: (focused: boolean) => void;
vi.mock('@tauri-apps/api/window', () => ({
	getCurrentWindow: () => ({
		onFocusChanged: async (listener: (event: { payload: boolean }) => void) => {
			focusChanged = (payload) => listener({ payload });
			return () => {};
		}
	})
}));

const element = (fields: Record<string, unknown>) => fields as unknown as EventTarget;
const textField = element({ tagName: 'INPUT', type: 'text' });
const button = element({ tagName: 'BUTTON' });
/** The host element of a widget. Its shadow root holds the fields. */
const widget = element({ tagName: 'DIV' });
/** The button that gives the keyboard to a web view. */
const keyboardButton = element({
	tagName: 'BUTTON',
	hasAttribute: (name: string) => name === KEYBOARD_BUTTON
});

let touch: (target: object) => void;
let stop: () => void;

beforeEach(() => {
	takes = true;
	host.mockImplementation(async (command) => (command === 'focus_borrow' ? takes : undefined));
	vi.stubGlobal('document', {
		addEventListener: (_: string, listener: (event: object) => void) =>
			(touch = (target) => listener({ target: widget, composedPath: () => [target, widget] })),
		removeEventListener: vi.fn()
	});
	stop = lendFocusToTextFields();
});

afterEach(() => {
	stop();
	host.mockReset();
	vi.unstubAllGlobals();
});

const sent = () => host.mock.calls.map(([command]) => command);

it('knows the fields that take typing', () => {
	expect(isTextField(textField)).toBe(true);
	expect(isTextField(element({ tagName: 'INPUT', type: 'search' }))).toBe(true);
	expect(isTextField(element({ tagName: 'TEXTAREA' }))).toBe(true);
	expect(isTextField(element({ tagName: 'DIV', isContentEditable: true }))).toBe(true);
	expect(isTextField(element({ tagName: 'INPUT', type: 'checkbox' }))).toBe(false);
	expect(isTextField(element({ tagName: 'INPUT', type: 'range' }))).toBe(false);
	expect(isTextField(button)).toBe(false);
	expect(isTextField(null)).toBe(false);
});

it('borrows the focus when a text field is touched or clicked, whatever the page thinks', () => {
	touch(textField);
	expect(host).toHaveBeenCalledExactlyOnceWith('focus_borrow', { view: undefined });
});

it('leaves the focus where it is when anything else is touched', () => {
	touch(button);
	expect(sent()).toEqual([]);
});

it('gives it back once something else on the Edge is touched, once', async () => {
	touch(textField);
	await flush();
	touch(button);
	touch(button);
	expect(sent()).toEqual(['focus_borrow', 'focus_return']);
});

it("gives nothing back when it took nothing: the Edge had the focus (Go to page's box)", async () => {
	takes = false;
	touch(textField);
	await flush();
	touch(button);
	expect(sent()).toEqual(['focus_borrow']);
});

it('gives nothing back once the user went to another app', async () => {
	touch(textField);
	await flush();
	focusChanged(false);
	touch(button);
	expect(sent()).toEqual(['focus_borrow']);
});

it('gives a web view the keyboard', () => {
	lendFocusToView('view-1');
	expect(host).toHaveBeenCalledExactlyOnceWith('focus_borrow', { view: 'view-1' });
});

it("keeps it lent when the view's keyboard button is touched", async () => {
	lendFocusToView('view-1');
	await flush();
	touch(keyboardButton);
	expect(sent()).toEqual(['focus_borrow']);
});

it('gives it back from a web view once something else on the Edge is touched', async () => {
	lendFocusToView('view-1');
	await flush();
	touch(button);
	expect(sent()).toEqual(['focus_borrow', 'focus_return']);
});

it('moves it from a web view to a touched text field', async () => {
	lendFocusToView('view-1');
	await flush();
	touch(textField);
	expect(sent()).toEqual(['focus_borrow', 'focus_borrow']);
});
