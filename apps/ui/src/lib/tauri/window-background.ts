import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import type { ThemeMode } from '#lib/theme/types.ts';

/** The `--background` of layout.css in each mode. Rust uses the same colors for a new window. */
const BACKGROUND: Record<ThemeMode, string> = { dark: '#0a0a0a', light: '#ffffff' };

/**
 * Gives the window the background color of the page. The window shows its own color before the
 * page draws and while it resizes or reloads. That color is white by default. The identify boxes
 * keep thier transparent window.
 */
export async function setWindowBackground(mode: ThemeMode) {
	const win = getCurrentWebviewWindow();
	if (win.label.startsWith('identify-')) return;
	await win.setBackgroundColor(BACKGROUND[mode]);
}
