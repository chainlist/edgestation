<!--
	The Go to page box on the Edge. The user types part of a page name, then selects one of the
	page that match. Its shortcut works from any app. Thus the box borrows the keyboard focus
	while it is open, then gives it back to the app that had it.
-->
<script lang="ts">
	import Search from '@lucide/svelte/icons/search';
	import { findPages } from '#lib/layout/pages.ts';
	import type { Page } from '#lib/layout/types.ts';
	import { cleanupOf } from '#lib/tauri/events.ts';
	import { borrowFocus, returnFocus } from '#lib/tauri/focus.ts';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import EdgeDialog from './EdgeDialog.svelte';

	let {
		pages,
		active,
		onselect,
		open = $bindable(false)
	}: {
		pages: Page[];
		active: number;
		onselect: (index: number) => void;
		open?: boolean;
	} = $props();

	let query = $state('');
	/** The position in `found` of the page that Enter opens. */
	let picked = $state(0);
	const found = $derived(findPages(pages, query));

	let list = $state<HTMLElement>();
	$effect(() => list?.children[picked]?.scrollIntoView({ block: 'nearest' }));

	// Close the box when the user goes to another app. Use the window focus event, not the page
	// blur, for the same reason as in `focus.ts`.
	$effect(() => {
		if (!open) return;
		return cleanupOf(
			getCurrentWindow().onFocusChanged(({ payload: focused }) => {
				if (!focused) close(false);
			})
		);
	});

	/** Opens the box, or closes it if it is open. The shortcut uses this. */
	export function toggle() {
		if (open) return close();
		query = '';
		picked = 0;
		open = true;
		borrowFocus().catch(console.error);
	}

	/** Closes the box. The app that had the focus gets it back, if the user did not go to another. */
	function close(giveBack = true) {
		open = false;
		if (giveBack) returnFocus().catch(console.error);
	}

	function go(index: number) {
		onselect(index);
		close();
	}

	function keydown(e: KeyboardEvent) {
		const count = found.length;
		if (e.key === 'Escape') close();
		else if (e.key === 'Enter' && count) go(found[picked]);
		else if (e.key === 'ArrowDown' && count) picked = (picked + 1) % count;
		else if (e.key === 'ArrowUp' && count) picked = (picked - 1 + count) % count;
		else return;
		e.preventDefault();
	}
</script>

{#if open}
	<!-- The box owns the pointer. The carousel must not start a swipe from it. A press outside
	     the box closes it, and Escape also closes it. -->
	<EdgeDialog
		backdropClass="absolute items-start"
		onbackdroppointerdown={(e) => {
			e.stopPropagation();
			if (e.target === e.currentTarget) close();
		}}
		class="flex max-h-full w-[min(44rem,100%)] flex-col overflow-hidden"
	>
		<label class="flex shrink-0 items-center gap-4 px-6">
			<Search class="size-7 shrink-0 text-muted-foreground" />
			<input
				{@attach (input) => input.focus()}
				bind:value={query}
				oninput={() => (picked = 0)}
				onkeydown={keydown}
				class="h-18 min-w-0 flex-1 bg-transparent text-3xl outline-none placeholder:text-muted-foreground"
				placeholder="Go to page"
				role="combobox"
				aria-expanded="true"
				aria-controls="page-switcher-list"
				aria-autocomplete="list"
				aria-activedescendant={found.length ? `page-switcher-${found[picked]}` : undefined}
				autocomplete="off"
				spellcheck="false"
			/>
		</label>
		{#if found.length}
			<div
				bind:this={list}
				id="page-switcher-list"
				class="min-h-0 overflow-y-auto border-t p-2"
				role="listbox"
				aria-label="Pages"
			>
				{#each found as index, i (pages[index].id)}
					<button
						id="page-switcher-{index}"
						class={[
							'flex h-14 w-full items-center gap-4 rounded-2xl px-4 text-left text-2xl',
							i === picked ? 'bg-accent text-accent-foreground' : 'hover:bg-accent/50'
						]}
						role="option"
						aria-selected={i === picked}
						tabindex="-1"
						onclick={() => go(index)}
					>
						<span class="w-8 shrink-0 text-muted-foreground tabular-nums">{index + 1}</span>
						<span class="min-w-0 flex-1 truncate">{pages[index].name}</span>
						{#if index === active}
							<span class="shrink-0 text-lg text-muted-foreground">Current</span>
						{/if}
					</button>
				{/each}
			</div>
		{:else}
			<p class="border-t px-6 py-4 text-xl text-muted-foreground">No page matches “{query}”</p>
		{/if}
	</EdgeDialog>
{/if}
