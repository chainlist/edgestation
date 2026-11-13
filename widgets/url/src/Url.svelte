<script lang="ts">
	import type { WidgetApi } from '@edgestation/sdk';

	let props: { api: WidgetApi } = $props();
	// `api` does not change. A changed setting mounts the widget again.
	// svelte-ignore state_referenced_locally
	const { api } = props;

	const typed = api.settings.url.trim();

	/**
	 * Returns the page to open, or `null` if `text` is not a web address. An address without a
	 * scheme (`example.com`) opens with https.
	 */
	function webAddress(text: string): URL | null {
		try {
			const url = new URL(text.includes('://') ? text : `https://${text}`);
			return url.protocol === 'http:' || url.protocol === 'https:' ? url : null;
		} catch {
			return null;
		}
	}

	const url = typed ? webAddress(typed) : null;
	const mobile = api.settings.layout === 'mobile';
</script>

{#if url}
	<!-- The page shows in a native view over this box (`api.webview`). The icon and the name
	     show only until the view appears for the first time. -->
	<div class="url" {@attach (el) => api.webview(el, url.href, { mobile, homeButton: true })}>
		<svg class="icon" viewBox="0 0 24 24" aria-hidden="true">
			<circle cx="12" cy="12" r="10" />
			<path
				d="M2 12h20M12 2c3 3.2 4.5 6.5 4.5 10S15 18.8 12 22c-3-3.2-4.5-6.5-4.5-10S9 5.2 12 2z"
			/>
		</svg>
		<span class="name">{url.hostname}</span>
	</div>
{:else}
	<div class="url">
		<span class="name">{typed ? 'Not a web address' : 'No page yet'}</span>
		<span class="hint">
			{typed || 'Set its URL in Settings → Pages, by clicking this widget.'}
		</span>
	</div>
{/if}

<style>
	.url {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 3cqh;
		height: 100%;
		padding: 0 6cqw;
		color: var(--muted-foreground);
		text-align: center;
	}

	.icon {
		width: min(24cqh, 24cqw);
		fill: none;
		stroke: currentColor;
		stroke-width: 1.5;
		stroke-linejoin: round;
	}

	.name {
		max-width: 100%;
		overflow: hidden;
		font-size: min(8cqh, 6cqw);
		font-weight: 600;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.hint {
		max-width: 100%;
		overflow: hidden;
		font-size: min(5cqh, 4cqw);
		text-overflow: ellipsis;
		white-space: nowrap;
	}
</style>
