/** How `api.webview` opens its page. */
export interface WebviewOptions {
	/**
	 * Shows the view to sites as Chrome on a phone, with the user agent and client hints. This
	 * applies to each page that the view loads. Sites that select their layout from the browser
	 * then serve their mobile layout. Sites that only follow the width fit the cell in both cases.
	 */
	mobile?: boolean;
	/**
	 * Shows a round Home button over the bottom right corner of the view, above the keyboard
	 * button. The button shows while the view is away from `url`, and takes the view back there.
	 * The view leaves a hole for it, because the button is an element in the root of the widget,
	 * like the picture. A page that `url` redirects to when it loads counts as `url`.
	 */
	homeButton?: boolean;
}
