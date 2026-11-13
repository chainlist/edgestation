import { vi } from 'vitest';

export const coreModule = () => ({
	invoke: vi.fn(),
	Channel: class {
		constructor(public onmessage: (message: unknown) => void) {}
	}
});
// The picture that replaces the view and its buttons need a page to draw in.
export const standInModule = () => ({
	createStandIn: vi.fn(() => ({ show: vi.fn(async () => {}), place: vi.fn(), remove: vi.fn() }))
});
export const cornerButtonsModule = () => {
	const button = (_target: unknown, press: () => void) => ({
		show: vi.fn(),
		place: vi.fn(),
		remove: vi.fn(),
		press
	});
	return { createHomeButton: vi.fn(button), createKeyboardButton: vi.fn(button) };
};
