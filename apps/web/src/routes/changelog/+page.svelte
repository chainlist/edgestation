<script lang="ts">
	import '../docs/docs.css';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<svelte:head>
	<title>EdgeStation changelog</title>
	<meta name="description" content="What changed in each release of EdgeStation and its SDK." />
</svelte:head>

<article class="mx-auto max-w-3xl px-4 py-14 sm:px-6 sm:py-20">
	<header class="border-b pb-8">
		<h1 class="text-3xl font-bold tracking-tight sm:text-4xl">Changelog</h1>
		<p class="mt-3 text-lg text-pretty text-muted-foreground">
			What changed in each release of EdgeStation and its SDK.
		</p>
	</header>

	<div class="doc">
		{@html data.html}
	</div>
</article>
