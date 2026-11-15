<script lang="ts">
	import { page } from '$app/state';
	import image from '#lib/assets/og-image.jpg';

	let { title, description }: { title: string; description: string } = $props();
</script>

<!-- Link previews in social posts and chat apps read the og: and twitter: tags. They need absolute
     URLs, so paths.origin in vite.config.ts gives page.url the real origin when the pages prerender. -->
<svelte:head>
	<title>{title}</title>
	<meta name="description" content={description} />
	<meta property="og:type" content="website" />
	<meta property="og:site_name" content="EdgeStation" />
	<meta property="og:title" content={title} />
	<meta property="og:description" content={description} />
	<meta property="og:url" content={page.url.href} />
	<meta property="og:image" content={new URL(image, page.url.href).href} />
	<meta property="og:image:width" content="1200" />
	<meta property="og:image:height" content="630" />
	<meta
		property="og:image:alt"
		content="EdgeStation on a Corsair Xeneon Edge: a clock, app shortcuts, CPU, GPU and internet tiles, a timeline of the day and a to-do list."
	/>
	<meta name="twitter:card" content="summary_large_image" />
</svelte:head>
