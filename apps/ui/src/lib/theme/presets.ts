import type { ThemeSettings } from './types';

export interface Accent {
	name: string;
	hue: number;
	chroma: number;
}

export const ACCENTS: Accent[] = [
	{ name: 'Neutral', hue: 0, chroma: 0 },
	{ name: 'Blue', hue: 260, chroma: 0.17 },
	{ name: 'Violet', hue: 293, chroma: 0.2 },
	{ name: 'Rose', hue: 10, chroma: 0.2 },
	{ name: 'Amber', hue: 75, chroma: 0.16 },
	{ name: 'Emerald', hue: 160, chroma: 0.15 },
	{ name: 'Cyan', hue: 215, chroma: 0.13 }
];

export const DEFAULT_THEME: ThemeSettings = { mode: 'dark', hue: 260, chroma: 0.17, radius: 0.75 };
