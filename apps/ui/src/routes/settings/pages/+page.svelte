<script lang="ts">
	import ArrowLeftRight from '@lucide/svelte/icons/arrow-left-right';
	import ArrowUpDown from '@lucide/svelte/icons/arrow-up-down';
	import Blend from '@lucide/svelte/icons/blend';
	import ChevronLeft from '@lucide/svelte/icons/chevron-left';
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import GalleryHorizontal from '@lucide/svelte/icons/gallery-horizontal';
	import GalleryVertical from '@lucide/svelte/icons/gallery-vertical';
	import Pencil from '@lucide/svelte/icons/pencil';
	import Plus from '@lucide/svelte/icons/plus';
	import Trash from '@lucide/svelte/icons/trash';
	import { goto } from '$app/navigation';
	import { flip } from 'svelte/animate';
	import { quintOut } from 'svelte/easing';
	import { prefersReducedMotion } from 'svelte/motion';
	import type { TransitionConfig } from 'svelte/transition';
	import GridPreview from '#lib/components/settings/pages/GridPreview.svelte';
	import PageHeader from '#lib/components/settings/PageHeader.svelte';
	import PreviewItem from '#lib/components/settings/pages/PreviewItem.svelte';
	import SegmentedControl from '#lib/components/settings/controls/SegmentedControl.svelte';
	import SettingsGroup from '#lib/components/settings/SettingsGroup.svelte';
	import SettingsRow from '#lib/components/settings/SettingsRow.svelte';
	import { tabGlide } from '#lib/components/settings/tab-glide.svelte.ts';
	import { Button } from '#lib/components/ui/button/index.ts';
	import * as Tabs from '#lib/components/ui/tabs/index.ts';
	import { addPage, movePage, removePage } from '#lib/state/page-actions.ts';
	import { pageDirection, pages, pageTransition } from '#lib/state/settings.svelte.ts';
	import { shownPage } from '#lib/state/shown-page.svelte.ts';
	import { showPageOnEdge } from '#lib/tauri/events.ts';

	const DIRECTIONS = [
		{ direction: 'horizontal', label: 'Horizontal', icon: ArrowLeftRight },
		{ direction: 'vertical', label: 'Vertical', icon: ArrowUpDown }
	] as const;

	// The Slide icon follows the swipe direction of the pages.
	const TRANSITIONS = $derived([
		{
			transition: 'slide',
			label: 'Slide',
			icon: pageDirection.current === 'vertical' ? GalleryVertical : GalleryHorizontal
		},
		{ transition: 'fade', label: 'Fade', icon: Blend }
	] as const);

	const addAndEdit = () => goto(`/settings/pages/${addPage()}`);

	// The open tab is the page that the user selected last, or the first page.
	const shown = $derived(pages.current.find((p) => p.id === shownPage.id) ?? pages.current[0]);
	const index = $derived(pages.current.indexOf(shown));

	/**
	 * The time for the preview to turn to another page, as on the Edge, and the space between two
	 * pages when they pass.
	 */
	const TURN_MS = 250;
	const TURN_GAP_PX = 24;

	/** The side of the open page: 1 if it is after the page that was open before, -1 if before. */
	let turn = $state(1);

	/** Opens the tab of page `id`. The Edge turns to that page too, so the user sees it there. */
	function open(id: string) {
		turn = Math.sign(pages.current.findIndex((p) => p.id === id) - index) || 1;
		shownPage.id = id;
		showPageOnEdge(id).catch(console.error);
	}

	/** Deletes the page `id` and opens the tab that takes its place. */
	function remove(id: string) {
		const i = pages.current.findIndex((p) => p.id === id);
		removePage(id);
		// The next page takes its place. After the last page, the page before it takes its place.
		turn = i < pages.current.length ? 1 : -1;
		shownPage.id = pages.current[Math.min(i, pages.current.length - 1)]?.id;
	}

	/**
	 * Turns the preview to another page as the Edge does (see its page transition). In slide mode,
	 * the open page slides in along the swipe axis from `side` (1 is after, -1 is before). The old
	 * page slides out the other way. A page that still slides in leaves from its curent position.
	 * In fade mode, the old page fades out in place while the open page fades in.
	 */
	function turnPage(
		node: HTMLElement,
		{ side }: { side: number },
		{ direction }: { direction: 'in' | 'out' | 'both' }
	): TransitionConfig {
		const duration = prefersReducedMotion.current ? 0 : TURN_MS;
		if (pageTransition.current === 'fade') {
			return { duration, easing: quintOut, css: (t) => `opacity: ${t}` };
		}
		const vertical = pageDirection.current === 'vertical';
		const cell = node.parentElement!.getBoundingClientRect();
		const box = node.getBoundingClientRect();
		const from = direction === 'in' ? 0 : vertical ? box.top - cell.top : box.left - cell.left;
		const to = side * ((vertical ? cell.height : cell.width) + TURN_GAP_PX);
		return {
			duration,
			easing: quintOut,
			css: (_, u) => {
				const at = from + u * (to - from);
				return `translate: ${vertical ? `0 ${at}px` : `${at}px 0`}`;
			}
		};
	}

	const glide = tabGlide(() => shown?.id);
