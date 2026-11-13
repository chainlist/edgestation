<script lang="ts">
	import ArrowLeft from '@lucide/svelte/icons/arrow-left';
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import SquarePen from '@lucide/svelte/icons/square-pen';
	import { resolve } from '$app/paths';
	import { pages, sections } from '#lib/docs/nav.ts';
	import { REPO_URL } from '#lib/site.ts';
	import type { PageProps } from './$types';

	let { data, params }: PageProps = $props();

	const index = $derived(pages.findIndex((p) => p.slug === params.slug));
	const doc = $derived(pages[index]);
	const previous = $derived(pages[index - 1]);
	const next = $derived(pages[index + 1]);
	const section = $derived(sections.find((s) => s.pages.includes(doc)));
	const source = $derived(
		`${REPO_URL}/blob/main/apps/web/src/lib/docs/pages/${params.slug || 'index'}.md`
	);
	// The sections of the page, and the subsections in them.
	const outline = $derived(data.headings.filter((h) => h.depth <= 3));
</script>

<svelte:head>
	<title>{doc.slug ? `${doc.title} · EdgeStation docs` : 'EdgeStation docs'}</title>
	<meta name="description" content={doc.description} />
</svelte:head>

<div class="flex gap-10">
	<article class="min-w-0 flex-1 py-10 lg:py-14">
		<header class="border-b pb-8">
			<p class="text-sm font-semibold text-primary">{section?.title}</p>
			<h1 class="mt-2 text-3xl font-bold tracking-tight sm:text-4xl">{doc.title}</h1>
			<p class="mt-3 text-lg text-pretty text-muted-foreground">{doc.description}</p>
		</header>

		<div class="doc">
			{@html data.html}
		</div>

		<footer class="mt-16 border-t pt-8">
			<a
				href={source}
				class="inline-flex items-center gap-2 text-sm text-muted-foreground transition hover:text-foreground"
			>
				<SquarePen class="size-4" />Edit this page on GitHub
			</a>
			<nav class="mt-8 grid gap-4 sm:grid-cols-2" aria-label="Previous and next pages">
				{#if previous}
					<a
						href={resolve('/docs/[...slug]', { slug: previous.slug })}
						class="group rounded-xl border p-4 transition hover:border-primary/60 hover:bg-card"
					>
						<span class="flex items-center gap-1.5 text-xs text-muted-foreground">
							<ArrowLeft class="size-3.5" />Previous
						</span>
						<span class="mt-1 block font-semibold group-hover:text-primary">{previous.title}</span>
					</a>
				{/if}
				{#if next}
					<a
						href={resolve('/docs/[...slug]', { slug: next.slug })}
						class="group rounded-xl border p-4 text-right transition hover:border-primary/60 hover:bg-card sm:col-start-2"
					>
						<span class="flex items-center justify-end gap-1.5 text-xs text-muted-foreground">
							Next<ArrowRight class="size-3.5" />
						</span>
						<span class="mt-1 block font-semibold group-hover:text-primary">{next.title}</span>
					</a>
				{/if}
			</nav>
		</footer>
	</article>

	{#if outline.length}
		<aside
			class="sticky top-16 hidden h-[calc(100dvh-4rem)] w-52 shrink-0 overflow-y-auto py-14 xl:block"
		>
			<p class="text-xs font-semibold tracking-wider uppercase">On this page</p>
			<ul class="mt-3 space-y-2 text-sm">
				{#each outline as { id, html, depth } (id)}
					<li class={depth === 3 ? 'pl-3' : ''}>
						<a href="#{id}" class="toc-link text-muted-foreground transition hover:text-foreground">
							{@html html}
						</a>
					</li>
				{/each}
			</ul>
		</aside>
	{/if}
</div>
