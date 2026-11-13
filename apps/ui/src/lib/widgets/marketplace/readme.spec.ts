import { describe, expect, it } from 'vitest';
import { renderReadme } from './readme';

const BASE = 'https://raw.githubusercontent.com/jane/weather/HEAD/README.md';

describe('renderReadme', () => {
	it('shows raw HTML as text', () => {
		const html = renderReadme('<script>alert(1)</script>\n\nHi <img src=x onerror=alert(1)>', BASE);
		expect(html).not.toContain('<script');
		expect(html).not.toContain('<img src=x');
		expect(html).toContain('&lt;script&gt;');
	});

	it('drops links and images that are not http(s)', () => {
		const html = renderReadme(
			'[click](javascript:alert(1)) ![pic](data:image/png;base64,AA)',
			BASE
		);
		expect(html).not.toContain('javascript:');
		expect(html).not.toContain('data:');
		expect(html).toContain('click');
	});

	it('points relative links and images into the repo', () => {
		const html = renderReadme('[docs](docs/usage.md) ![shot](shot.png "Shot")', BASE);
		expect(html).toContain(
			'href="https://raw.githubusercontent.com/jane/weather/HEAD/docs/usage.md"'
		);
		expect(html).toContain(
			'<img src="https://raw.githubusercontent.com/jane/weather/HEAD/shot.png" alt="shot" title="Shot">'
		);
	});

	it('keeps https links and escapes their attributes', () => {
		const html = renderReadme('[site](https://example.com/?a="b")', BASE);
		expect(html).toContain('href="https://example.com/?a=%22b%22"');
	});
});
