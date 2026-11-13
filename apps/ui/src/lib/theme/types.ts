export type ThemeMode = 'dark' | 'light';

export interface ThemeSettings {
	mode: ThemeMode;
	/** The accent hue in degrees (oklch). */
	hue: number;
	/** The accent chroma (oklch). A value of 0 gives a neutral accent. */
	chroma: number;
	/** The corner radius in rem. */
	radius: number;
}
