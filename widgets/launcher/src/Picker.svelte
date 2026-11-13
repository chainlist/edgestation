<script lang="ts">
	import type { AppInfo, WidgetApi } from '@edgestation/sdk';
	import type { Snippet } from 'svelte';
	import Glyph from './Glyph.svelte';
	import { fold } from './search.ts';

	let {
		api,
		apps,
		current,
		typing,
		listIcon,
		appIcon,
		onpick,
		onclose
	}: {
		api: WidgetApi;
		apps?: { app: AppInfo; folded: string }[];
		current?: AppInfo | null;
		typing: boolean;
		listIcon: number;
		appIcon: Snippet<[AppInfo, number]>;
		onpick: (app: AppInfo | null) => void;
		onclose: () => void;
	} = $props();

	let query = $state('');
	const found = $derived.by(() => {
		const words = fold(query).split(/\s+/).filter(Boolean);
		return apps
			?.filter(({ folded }) => words.every((word) => folded.includes(word)))
			.map(({ app }) => app);
	});

	// Enter picks the first match.
	function submit(event: SubmitEvent) {
		event.preventDefault();
		if (found?.length) onpick(found[0]);
	}
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
	class="picker"
	role="dialog"
	aria-label="Pick an app"
	tabindex="-1"
	onkeydown={(event) => event.key === 'Escape' && onclose()}
>
	<form onsubmit={submit}>
		<button type="button" class="round" aria-label="Back" onclick={onclose}>
			<Glyph name="back" />
		</button>
		<input
			type="search"
			bind:value={query}
			placeholder="Search apps"
			aria-label="Search apps"
			enterkeyhint="go"
			{@attach (input) => {
				if (typing) input.focus({ preventScroll: true });
			}}
		/>
		{#if current}
			<button
				type="button"
				class="round remove"
				aria-label="Empty this key"
				title="Empty this key"
				onclick={() => onpick(null)}
			>
				<Glyph name="remove" />
			</button>
		{/if}
	</form>

	{#if !found}
		<p class="note">Loading apps</p>
	{:else if !found.length}
		<p class="note">No app matches</p>
	{:else}
		<!-- This holds the overlay scrollbars of the list. -->
		<div class="list">
			<ul {@attach (ul) => api.scrollbars(ul)}>
				{#each found as app (app.id)}
					<li>
						<button class="app" class:current={app.id === current?.id} onclick={() => onpick(app)}>
							{@render appIcon(app, listIcon)}
							<span class="name">{app.name}</span>
						</button>
					</li>
				{/each}
			</ul>
		</div>
	{/if}
</div>

<style>
	/* The picker shows in place of the keys: a search feild above the list of apps. */
	.picker {
		display: flex;
		flex-direction: column;
		gap: 0.5em;
		height: 100%;
		padding: 0.6em;
		font-size: clamp(13px, 4cqw, 18px);
		outline: none;
	}

	form {
		display: flex;
		gap: 0.5em;
	}

	.round {
		display: grid;
		flex: none;
		place-items: center;
		width: 2.2em;
		height: 2.2em;
		border-radius: 50%;
		background: var(--secondary);
		color: var(--secondary-foreground);
	}

	.round :global(svg) {
		width: 55%;
	}

	.remove {
		color: var(--destructive);
	}

	input {
		flex: 1;
		min-width: 0;
		height: 2.2em;
		padding: 0 0.9em;
		border: 1px solid var(--border);
		border-radius: 999px;
		background: transparent;
		color: inherit;
		font: inherit;
		-webkit-user-select: text;
		user-select: text;
	}

	input::placeholder {
		color: var(--muted-foreground);
	}

	input:focus {
		border-color: var(--ring);
		outline: none;
	}

	.list {
		flex: 1;
		min-height: 0;
	}

	/* The list scrolls inside `.list` and fills it (see `api.scrollbars`). */
	ul {
		margin: 0;
		padding: 0;
		overscroll-behavior: contain;
		list-style: none;
	}

	.app {
		--icon-width: 1.6em;
		--initial-size: 0.9em;
		display: flex;
		align-items: center;
		gap: 0.7em;
		width: 100%;
		min-height: 2.4em;
		padding: 0.25em 0.4em;
		border-radius: calc(var(--radius) * 0.6);
		text-align: left;
	}

	.app:active {
		background: var(--muted);
	}

	.name {
		min-width: 0;
		overflow: hidden;
		white-space: nowrap;
		text-overflow: ellipsis;
	}

	.current {
		color: var(--primary);
		font-weight: 600;
	}

	.note {
		display: grid;
		flex: 1;
		place-items: center;
		margin: 0;
		color: var(--muted-foreground);
	}
</style>
