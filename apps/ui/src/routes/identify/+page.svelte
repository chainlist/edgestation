<!--
	The box that "Identify screens" shows in the middle of each monitor. It has the monitor name
	and size, as the display settings list them. The window has the same size as the box. The host
	creates it hiden, with the data in its URL, so a click only has to show it.
-->
<script lang="ts">
	import MonitorIcon from '@lucide/svelte/icons/monitor';
	import { page } from '$app/state';

	const params = page.url.searchParams;
	const full = params.get('name') ?? '';
	// Windows gives monitors names like `\\.\DISPLAY1`. The part after the last backslash stands out.
	const cut = full.lastIndexOf('\\') + 1;
</script>

<main class="identify">
	<MonitorIcon class="icon" strokeWidth={1.5} />
	<p class="name"><span class="prefix">{full.slice(0, cut)}</span>{full.slice(cut)}</p>
	<p class="size">{params.get('width')} × {params.get('height')}</p>
</main>

<style>
	/* The window is transparent, so only the box shows. */
	:global(html:has(.identify)),
	:global(body:has(.identify)) {
		overflow: hidden;
		background: transparent;
	}

	.identify {
		position: fixed;
		inset: 0;
		display: grid;
		place-content: center;
		justify-items: center;
		padding: 0 2rem;
		/* Windows 11 rounds the window corners by 8px. The inner edge follows them. */
		border-radius: 8px;
		color: white;
		background: linear-gradient(180deg, rgb(28 28 32), rgb(6 6 8));
		box-shadow:
			inset 0 1px 0 rgb(255 255 255 / 0.12),
			inset 0 0 0 1px rgb(255 255 255 / 0.08);
		cursor: default;
		user-select: none;
	}

	.identify :global(.icon) {
		width: 1.75rem;
		height: 1.75rem;
		margin-bottom: 0.875rem;
		color: rgb(255 255 255 / 0.55);
	}
	.name {
		max-width: 100%;
		overflow: hidden;
		font-size: 3.25rem;
		font-weight: 650;
		line-height: 1.15;
		letter-spacing: -0.02em;
		white-space: nowrap;
		text-overflow: ellipsis;
	}
	.prefix {
		color: rgb(255 255 255 / 0.35);
		font-weight: 500;
	}
	.size {
		margin-top: 0.375rem;
		font-size: 0.9375rem;
		font-weight: 500;
		font-variant-numeric: tabular-nums;
		letter-spacing: 0.02em;
		color: rgb(255 255 255 / 0.55);
	}
</style>
