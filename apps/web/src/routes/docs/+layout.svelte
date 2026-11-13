<script lang="ts">
	import './docs.css';
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import Menu from '@lucide/svelte/icons/menu';
	import X from '@lucide/svelte/icons/x';
	import { afterNavigate } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { sections } from '#lib/docs/nav.ts';
	import type { LayoutProps } from './$types';

	let { children }: LayoutProps = $props();

	const slug = $derived(page.params.slug ?? '');
	const section = $derived(sections.find((s) => s.pages.some((p) => p.slug === slug)));
	const current = $derived(section?.pages.find((p) => p.slug === slug));

	// Close the small-screen menu when the user follows an link in it.
	let menuOpen = $state(false);
	afterNavigate(() => (menuOpen = false));
</script>

{#snippet nav()}
	{#each sections as { title, pages } (title)}
		<p class="mt-8 mb-2 text-xs font-semibold tracking-wider text-foreground uppercase first:mt-0">
			{title}
		</p>
		<ul class="space-y-0.5 border-l">
			{#each pages as p (p.slug)}
				<li>
					<a
						href={resolve('/docs/[...slug]', { slug: p.slug })}
						aria-current={p.slug === slug ? 'page' : undefined}
						class="-ml-px block border-l border-transparent py-1.5 pl-4 text-sm text-muted-foreground transition hover:border-muted-foreground hover:text-foreground aria-[current=page]:border-primary aria-[current=page]:font-medium aria-[current=page]:text-primary"
					>
						{p.title}
					</a>
				</li>
			{/each}
		</ul>
	{/each}
{/snippet}

<!-- On small screens, the sidebar becomes a menu under the site header. -->
<div class="sticky top-16 z-40 border-y bg-background/80 backdrop-blur-xl lg:hidden">
	<button
		type="button"
		class="flex h-12 w-full items-center gap-2 px-4 text-sm sm:px-6"
		aria-expanded={menuOpen}
		aria-controls="docs-menu"
		onclick={() => (menuOpen = !menuOpen)}
	>
		{#if menuOpen}<X class="size-4" />{:else}<Menu class="size-4" />{/if}
		<span class="text-muted-foreground">{section?.title ?? 'Docs'}</span>
		{#if current}
			<ChevronRight class="size-3.5 text-muted-foreground" />
			<span class="truncate font-medium">{current.title}</span>
		{/if}
	</button>
	{#if menuOpen}
		<nav id="docs-menu" class="max-h-[70dvh] overflow-y-auto border-t px-4 py-6 sm:px-6">
			{@render nav()}
		</nav>
	{/if}
</div>

<div class="mx-auto flex max-w-7xl gap-10 px-4 sm:px-6">
	<aside
		class="sticky top-16 hidden h-[calc(100dvh-4rem)] w-56 shrink-0 overflow-y-auto py-10 pr-2 lg:block"
	>
		<nav aria-label="Documentation">{@render nav()}</nav>
	</aside>
	<div class="min-w-0 flex-1">{@render children()}</div>
</div>
