<script lang="ts">
	import type { MediaAction, MediaInfo, WidgetApi } from '@edgestation/sdk';
	import Controls from './Controls.svelte';
	import Cover from './Cover.svelte';
	import Progress from './Progress.svelte';

	let props: { api: WidgetApi } = $props();
	// `api` does not change. A changed setting mounts the widget again.
	// svelte-ignore state_referenced_locally
	const { api } = props;

	/** All sessions. `undefined` until the first read. */
	let sessions = $state.raw<MediaInfo[]>();
	/**
	 * The session that the user stepped to. While it is set, the widget shows it, not the current
	 * session. It clears when the session becomes current or closes.
	 */
	let chosen = $state<string | null>(null);
	/** The shown session. `undefined` until the first read, `null` if nothing plays. */
	const now = $derived(
		sessions &&
			(sessions.find((session) => session.id === chosen) ??
				sessions.find((session) => session.current) ??
				sessions[0] ??
				null)
	);
	const playing = $derived(!!now?.playing);
	const index = $derived(sessions && now ? sessions.indexOf(now) : 0);
	let art = $state.raw<string | null>(null);
	let shownArt: number | null = null;

	// The art is large, so get it only when it changes.
	async function loadArt() {
		const shown = now;
		const artId = shown?.art ?? null;
		if (artId === shownArt) return;
		shownArt = artId;
		const url = shown && artId != null ? await api.media.art(shown.id) : null;
		// A step to an other session during the wait can have loaded newer art.
		if (shownArt === artId) art = url;
	}

	/** The time of the last read and the time now, in ms. The track plays on between reads. */
	let readAt = $state(0);
	let tick = $state(0);
	const position = $derived.by(() => {
		if (now?.position == null) return 0;
		const played = now.playing ? ((tick - readAt) / 1000) * now.rate : 0;
		return Math.min(now.position + played, now.duration ?? Infinity);
	});

	const refresh = api.guard(async () => {
		const read = await api.media.sessions();
		const kept = read.find((session) => session.id === chosen);
		if (!kept || kept.current) chosen = null;
		sessions = read;
		readAt = tick = performance.now();
		await loadArt();
	});
	// The host tells when something changes. The position moves on with no event, so a timer
	// moves it here, and it asks the host nothing.
	api.media.onChange(refresh);
	api.every(1000, () => (tick = performance.now()));
	refresh();

	const step = api.guard(async () => {
		if (!sessions?.length) return;
		const next = sessions[(index + 1) % sessions.length];
		chosen = next.current ? null : next.id;
		await loadArt();
	});

	// Apps need a short time to aply a command. Look again soon, so the button shows it.
	let cancelSoon: (() => void) | undefined;
	const send = api.guard(async (action: MediaAction | 'restart') => {
		if (!now) return;
		if (action === 'restart') await api.media.seek(0, now.id);
		else await api.media.control(action, now.id);
		cancelSoon?.();
		cancelSoon = api.after(300, refresh);
	});
</script>

<!-- `el` is the size container, so its padding cannot use cqh or cqw. Put the layout in it. -->
{#if now !== undefined}
	<div class="body" class:idle={!now}>
		<Cover
			{art}
			count={sessions?.length ?? 0}
			{index}
			onstep={step}
			onrestart={now?.can.seek ? () => send('restart') : undefined}
		/>
		<div class="info">
			<p class="title">{now ? now.title || 'Unknown title' : 'Nothing playing'}</p>
			{#if now?.artist}<p class="artist">{now.artist}</p>{/if}
			{#if now?.duration != null}
				<Progress {position} duration={now.duration} />
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
