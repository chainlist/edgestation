import { error } from '@sveltejs/kit';
import { pages } from '#lib/docs/nav.ts';
import { renderDoc } from '#lib/server/docs.ts';
import type { EntryGenerator, PageServerLoad } from './$types';

export const entries: EntryGenerator = () => pages.map(({ slug }) => ({ slug }));

export const load: PageServerLoad = async ({ params }) => {
	const doc = await renderDoc(params.slug);
	if (!doc) error(404, 'No such page in the docs');
	return doc;
};
