import { renderChangelog } from '#lib/server/changelog.ts';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = () => ({ html: renderChangelog() });
