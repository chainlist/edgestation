// Changes CHANGELOG.md into HTML when the site builds. release-please writes this file at each
// release. None of this code gets to the broswer.
import { marked } from 'marked';
import changelog from '../../../../../CHANGELOG.md?raw';

export function renderChangelog(): string {
	const markdown = changelog
		// The page has its own title.
		.replace(/^# Changelog\s*/, '')
		// Visitors do not need the commit link of each entry, ` ([6189ea4](…/commit/6189ea4…))`.
		.replace(/ \(\[[0-9a-f]{7,40}\]\([^)]*\)\)/g, '');
	return marked.parse(markdown, { async: false });
}
