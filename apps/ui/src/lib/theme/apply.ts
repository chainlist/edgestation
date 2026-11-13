import type { ThemeSettings } from './types';
import { themeVars } from './vars';

/** Applies the theme to `<html>`. The widgets get the variables through their shadow roots. */
export function applyTheme(root: HTMLElement, theme: ThemeSettings) {
	root.classList.toggle('dark', theme.mode === 'dark');
	root.style.colorScheme = theme.mode;
	for (const [name, value] of Object.entries(themeVars(theme))) {
		root.style.setProperty(name, value);
	}
	// The script in app.html reads this value to set the mode before the saved settings load.
	localStorage.setItem('theme-mode', theme.mode);
}
