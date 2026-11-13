<script lang="ts">
	import type { MediaAction, MediaInfo, WidgetApi } from '@edgestation/sdk';
	import Controls from './Controls.svelte';
	import Cover from './Cover.svelte';
	import Progress from './Progress.svelte';

	let props: { api: WidgetApi } = $props();
	// `api` does not change. A changed setting mounts the widget again.
	// svelte-ignore state_referenced_locally
	const { api } = props;

	/** The current session. `undefined` until the first read, `null` if nothing plays. */
	let now = $state.raw<MediaInfo | null>();
	let art = $state.raw<string | null>(null);
	let shownArt: number | null = null;
	const playing = $derived(!!now?.playing);

	const refresh = api.guard(async () => {
		now = await api.media.current();
		// The art is large, so get it only when it changes.
		const artId = now?.art ?? null;
		if (artId !== shownArt) {
			shownArt = artId;
			art = artId == null ? null : await api.media.art();
		}
	});
	api.every(1000, refresh);
	refresh();

	// Apps need a short time to aply a command. Look again soon, not at the next tick.
	let cancelSoon: (() => void) | undefined;
	const send = api.guard(async (action: MediaAction) => {
		await api.media.control(action);
		cancelSoon?.();
		cancelSoon = api.after(300, refresh);
	});
</script>

<!-- `el` is the size container, so its padding cannot use cqh or cqw. Put the layout in it. -->
{#if now !== undefined}
	<div class="body" class:idle={!now}>
		<Cover {art} />
		<div class="info">
			<p class="title">{now ? now.title || 'Unknown title' : 'Nothing playing'}</p>
			{#if now?.artist}<p class="artist">{now.artist}</p>{/if}
			{#if now?.duration != null}
				<Progress position={now.position ?? 0} duration={now.duration} />
			{/if}
			<Controls {now} {playing} {send} />
		</div>
	</div>
{/if}

<style>
	/* cqh and cqw are relative to the cell of the widget, so all parts grow with the cell. */
	.body {
		display: flex;
		align-items: center;
		gap: min(7cqh, 5cqw);
		height: 100%;
		padding: min(8cqh, 8cqw);
	}

	.info {
		display: flex;
		flex: 1;
		flex-direction: column;
		gap: min(3cqh, 2cqw);
		min-width: 0;
	}

	/* The child components use this rule too. `:where()` keeps its specificity low, so a rule in
	   a child component always overrides it. */
	:where(.body) :global(p) {
		margin: 0;
	}

	.title {
		display: -webkit-box;
		overflow: hidden;
		font-size: min(10cqh, 5.5cqw);
		font-weight: 600;
		line-height: 1.2;
		overflow-wrap: anywhere;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
	}

	.artist {
		overflow: hidden;
		color: var(--muted-foreground);
		font-size: min(8cqh, 4.5cqw);
		white-space: nowrap;
		text-overflow: ellipsis;
	}

	.idle .title {
		color: var(--muted-foreground);
	}

	/* In square and tall cells, the art is above the text, not next to it. */
	@container (aspect-ratio < 1.4) {
		.body {
			flex-direction: column;
			align-items: stretch;
			gap: min(5cqh, 5cqw);
		}

		.info {
			flex: none;
		}

		.title {
			font-size: min(6.5cqh, 8cqw);
		}

		.artist {
			font-size: min(5.5cqh, 6.5cqw);
		}
	}
</style>
