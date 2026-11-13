<script lang="ts">
	import type { WidgetApi } from '@edgestation/sdk';

	let props: { api: WidgetApi } = $props();
	// `api` does not change. A changed setting mounts the widget again.
	// svelte-ignore state_referenced_locally
	const { api } = props;

	/**
	 * The start page shows the latest videos of the channels that you subscibe to. `persist_app`
	 * keeps the layout for the pages that open from there. It sets a cookie that all YouTube views
	 * of the app share. Both URLs set it. If one did not, the last layout would apply to all views.
	 */
	const SUBSCRIPTIONS = {
		desktop: 'https://www.youtube.com/feed/subscriptions?app=desktop&persist_app=1',
		// The mobile player uses the full width of the cell, so it fits only in narrow cells.
		mobile: 'https://m.youtube.com/feed/subscriptions?app=m&persist_app=1'
	};
	const url = api.settings.layout === 'mobile' ? SUBSCRIPTIONS.mobile : SUBSCRIPTIONS.desktop;
</script>

<!-- YouTube does not load in iframes, so it shows in a native view over this box. The icon and
     the name show only until the view appears for the first time. -->
<div class="youtube" {@attach (el) => api.webview(el, url)}>
	<svg class="icon" viewBox="0 0 24 24" aria-hidden="true">
		<rect x="1" y="4" width="22" height="16" rx="5" />
		<path d="M10 8.5v7l6-3.5z" />
	</svg>
	<span class="name">YouTube</span>
</div>

<style>
	.youtube {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 3cqh;
		height: 100%;
		color: var(--muted-foreground);
	}

	.icon {
		width: min(24cqh, 24cqw);
		fill: none;
		stroke: currentColor;
		stroke-width: 1.5;
		stroke-linejoin: round;
	}

	.icon path {
		fill: currentColor;
	}

	.name {
		font-size: min(8cqh, 6cqw);
		font-weight: 600;
	}
</style>
