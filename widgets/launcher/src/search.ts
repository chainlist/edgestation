/** Returns `text` in lowercase and without accents, for search. */
export const fold = (text: string) =>
	text
		.normalize('NFD')
		.replace(/\p{Diacritic}/gu, '')
		.toLowerCase();
