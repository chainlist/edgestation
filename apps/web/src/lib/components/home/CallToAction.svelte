<script lang="ts">
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import { resolve } from '$app/paths';
	import { VERSION } from '#lib/site.ts';
</script>

<section class="relative overflow-hidden border-t">
	<div
		class="pointer-events-none absolute inset-0 bg-[radial-gradient(ellipse_at_bottom,oklch(0.55_0.2_260/0.3),transparent_60%)]"
		aria-hidden="true"
	></div>
	<div class="relative mx-auto max-w-3xl px-4 py-24 text-center sm:px-6">
		<h2 class="text-3xl font-bold tracking-tight sm:text-4xl">
			Put the screen below your screen to work
		</h2>
		<p class="mt-4 text-muted-foreground">EdgeStation for Windows and Linux, version {VERSION}.</p>
		<a
			href={resolve('/download')}
			class="mt-8 inline-flex h-12 items-center gap-2 rounded-xl bg-primary px-6 font-semibold text-primary-foreground shadow-[0_8px_30px_-8px] shadow-primary/60 transition hover:brightness-110"
		>
			Download <ArrowRight class="size-4" />
		</a>
	</div>
</section>
