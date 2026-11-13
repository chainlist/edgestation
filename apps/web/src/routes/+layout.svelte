<script lang="ts">
	import './layout.css';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import BetaBadge from '#lib/components/BetaBadge.svelte';
	import GithubIcon from '#lib/components/GithubIcon.svelte';
	import { REPO_URL } from '#lib/site.ts';
	import icon from '#lib/assets/app-icon.svg';
	import type { LayoutProps } from './$types';

	let { children }: LayoutProps = $props();

	const links = [
		{ href: `${resolve('/')}#features`, label: 'Features' },
		{ href: `${resolve('/')}#widgets`, label: 'Widgets' },
		{ href: `${resolve('/')}#developers`, label: 'Developers' },
		{ href: resolve('/docs/[...slug]', { slug: '' }), label: 'Docs' },
		{ href: resolve('/changelog'), label: 'Changelog' }
	];

	// The docs are wider than the other pages, because they have a sidebar, the text and an
	// outline. The header uses the same width.
	const wide = $derived(page.route.id?.startsWith('/docs') ?? false);
</script>

<header
	class="sticky top-0 z-50 border-b border-transparent bg-background/70 backdrop-blur-xl supports-[backdrop-filter]:bg-background/50"
>
	<nav
		class={['mx-auto flex h-16 items-center gap-6 px-4 sm:px-6', wide ? 'max-w-7xl' : 'max-w-6xl']}
	>
		<a href={resolve('/')} class="flex items-center gap-2.5 font-semibold tracking-tight">
			<img src={icon} alt="" class="size-8" />EdgeStation
			<!-- Below lg the badge overlaps the links or the GitHub button. The footer still shows it. -->
			<BetaBadge class="max-lg:hidden" />
		</a>
		<div class="ml-auto hidden items-center gap-6 text-sm text-muted-foreground md:flex">
			{#each links as { href, label } (label)}
				<a {href} class="transition hover:text-foreground">{label}</a>
			{/each}
		</div>
		<div class="ml-auto flex items-center gap-2 md:ml-0">
			<a
				href={REPO_URL}
				class="grid size-9 place-items-center rounded-lg text-muted-foreground transition hover:bg-muted hover:text-foreground"
				aria-label="EdgeStation on GitHub"
			>
				<GithubIcon class="size-5" />
			</a>
			<a
				href={resolve('/download')}
				class="inline-flex h-9 items-center rounded-lg bg-foreground px-4 text-sm font-semibold text-background transition hover:bg-foreground/85"
			>
				Download
			</a>
		</div>
	</nav>
</header>

<main>{@render children()}</main>

<footer class="border-t">
	<div
		class="mx-auto flex max-w-6xl flex-col gap-4 px-4 py-10 text-sm text-muted-foreground sm:flex-row sm:items-center sm:px-6"
	>
		<a href={resolve('/')} class="flex items-center gap-2 font-medium text-foreground">
			<img src={icon} alt="" class="size-6" />EdgeStation
			<BetaBadge />
		</a>
		<p>A widget dashboard for your secondary display.</p>
		<div class="flex gap-5 sm:ml-auto">
			<a href={resolve('/download')} class="hover:text-foreground">Download</a>
			<a href={resolve('/changelog')} class="hover:text-foreground">Changelog</a>
			<a href={REPO_URL} class="hover:text-foreground">GitHub</a>
			<a href={resolve('/docs/[...slug]', { slug: '' })} class="hover:text-foreground">
				Write a widget
			</a>
		</div>
	</div>
	<p class="mx-auto max-w-6xl px-4 pb-10 text-xs text-muted-foreground/70 sm:px-6">
		EdgeStation is an independent project, not affiliated with or endorsed by Corsair.
	</p>
</footer>
