<script lang="ts">
	import type { WidgetApi } from '@edgestation/sdk';
	import { tick } from 'svelte';
	import DaySelector from './DaySelector.svelte';
	import EntryForm from './EntryForm.svelte';
	import Timeline from './Timeline.svelte';
	import { dateFormat, dateOf, dayBefore, dayName, dayOf, hourOptions } from './dates.ts';
	import { DAYS, dayKey, type Entry } from './entries.ts';

	let props: { api: WidgetApi } = $props();
	// `api` does not change. A changed setting mounts the widget again.
	// svelte-ignore state_referenced_locally
	const { api } = props;
	// All copies of the widget share this storage. So there is one log on all pages, and the log
	// stays when the user removes the widget and adds it again.
	const store = api.storage.widget;
	const timeFormat = new Intl.DateTimeFormat(undefined, {
		...hourOptions[api.settings.hours],
		minute: '2-digit'
	});

	const start = dayOf(new Date());
	let today = $state(start);
	let day = $state(start);
	/** Days with entries, in date order. */
	let days = $state.raw<string[]>([]);
	/** The entries of the shown day, oldest first. `undefined` until the first read. */
	let entries = $state.raw<Entry[]>();
	/** The element of the timeline that scrolls, while there are entries. */
	let scroller = $state<HTMLElement>();
	/** The last entry that the user tapped. It shows its Edit and Delete buttons. */
	let selected = $state<string>();
	/** The entry that the field rewrites. `undefined` when the field logs new entries. */
	let editing = $state<{ day: string; id: string }>();
	let draft = $state('');
	let field = $state<HTMLInputElement>();

	const isToday = $derived(day === today);
	const name = $derived(dayName(day, today));
	/** The date next to "Today" and "Yesterday", because these words do not give the date. */
	const date = $derived(
		isToday || day === dayBefore(today) ? dateFormat.format(dateOf(day)) : undefined
	);
	// The arrows skip the days without entries. Today is the only empty day that they can show.
	const earlier = $derived(days.findLast((d) => d < day));
	const later = $derived(
		day < today ? (days.find((d) => d > day && d < today) ?? today) : undefined
	);
	/** The latest entry of today, which is the current activity. */
	const latest = $derived(isToday ? entries?.at(-1)?.id : undefined);

	async function scrollTo(end: 'first' | 'last') {
		await tick();
		if (scroller) scroller.scrollTop = end === 'last' ? scroller.scrollHeight : 0;
	}

	/** Reads the shown day and the list of days again, because another copy can change them. */
	async function refresh() {
		const shown = day;
		const [saved, list] = await Promise.all([
			store.get<Entry[]>(dayKey(shown)),
			store.get<string[]>(DAYS)
		]);
		days = list ?? [];
		if (day === shown) entries = saved ?? [];
	}

	api.guard(async () => {
		await refresh();
		await scrollTo('last');
	})();

	// Follow the clock after midnight, and get the entries that other copies loged.
	api.every(60_000, async () => {
		const now = dayOf(new Date());
		if (now !== today) {
			if (day === today) {
				day = now;
				selected = undefined;
			}
			today = now;
		}
		await refresh();
	});

	/**
	 * Shows `target`. Today scrolls to its latest entries, next to the field. Other days show
	 * from their start.
	 */
	const show = api.guard(async (target: string) => {
		day = target;
		selected = undefined;
		const saved = (await store.get<Entry[]>(dayKey(target))) ?? [];
		if (day !== target) return;
		entries = saved;
		await scrollTo(target === today ? 'last' : 'first');
	});

	/**
	 * Rewrites the entries of `target` with `edit`. It starts from the saved entries, not the
	 * shown ones, because another copy of the widget can have newer entires. Shows the result
	 * immediately.
	 */
	async function change(target: string, edit: (entries: Entry[]) => Entry[]) {
		const [saved, list = []] = await Promise.all([
			store.get<Entry[]>(dayKey(target)),
			store.get<string[]>(DAYS)
		]);
		const next = edit(saved ?? []);
		const listed = list.includes(target);
		const relist = listed !== next.length > 0;
		days = !relist ? list : listed ? list.filter((d) => d !== target) : [...list, target].sort();
		if (day === target) entries = next;
		await Promise.all([
			next.length ? store.set(dayKey(target), next) : store.delete(dayKey(target)),
			relist && store.set(DAYS, days)
		]);
	}

	/** Adds `text` to the log of today at the current time, and shows it. */
	const log = api.guard(async (text: string) => {
		const now = new Date();
		const target = dayOf(now);
		today = target;
		if (day !== target) {
			day = target;
			selected = undefined;
		}
		await change(target, (saved) =>
			[...saved, { id: crypto.randomUUID(), at: now.getTime(), text }].sort((a, b) => a.at - b.at)
		);
		await scrollTo('last');
	});

	const rewrite = api.guard((target: string, id: string, text: string) =>
		change(target, (saved) => saved.map((entry) => (entry.id === id ? { ...entry, text } : entry)))
	);

	const remove = api.guard((id: string) => {
		if (editing?.id === id) stopEditing();
		selected = undefined;
		return change(day, (saved) => saved.filter((entry) => entry.id !== id));
	});

	const toggle = (id: string) => (selected = selected === id ? undefined : id);

	/** Puts the text of `entry` in the field, so that the field rewrites that entry. */
	function edit(entry: Entry) {
		editing = { day, id: entry.id };
		selected = undefined;
		draft = entry.text;
		field?.focus();
	}

	function stopEditing() {
		editing = undefined;
		draft = '';
	}

	function submit(event: SubmitEvent) {
		event.preventDefault();
		const text = draft.trim();
		if (!text) return;
		if (editing) rewrite(editing.day, editing.id, text);
		else log(text);
		stopEditing();
	}
</script>

<!-- `el` is the size container, so its padding cannot use cqh or cqw. Put the layout in it. -->
{#if entries}
	<div class="logs">
		<DaySelector {name} {date} {isToday} {earlier} {later} {today} onshow={show} />

		{#if entries.length}
			<Timeline
				{api}
				{entries}
				{selected}
				editing={editing?.id}
				{latest}
				{timeFormat}
				bind:scroller
				ontoggle={toggle}
				onedit={edit}
				onremove={remove}
			/>
		{:else}
			<p class="empty">{isToday ? 'Nothing logged yet' : 'Nothing logged'}</p>
		{/if}

		<EntryForm editing={!!editing} bind:draft bind:field onsubmit={submit} oncancel={stopEditing} />
	</div>
{/if}

<style>
	/* As in a list, a bigger cell shows more entries, not bigger text. The text size follows only
	   the widht of the cell, with limits that keep it readable. */
	.logs {
		display: flex;
		flex-direction: column;
		gap: 0.5em;
		height: 100%;
		padding: 0.75em;
		font-size: clamp(13px, 5cqw, 22px);
	}

	.empty {
		display: grid;
		flex: 1;
		place-items: center;
		margin: 0;
		color: var(--muted-foreground);
	}

	/* The child components use the rules that follow. `:where()` keeps their specificity low, so
	   a rule in a child component always overrides them. */
	:where(.logs) :global(button) {
		padding: 0;
		border: none;
		background: none;
		color: inherit;
		font: inherit;
		cursor: pointer;
		-webkit-tap-highlight-color: transparent;
	}

	:where(.logs) :global(button:disabled) {
		cursor: default;
	}
</style>
