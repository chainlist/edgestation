import { expect, it } from 'vitest';
import { mobileUserAgent } from './mobile-user-agent';

const WEBVIEW2 =
	'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36 Edg/141.0.0.0';
const WEBKITGTK =
	'Mozilla/5.0 (X11; Ubuntu; Linux x86_64) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15';

it('uses the Chrome version of the engine', () => {
	const { userAgent, userAgentMetadata } = mobileUserAgent(WEBVIEW2);
	expect(userAgent).toContain(' Chrome/141.0.0.0 Mobile Safari/537.36');
	expect(userAgentMetadata.brands[0]).toEqual({ brand: 'Chromium', version: '141' });
});

it('uses a recent Chrome version when the engine has none', () => {
	const { userAgent, userAgentMetadata } = mobileUserAgent(WEBKITGTK);
	expect(userAgent).toMatch(/ Chrome\/\d+\.0\.0\.0 Mobile Safari\/537\.36$/);
	expect(Number(userAgentMetadata.brands[0].version)).toBeGreaterThan(100);
});
