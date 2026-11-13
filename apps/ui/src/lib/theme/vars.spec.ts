import { expect, it } from 'vitest';
import { themeVars } from './vars';

it('derives the accent from hue and chroma for the current mode', () => {
	const dark = themeVars({ mode: 'dark', hue: 260, chroma: 0.2, radius: 0.5 });
	const light = themeVars({ mode: 'light', hue: 260, chroma: 0.2, radius: 0.5 });
	expect(dark['--primary']).toBe('oklch(0.72 0.2 260)');
	expect(light['--primary']).toBe('oklch(0.55 0.2 260)');
	expect(dark['--radius']).toBe('0.5rem');
});

it('keeps chart colors distinct with a neutral accent', () => {
	const vars = themeVars({ mode: 'dark', hue: 0, chroma: 0, radius: 0.5 });
	const charts = [1, 2, 3, 4, 5].map((i) => vars[`--chart-${i}`]);
	expect(new Set(charts).size).toBe(5);
});
