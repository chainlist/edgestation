<script lang="ts">
	import type { WidgetApi } from '@edgestation/sdk';
	import Glyph from './Glyph.svelte';
	import type { Entry } from './entries.ts';

	let {
		api,
		entries,
		selected,
		editing,
		latest,
		timeFormat,
		scroller = $bindable(),
		ontoggle,
		onedit,
		onremove
	}: {
		api: WidgetApi;
		entries: Entry[];
		selected?: string;
		editing?: string;
		latest?: string;
		timeFormat: Intl.DateTimeFormat;
		scroller?: HTMLElement;
		ontoggle: (id: string) => void;
		onedit: (entry: Entry) => void;
		onremove: (id: string) => void;
	} = $props();

	/** Scrolls the timeline only as much as necessary to show all of `row`. */
	function reveal(row: Element) {
		if (!scroller) return;
		const view = scroller.getBoundingClientRect();
		const box = row.getBoundingClientRect();
		if (box.bottom > view.bottom) scroller.scrollTop += box.bottom - view.bottom;
		if (box.top < view.top) scroller.scrollTop -= view.top - box.top;
	}
</script>

<!-- This holds the overlay scrollbars of the timeline. -->
<div class="list">
	<div class="scroller" bind:this={scroller} {@attach (el) => api.scrollbars(el)}>
		<ol>
			{#each entries as entry (entry.id)}
				<li
					class:selected={selected === entry.id}
					class:editing={editing === entry.id}
					class:now={entry.id === latest}
				>
					<button
						class="entry"
						aria-expanded={selected === entry.id}
						onclick={() => ontoggle(entry.id)}
					>
						<time datetime={new Date(entry.at).toISOString()}>
							{timeFormat.format(entry.at)}
						</time>
						<span class="rail" aria-hidden="true"></span>
						<span class="text">{entry.text}</span>
					</button>
					{#if selected === entry.id}
						<span class="rail" aria-hidden="true"></span>
						<div class="actions" {@attach (actions) => reveal(actions.parentElement!)}>
							<button onclick={() => onedit(entry)}><Glyph name="edit" />Edit</button>
							<button class="remove" onclick={() => onremove(entry.id)}>
								<Glyph name="remove" />Delete
							</button>
						</div>
					{/if}
				</li>
			{/each}
		</ol>
	</div>
</div>

<style>
	.list {
		flex: 1;
		min-height: 0;
	}

	/* The scroller scrolls inside `.list` and fills it (see `api.scrollbars`). */
	.scroller {
		overscroll-behavior: contain;
	}

	/* Columns for the time, the rail and the text. Each entry uses them (subgrid), so the rail
	   stays straigth for all widths of the times. */
	ol {
		/* The middle of the first line of an entry, where its dot is. It is the top padding of
		   the text plus half of its line height. */
		--dot-y: calc(0.4em + 1.3em / 2);
		display: grid;
		grid-template-columns: auto 1.4em minmax(0, 1fr);
		margin: 0;
		padding: 0;
		list-style: none;
	}

	li {
		display: grid;
		grid-column: 1 / -1;
		grid-template-columns: subgrid;
		border-radius: calc(var(--radius) * 0.6);
	}

	li.selected,
	li.editing {
		background: var(--muted);
	}

	/* No vertical padding. The rail fills the height of the row, so it joins the rail of the
	   next entry. */
	.entry {
		display: grid;
		grid-column: 1 / -1;
		grid-template-columns: subgrid;
		align-items: baseline;
		text-align: left;
		-webkit-user-select: none;
		user-select: none;
	}

	time {
		padding: 0.5em 0 0.5em 0.6em;
		font-size: 0.8em;
		font-variant-numeric: tabular-nums;
		color: var(--muted-foreground);
		text-align: right;
		white-space: nowrap;
	}

	.text {
		padding: 0.4em 0.5em 0.4em 0;
		line-height: 1.3;
		overflow-wrap: anywhere;
	}

	.rail {
		position: relative;
		align-self: stretch;
	}

	/* The line, from the entry above to the entry below. */
	.rail::before {
		position: absolute;
		top: 0;
		bottom: 0;
		left: 50%;
		width: 2px;
		translate: -50% 0;
		background: var(--border);
		content: '';
	}

	/* The dot of the entry, next to the frist line of its text. */
	.entry .rail::after {
		position: absolute;
		top: var(--dot-y);
		left: 50%;
		width: 0.6em;
		height: 0.6em;
		translate: -50% -50%;
		border-radius: 50%;
		background: var(--primary);
		content: '';
	}

	/* The current activity: the latest entry of today. */
	.now .entry .rail::after {
		box-shadow: 0 0 0 0.22em color-mix(in oklab, var(--primary) 35%, transparent);
	}

	/* The line starts at the first dot and stops at the last one. */
	li:first-child .entry .rail::before {
		top: var(--dot-y);
	}

	li:last-child .entry .rail::before {
		bottom: auto;
		height: var(--dot-y);
	}

	li:only-child .entry .rail::before,
	li:last-child > .rail::before {
		display: none;
	}

	/* Below the text of the selected entry, next to its rail. */
	li > .rail {
		grid-row: 2;
		grid-column: 2;
	}

	.actions {
		display: flex;
		flex-wrap: wrap;
		grid-row: 2;
		grid-column: 3;
		gap: 0.4em;
		padding: 0 0.5em 0.5em 0;
	}

	.actions button {
		display: flex;
		align-items: center;
		gap: 0.35em;
		padding: 0.3em 0.8em;
		border-radius: 999px;
		background: var(--secondary);
		color: var(--secondary-foreground);
		font-size: 0.8em;
		/* An outline, because the background of the selected row can have the same color. */
		box-shadow: inset 0 0 0 1px var(--border);
	}

	.actions :global(svg) {
		width: 1.1em;
	}

	.actions .remove {
		color: var(--destructive);
	}
</style>
