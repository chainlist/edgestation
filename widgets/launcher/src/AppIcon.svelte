<script lang="ts" module>
	/** The first letter of `name`. It shows if the icon cannot load. */
	const initial = (name: string) => [...name.trim()][0]?.toUpperCase() ?? '?';
</script>

<script lang="ts">
	import type { AppInfo, WidgetApi } from '@edgestation/sdk';

	let {
		api,
		app,
		size,
		broken,
		onerror
	}: {
		api: WidgetApi;
		app: AppInfo;
		size: number;
		broken: boolean;
		onerror: () => void;
	} = $props();
</script>

<span class="icon">
	{#if broken}
		<span class="initial" aria-hidden="true">{initial(app.name)}</span>
	{:else}
		<img alt="" src={api.apps.icon(app.id, size)} draggable="false" loading="lazy" {onerror} />
	{/if}
</span>

<style>
	.icon {
		display: grid;
		flex: none;
		place-items: center;
		width: var(--icon-width);
		aspect-ratio: 1;
	}

	.icon img {
		width: 100%;
		height: 100%;
		object-fit: contain;
		-webkit-user-drag: none;
	}

	.initial {
		display: grid;
		place-items: center;
		width: 100%;
		height: 100%;
		border-radius: 22%;
		background: var(--muted);
		color: var(--muted-foreground);
		font-size: var(--initial-size);
		font-weight: 600;
	}
</style>