</script>

<PageHeader
	title="Pages"
	description="Swipe with two fingers between pages on the Edge. Each page has its own grid."
/>

{#if shown}
	<Tabs.Root bind:value={() => shown.id, open}>
		<!-- One row with the tabs in Edge order, then the actions of the open page. The tabs wrap if
		     they do not fit. -->
		<div class="flex flex-wrap items-center gap-2">
			<Tabs.List class="max-w-full flex-wrap justify-start">
				{#each pages.current as page (page.id)}
					<div
						class="relative flex min-w-0"
						animate:flip={{ duration: prefersReducedMotion.current ? 0 : 350, easing: quintOut }}
					>
						<!-- The look of the open tab, in its own element so that it can glide between tabs. -->
						{#if page.id === shown.id}
							<span
								{@attach glide}
								class="pointer-events-none absolute top-0 left-0 size-full rounded-md border border-transparent bg-background shadow-sm dark:border-input dark:bg-input/30"
							></span>
						{/if}
						<Tabs.Trigger
							value={page.id}
							title={page.name}
							class="relative max-w-48 min-w-0 data-[state=active]:bg-transparent data-[state=active]:shadow-none dark:data-[state=active]:border-transparent dark:data-[state=active]:bg-transparent"
						>
							<span class="truncate">{page.name}</span>
						</Tabs.Trigger>
					</div>
				{/each}
			</Tabs.List>
			<Button
				size="icon-sm"
				variant="ghost"
				aria-label="Add page"
				title="Add page"
				onclick={addAndEdit}
			>
				<Plus />
			</Button>
			<div class="ml-auto flex gap-1">
				<Button
					size="icon-sm"
					variant="ghost"
					aria-label="Move left"
					title="Move left"
					disabled={index === 0}
					onclick={() => movePage(shown.id, -1)}
				>
					<ChevronLeft />
				</Button>
				<Button
					size="icon-sm"
					variant="ghost"
					aria-label="Move right"
					title="Move right"
					disabled={index === pages.current.length - 1}
					onclick={() => movePage(shown.id, 1)}
				>
					<ChevronRight />
				</Button>
				<Button
					size="icon-sm"
					variant="ghost"
					aria-label="Edit"
					title="Edit"
					href="/settings/pages/{shown.id}"
				>
					<Pencil />
				</Button>
				<Button
					size="icon-sm"
					variant="ghost"
					aria-label="Delete"
					title="Delete"
					disabled={pages.current.length === 1}
					onclick={() => remove(shown.id)}
				>
					<Trash />
				</Button>
			</div>
		</div>
		<Tabs.Content value={shown.id}>
			<!-- Pages pass each other in one cell, which clips them but not the focus ring of the
			     preview. The key is the page, so its widgets come and go with it. They only scale in
			     or out when they are added to or removed from the open page. -->
			<div class="grid overflow-clip [overflow-clip-margin:0.25rem]">
				{#key shown.id}
					<!-- The whole preview opens the editor too. -->
					<a
						href="/settings/pages/{shown.id}"
						aria-label="Edit {shown.name}"
						in:turnPage={{ side: turn }}
						out:turnPage={{ side: -turn }}
						class="col-start-1 row-start-1 block rounded-lg outline-none [&>[data-page]]:transition-[border-color,box-shadow] hover:[&>[data-page]]:border-ring focus-visible:[&>[data-page]]:border-ring focus-visible:[&>[data-page]]:ring-3 focus-visible:[&>[data-page]]:ring-ring/50"
					>
						<GridPreview page={shown}>
							{#each shown.items as item (item.id)}
								<PreviewItem widget={item.widget} rect={item} />
							{/each}
						</GridPreview>
					</a>
				{/key}
			</div>
		</Tabs.Content>
	</Tabs.Root>
{/if}

<!-- These two settings controll how the Edge turns pages. The preview above turns the same way. -->
<div class="mt-8">
	<SettingsGroup title="Turning pages">
		<SettingsRow label="Swipe direction" description="Which way you swipe on the Edge.">
			<SegmentedControl
				name="Swipe direction"
				items={DIRECTIONS}
				selected={(d) => d.direction === pageDirection.current}
				onpick={(d) => (pageDirection.current = d.direction)}
			>
				{#snippet label(d)}<d.icon />{d.label}{/snippet}
			</SegmentedControl>
		</SettingsRow>
		<SettingsRow label="Transition" description="How one page gives way to the next.">
			<SegmentedControl
				name="Page transition"
				items={TRANSITIONS}
				selected={(t) => t.transition === pageTransition.current}
				onpick={(t) => (pageTransition.current = t.transition)}
			>
				{#snippet label(t)}<t.icon />{t.label}{/snippet}
			</SegmentedControl>
		</SettingsRow>
	</SettingsGroup>
</div>
