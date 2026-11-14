import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { cleanupOf } from './events';

/**
 * Gives the keyboard focus to the page of this window (see apps/desktop/src/focus.rs). Windows
 * allows this only right after user input to this app, for example a global shortcut. Resolves
 * to true if the focus came from another window.
 */
export const borrowFocus = () => invoke<boolean>('focus_borrow');

export const returnFocus = () => invoke('focus_return');

const UNTYPED = new Set([
	'button',
	'checkbox',
	'color',
	'file',
	'hidden',
	'image',
	'radio',
	'range',
	'reset',
	'submit'
]);

export function isTextField(target: EventTarget | null): boolean {
	const el = target as Partial<HTMLInputElement> | null;
	if (el?.isContentEditable || el?.tagName === 'TEXTAREA') return true;
	return el?.tagName === 'INPUT' && !UNTYPED.has(el.type ?? 'text');
}

/** Marks the keyboard button of a web view. A touch on it does not give the focus back. */
export const KEYBOARD_BUTTON = 'data-keyboard-button';

const isKeyboardButton = (target: EventTarget) =>
	(target as Partial<Element>).hasAttribute?.(KEYBOARD_BUTTON) === true;

/** True when a text field or a web view took the focus from another app, and the Edge has it. */
let lent = false;

const borrow = (view?: string) =>
	invoke<boolean>('focus_borrow', { view }).then((took) => {
		if (took) lent = true;
	}, console.error);

/**
 * Gives the keyboard to web view `view`, a label from `webview_open`. The web view keeps it untill
 * the user touches something else on the Edge.
 */
export function lendFocusToView(view: string) {
	void borrow(view);
}

/**
 * Lends the keyboard focus to the text fields on the Edge, because the Edge window cannot take
 * it. A text field borrows the focus when the user touches it. It gives the focus back when the
 * user touches something else on the Edge. Returns a function that stops this.
 */
export function lendFocusToTextFields(): () => void {
	const touched = (event: PointerEvent) => {
		// In the shadow root of a widget, `target` is the widget host. The first item of the path is
		// the element that the user touched.
		const path = event.composedPath();
		// Always ask the host. The page cannot tell where the keyboard is (see focus.rs).
		if (isTextField(path[0])) void borrow();
		else if (lent && !path.some(isKeyboardButton)) {
			lent = false;
			returnFocus().catch(console.error);
		}
	};
	// If the user goes to another app, there is nothing to give back. Use the focus event of the
	// window, becuase the blur event of the page also fires when the host moves the focus.
	const stopListening = cleanupOf(
		getCurrentWindow().onFocusChanged(({ payload: focused }) => {
			if (!focused) lent = false;
		})
	);
	document.addEventListener('pointerdown', touched, true);
	return () => {
		document.removeEventListener('pointerdown', touched, true);
		stopListening();
		lent = false;
	};
}
