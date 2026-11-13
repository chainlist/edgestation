<script lang="ts">
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import BookOpen from '@lucide/svelte/icons/book-open';
	import { resolve } from '$app/paths';

	const api = [
		'api.system()',
		'api.topPrograms()',
		'api.media',
		'api.audio',
		'api.apps',
		'api.storage',
		'api.webview()',
		'api.every()',
		'api.settings'
	];

	const example = `export default {
  mount(el, api) {
    api.every(1000, async () => {
      const { cpu } = await api.system();
      el.textContent = \`\${Math.round(cpu.load)} %\`;
    });
  }
};`;
</script>

<section id="developers" class="border-t">
	<div class="mx-auto grid max-w-6xl items-center gap-12 px-4 py-24 sm:px-6 lg:grid-cols-2">
		<div>
			<p class="text-sm font-semibold text-primary">Developers</p>
			<h2 class="mt-2 text-3xl font-bold tracking-tight sm:text-4xl">Write your own widget</h2>
			<p class="mt-4 text-muted-foreground">
				A widget is a folder with a <code>manifest.json</code> and a JavaScript module, loaded at
				runtime like a plugin. Write it by hand, or with
				<code>@edgestation/sdk</code>: TypeScript types, adapters for Svelte, React and Vue, a Vite
				plugin that builds it, and a schema for the manifest.
			</p>
			<div class="mt-6 flex flex-wrap gap-2">
				{#each api as name (name)}
					<code class="rounded-md border bg-card px-2 py-1 text-xs text-muted-foreground"
						>{name}</code
					>
				{/each}
			</div>
			<div class="mt-8">
				<a
					href={resolve('/docs/[...slug]', { slug: '' })}
					class="inline-flex h-11 items-center gap-2 rounded-xl border bg-card px-5 font-semibold transition hover:bg-muted"
				>
					<BookOpen class="size-4" />SDK documentation <ArrowRight class="size-4" />
				</a>
			</div>
		</div>
		<div class="overflow-hidden rounded-2xl border bg-card">
			<div class="flex items-center gap-2 border-b px-4 py-3 text-xs text-muted-foreground">
				<span class="size-2.5 rounded-full bg-muted"></span>
				<span class="size-2.5 rounded-full bg-muted"></span>
				<span class="size-2.5 rounded-full bg-muted"></span>
				<span class="ml-2 font-mono">my-widget/main.js</span>
			</div>
			<pre class="overflow-x-auto p-5 font-mono text-sm leading-relaxed"><code>{example}</code
				></pre>
		</div>
	</div>
</section>

<style>
	code:not(pre code) {
		font-size: 0.875em;
	}
</style>
