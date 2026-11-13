<script lang="ts" module>
	/** The gap between and around the keys, as a fraction of the key side. */
	const GAP = 0.12;
	/** Icon sizes to ask the host for. With few sizes, a resize seldom loads the icons again. */
	const ICON_SIZES = [32, 48, 64, 96, 128, 192, 256];

	const iconSize = (px: number) => ICON_SIZES.find((size) => size >= px) ?? 256;

	/**
	 * Returns true if `event` came from a finger or a pen. Then the picker does not focus its
	 * search, because the focus opens the touch keybaord over the screen.
	 */
	const touched = (event: Event) =>
		(event instanceof PointerEvent && ['touch', 'pen'].includes(event.pointerType)) ||
		!!(event as UIEvent & { sourceCapabilities?: { firesTouchEvents: boolean } }).sourceCapabilities
			?.firesTouchEvents;
</script>

<script lang="ts">
	import type { AppInfo, WidgetApi } from '@edgestation/sdk';
	import AppIcon from './AppIcon.svelte';
	import Deck from './Deck.svelte';
	import Picker from './Picker.svelte';
	import { setKey, type Keys } from './keys.ts';
	import { fold } from './search.ts';

	let props: { api: WidgetApi } = $props();
	// `api` does not change. A changed setting mounts the widget again.
	// svelte-ignore state_referenced_locally
	const { api } = props;

	const rows = Number(api.settings.rows) || 2;
	const labels = api.settings.labels === 'show';
	const editable = api.settings.editing === 'on';
	// The icon sizes are in screen pixels, not CSS pixels.
	const dpr = window.devicePixelRatio || 1;
	/** The icon size in the picker rows (about 1.6em, with a font of 18px or less). */
	const listIcon = iconSize(30 * dpr);

	/**
	 * `undefined` until the widget reads the saved keys. Keys that do not fit stay saved, for when
	 * the cell gets bigger again.
	 */
	let keys = $state.raw<Keys>();
	let width = $state(0);
	let height = $state(0);

	/**
	 * Square keys in `rows` rows, as tall as the cell permits. The number of columns is rounded to
	 * the nearest fit. So the keys can get a little smaller to fill the width, with no wide
	 * margins.
	 */
	const deck = $derived.by(() => {
		if (!(width > 0 && height > 0)) return { side: 0, gap: 0, cols: 0, count: 0 };
		const tallest = height / (rows + (rows + 1) * GAP);
		const cols = Math.max(1, Math.round((width / tallest - GAP) / (1 + GAP)));
		const side = Math.min(tallest, width / (cols + (cols + 1) * GAP));
		return { side, gap: side * GAP, cols, count: rows * cols };
	});
	const slots = $derived(Array.from({ length: deck.count }, (_, i) => keys?.[i] ?? null));
	const keyIcon = $derived(iconSize(deck.side * (labels ? 0.5 : 0.6) * dpr));

	/** Apps with a icon that did not load (removed since). They show their initial letter. */
	let broken = $state<Record<string, boolean>>({});

	api.guard(async () => {
		keys = (await api.storage.instance.get<Keys>('keys')) ?? [];
	})();

	const save = api.guard(async (next: Keys) => {
		keys = next;
		await api.storage.instance.set('keys', next);
	});

	/** The key that gets a new app. `undefined` when the keys show. */
	let editing = $state<number>();
	/** All apps, with the names folded for search. `undefined` until the first list. */
	let apps = $state.raw<{ app: AppInfo; folded: string }[]>();
	/** True if a mouse or a keyboard opened the picker. Then its search gets the focus. */
	let typing = $state(false);
	const current = $derived(editing === undefined ? undefined : keys?.[editing]);

	// List the apps again each time the picker opens, for new apps. The old list shows untill then.
	const listApps = api.guard(async () => {
		const list = await api.apps.list();
		apps = list.map((app) => ({ app, folded: fold(app.name) }));
	});

	function edit(index: number, event: Event) {
		if (!editable || editing === index) return;
		editing = index;
		typing = !touched(event);
		listApps();
	}

	function close() {
		editing = undefined;
	}

	/** Puts `app` on the key in edit, or makes the key empty for `null`. */
	function pick(app: AppInfo | null) {
		if (keys && editing !== undefined) save(setKey(keys, editing, app));
		close();
	}
</script>

{#snippet appIcon(app: AppInfo, size: number)}
	<AppIcon {api} {app} {size} broken={broken[app.id]} onerror={() => (broken[app.id] = true)} />
{/snippet}

<!-- `el` is the size container, so its padding cannot use cqh or cqw. Put the layout in it.
     There is no context menu, because a long touch or a right click on a key edits the key. -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="launcher"
	bind:clientWidth={width}
	bind:clientHeight={height}
	oncontextmenu={(event) => event.preventDefault()}
>
	{#if editing !== undefined}
		<Picker {api} {apps} {current} {typing} {listIcon} {appIcon} onpick={pick} onclose={close} />
	{:else if keys}
		<Deck {api} {deck} {rows} {slots} {labels} {editable} {keyIcon} {appIcon} onedit={edit} />
	{/if}
</div>

<style>
	.launcher {
		position: relative;
		height: 100%;
		-webkit-user-select: none;
		user-select: none;
	}

	/* The child components use the rules that follow. `:where()` keeps their specificity low, so
	   a rule in a child component always overrides them. */
	:where(.launcher) :global(button) {
		padding: 0;
		border: none;
		background: none;
		color: inherit;
		font: inherit;
		cursor: pointer;
		-webkit-tap-highlight-color: transparent;
	}
</style>
