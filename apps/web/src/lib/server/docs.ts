// Changes the Markdown of the docs into HTML when the site builds.
// None of this code gets to the browser.
import { posix } from 'node:path';
import { Marked, Renderer, type Tokens } from 'marked';
import { createHighlighter } from 'shiki';

const files = import.meta.glob<string>('../docs/pages/**/*.md', {
	query: '?raw',
	import: 'default',
	eager: true
});

const THEME = 'github-dark-default';

const highlighter = createHighlighter({
	themes: [THEME],
	langs: ['bash', 'bat', 'css', 'html', 'js', 'json', 'jsx', 'svelte', 'ts', 'tsx', 'vue']
});

/** A section heading of a page, for its "On this page" list. */
export interface Heading {
	id: string;
	html: string;
	depth: number;
}

export interface RenderedDoc {
	html: string;
	headings: Heading[];
}

const escapeHtml = (text: string) =>
	text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');

/** `api.system()` → `api-system`. */
const slugify = (text: string) =>
	text
		.toLowerCase()
		.replace(/<[^>]*>|[`*_]/g, '')
		.replace(/[^a-z0-9]+/g, '-')
		.replace(/^-|-$/g, '');

/**
 * Makes the site path `to`, such as `/docs/manifest#size`, relative to the page `from`. Then the
 * link works under any base path of the site.
 */
function relativeLink(from: string, to: string): string {
	const [path, hash] = to.split('#');
	const dir = posix.relative(posix.dirname(from), posix.dirname(path));
	const link = dir ? `${dir}/${posix.basename(path)}` : posix.basename(path);
	return hash === undefined ? link : `${link}#${hash}`;
}

/** The HTML of the docs page `slug` ('' for `/docs`), or `undefined` if there is no such page. */
export async function renderDoc(slug: string): Promise<RenderedDoc | undefined> {
	const markdown = files[`../docs/pages/${slug || 'index'}.md`];
	if (markdown === undefined) return undefined;

	const shiki = await highlighter;
	const pagePath = slug ? `/docs/${slug}` : '/docs';
	const headings: Heading[] = [];
	const ids = new Set<string>();

	const marked = new Marked({
		walkTokens(token) {
			if (token.type === 'link' && token.href.startsWith('/')) {
				token.href = relativeLink(pagePath, token.href);
			}
		},
		renderer: {
			heading({ tokens, depth, text }) {
				const html = this.parser.parseInline(tokens);
				let id = slugify(text);
				for (let n = 2; ids.has(id); n++) id = `${slugify(text)}-${n}`;
				ids.add(id);
				headings.push({ id, html, depth });
				return `<h${depth} id="${id}"><a class="anchor" href="#${id}">${html}</a></h${depth}>\n`;
			},
			// ```js vite.config.js: the language, then an optonal file name that shows above the code.
			code({ text, lang }) {
				const [language = 'text', ...title] = (lang ?? '').trim().split(/\s+/);
				const known = shiki.getLoadedLanguages().includes(language);
				const code = shiki.codeToHtml(text, { lang: known ? language : 'text', theme: THEME });
				const caption = title.length
					? `<figcaption>${escapeHtml(title.join(' '))}</figcaption>`
					: '';
				return `<figure class="code">${caption}${code}</figure>\n`;
			},
			// A wide table scrolls in its own box, so the page does not scroll.
			table(token: Tokens.Table) {
				return `<div class="table">${Renderer.prototype.table.call(this, token)}</div>\n`;
			}
		}
	});

	return { html: await marked.parse(markdown), headings };
}
