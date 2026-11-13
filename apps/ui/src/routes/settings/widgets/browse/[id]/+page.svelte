<script lang="ts">
	import ExternalLink from '@lucide/svelte/icons/external-link';
	import { on } from 'svelte/events';
	import { page } from '$app/state';
	import PageHeader from '#lib/components/settings/PageHeader.svelte';
	import MarketplaceButtons from '#lib/components/settings/widgets/MarketplaceButtons.svelte';
	import WidgetRow from '#lib/components/settings/widgets/WidgetRow.svelte';
	import { downloadFacts } from '#lib/components/settings/widgets/facts.ts';
	import { Button } from '#lib/components/ui/button/index.ts';
	import * as Card from '#lib/components/ui/card/index.ts';
	import { installed } from '#lib/widgets/installed.svelte.ts';
	import { listedAsWidget } from '#lib/widgets/marketplace/listing.ts';
	import { renderReadme } from '#lib/widgets/marketplace/readme.ts';
	import { loadListing, marketplace, openLink } from '#lib/widgets/marketplace/state.svelte.ts';
	import type { PageProps } from './$types';

	let { params }: PageProps = $props();

	void loadListing();

	// Go back to the list that opened this page: `?from=installed` from Installed, else Browse.
	const back =
		page.url.searchParams.get('from') === 'installed'
			? '/settings/widgets'
			: '/settings/widgets/browse';

	const listed = $derived(marketplace.listing?.find((widget) => widget.id === params.id));
	// A widget that is no longer in the marketplace still shows from its folder while instaled.
	const mine = $derived(
		installed.current.find((widget) => widget.id === params.id && widget.source === 'marketplace')
	);
	const widget = $derived(listed ? listedAsWidget(listed) : mine);

	let readme = $state<string>();
	let readmeError = $state<string>();
	$effect(() => {
		const url = listed?.readme;
		readme = readmeError = undefined;
		if (!url) return;
		let current = true;
		fetch(url)
			.then((response) =>
				response.ok ? response.text() : Promise.reject(new Error(`it answered ${response.status}`))
			)
			.then(
				(markdown) => current && (readme = renderReadme(markdown, url)),
				(error) => current && (readmeError = error instanceof Error ? error.message : String(error))
			);
		return () => (current = false);
	});

	/** Links in the README open in the browser, not in Settings. */
	const followLinks = (node: HTMLElement) =>
		on(node, 'click', (event) => {
			const link = (event.target as Element).closest('a');
			if (!link) return;
			event.preventDefault();
			openLink(link.href).catch(console.error);
		});
</script>

<PageHeader title={widget?.name ?? params.id} {back} />

{#if widget}
	<div class="grid gap-3">
		<Card.Root class="gap-0 py-0">
			<WidgetRow
				{widget}
				icon={listed?.icon}
				more={downloadFacts(listed?.downloads)}
				problem={marketplace.problemOf(params.id)}
			>
				{#snippet actions()}
					<MarketplaceButtons id={params.id} removable />
				{/snippet}
			</WidgetRow>
		</Card.Root>
		{#if listed}
			<div>
				<Button
					variant="link"
					class="h-auto px-1"
					onclick={() => openLink(`https://github.com/${listed.repo}`).catch(console.error)}
				>
					<ExternalLink />{listed.repo}
				</Button>
			</div>
			<Card.Root class="px-6 py-5">
				{#if readme}
					<div class="readme" {@attach followLinks}>
						<!-- renderReadme makes this HTML and keeps it from running. -->
						{@html readme}
					</div>
				{:else if readmeError}
					<p class="text-muted-foreground">No README to show: {readmeError}</p>
				{:else}
					<p class="text-muted-foreground">Loading its README…</p>
				{/if}
			</Card.Root>
		{/if}
	</div>
{:else if marketplace.error}
	<p class="text-destructive">{marketplace.error}</p>
{:else if marketplace.listing}
	<p class="text-muted-foreground">This widget isn't in the marketplace.</p>
{:else}
	<p class="text-muted-foreground">Looking for it…</p>
{/if}

<style>
	/* The Markdown of a README, with the same style as the rest of Settings. */
	.readme :global {
		& > * + * {
			margin-top: 0.75rem;
		}
		h1,
		h2,
		h3,
		h4 {
			margin-top: 1.25rem;
			font-weight: 600;
		}
		h1 {
			font-size: 1.25rem;
		}
		h2 {
			font-size: 1.125rem;
		}
		a {
			color: var(--primary);
			text-decoration: underline;
		}
		ul {
			padding-left: 1.25rem;
			list-style: disc;
		}
		ol {
			padding-left: 1.25rem;
			list-style: decimal;
		}
		code {
			border-radius: 0.25rem;
			background: var(--muted);
			padding: 0.1em 0.3em;
			font-size: 0.875em;
		}
		pre {
			overflow-x: auto;
			border-radius: 0.5rem;
			background: var(--muted);
			padding: 0.75rem 1rem;
		}
		pre code {
			background: none;
			padding: 0;
		}
		img {
			display: inline;
			max-width: 100%;
		}
		blockquote {
			border-left: 3px solid var(--border);
			padding-left: 0.75rem;
			color: var(--muted-foreground);
		}
		th,
		td {
			border: 1px solid var(--border);
			padding: 0.25rem 0.5rem;
		}
	}
</style>
