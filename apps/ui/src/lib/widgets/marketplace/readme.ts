import { Marked } from 'marked';

const escapeHtml = (text: string) =>
	text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');

function webUrl(href: string, base: string): string | undefined {
	try {
		const url = new URL(href, base);
		return url.protocol === 'https:' || url.protocol === 'http:' ? url.href : undefined;
	} catch {
		return undefined;
	}
}

/**
 * Renders the README of a marketplace widget as HTML for Settings. Its author wrote it, so
 * nothing in it may run there. Its raw HTML shows as text. Only `http(s)` links and images
 * stay. They become absolute against `base`, the URL of the README, so relative links point
 * into its repo.
 */
export function renderReadme(markdown: string, base: string): string {
	const marked = new Marked({
		renderer: {
			html: ({ text }) => escapeHtml(text),
			link({ href, title, tokens }) {
				const text = this.parser.parseInline(tokens);
				const url = webUrl(href, base);
				if (!url) return text;
				const tip = title ? ` title="${escapeHtml(title)}"` : '';
				return `<a href="${escapeHtml(url)}"${tip}>${text}</a>`;
			},
			image({ href, title, text }) {
				const url = webUrl(href, base);
				if (!url) return escapeHtml(text);
				const tip = title ? ` title="${escapeHtml(title)}"` : '';
				return `<img src="${escapeHtml(url)}" alt="${escapeHtml(text)}"${tip}>`;
			}
		}
	});
	return marked.parse(markdown, { async: false });
}
