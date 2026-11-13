/**
 * A recent Chrome version, for an engine that has none in its user agent: WebKitGTK on Linux.
 * Sites only complain about versions that are too old.
 */
const RECENT_CHROME = '150.0.0.0';

/**
 * Returns what Chrome on a phone sends, so that sites that read it serve their mobile layout.
 * This is the user agent and its client hints. Google sites read the client hints first. It
 * uses the version of the browser engine of the app, because some sites complain about an old
 * version. The shape is that of the DevTools `Emulation.setUserAgentOverride` method. WebKitGTK
 * takes only the user agent: it has no client hints.
 */
export const mobileUserAgent = (userAgent = navigator.userAgent) => {
	const version = /Chrome\/([\d.]+)/.exec(userAgent)?.[1] ?? RECENT_CHROME;
	const major = version.split('.')[0];
	return {
		userAgent: `Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/${version} Mobile Safari/537.36`,
		userAgentMetadata: {
			brands: [
				{ brand: 'Chromium', version: major },
				{ brand: 'Google Chrome', version: major },
				{ brand: 'Not_A Brand', version: '99' }
			],
			// Without these, the engine keeps its own values: Microsoft Edge WebView2, Desktop.
			fullVersionList: [
				{ brand: 'Chromium', version },
				{ brand: 'Google Chrome', version },
				{ brand: 'Not_A Brand', version: '99.0.0.0' }
			],
			formFactors: ['Mobile'],
			platform: 'Android',
			platformVersion: '10.0.0',
			architecture: '',
			model: 'K',
			mobile: true
		}
	};
};
