import type { ThemeSettings } from './types';

/**
 * The CSS variables from the theme settings. This function sets only the accent. The neutral
 * colors, such as background, card and muted, stay in layout.css under `:root` and `.dark`.
 */
export function themeVars({ mode, hue, chroma, radius }: ThemeSettings): Record<string, string> {
	const dark = mode === 'dark';
	const color = (lightness: number, c = chroma, h = hue) => `oklch(${lightness} ${c} ${h % 360})`;
	const primary = color(dark ? 0.72 : 0.55);

	const vars: Record<string, string> = {
		'--primary': primary,
		'--primary-foreground': color(dark ? 0.2 : 0.98, chroma / 4),
		'--ring': color(0.65, chroma / 2),
		'--sidebar-primary': primary,
		'--radius': `${radius}rem`
	};
	// Charts need different colors also with a neutral accent, so they always keep some chroma.
	for (let i = 0; i < 5; i++) {
		vars[`--chart-${i + 1}`] = color(dark ? 0.7 : 0.6, Math.max(chroma, 0.12), hue + i * 50);
	}
	return vars;
}
