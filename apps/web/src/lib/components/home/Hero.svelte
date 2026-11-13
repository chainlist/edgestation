<script lang="ts">
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import { resolve } from '$app/paths';
	import BetaBadge from '#lib/components/BetaBadge.svelte';
	import EdgeSetup from '#lib/components/EdgeSetup.svelte';
	import GithubIcon from '#lib/components/GithubIcon.svelte';
	import { REPO_URL } from '#lib/site.ts';
	import edgeHome from '#lib/assets/edge-home.jpg';
	import monitorHome from '#lib/assets/monitor-home.jpg';
</script>

<section class="relative overflow-hidden">
	<div
		class="pointer-events-none absolute inset-x-0 -top-40 h-[640px] bg-[radial-gradient(ellipse_at_center,oklch(0.55_0.2_260/0.35),transparent_65%)]"
		aria-hidden="true"
	></div>
	<div class="relative mx-auto max-w-6xl px-4 pt-12 pb-8 text-center sm:px-6 sm:pt-16 sm:pb-10">
		<p
			class="mx-auto inline-flex items-center gap-2 rounded-full border bg-card/60 px-3.5 py-1 text-xs text-muted-foreground sm:text-sm"
		>
			<BetaBadge />
			Made for the Corsair Xeneon Edge. Works on any display.
		</p>
		<h1 class="mx-auto mt-5 max-w-3xl text-4xl font-bold tracking-tight text-balance sm:text-6xl">
			Give your second screen
			<span class="bg-gradient-to-r from-[#6FA9FF] to-[#3D7AF7] bg-clip-text text-transparent"
				>something to do</span
			>
		</h1>
		<p class="mx-auto mt-4 max-w-2xl text-lg text-pretty text-muted-foreground">
			EdgeStation fills a secondary display with swipeable pages of widgets: the time, how hard your
			PC is working, what's playing, your to-dos, app shortcuts and whole web pages.
		</p>
		<div class="mt-7 flex flex-wrap items-center justify-center gap-3">
			<a
				href={resolve('/download')}
				class="inline-flex h-12 items-center gap-2 rounded-xl bg-primary px-6 font-semibold text-primary-foreground shadow-[0_8px_30px_-8px] shadow-primary/60 transition hover:brightness-110"
			>
				Download <ArrowRight class="size-4" />
			</a>
			<a
				href={REPO_URL}
				class="inline-flex h-12 items-center gap-2 rounded-xl border bg-card/60 px-6 font-semibold transition hover:bg-muted"
			>
				<GithubIcon class="size-5" />View on GitHub
			</a>
		</div>
		<p class="mt-3 text-sm text-muted-foreground">For Windows and Linux</p>
	</div>

	<div class="relative mx-auto max-w-6xl px-4 pb-16 sm:px-6 sm:pb-20">
		<EdgeSetup
			edge={edgeHome}
			edgeAlt="The EdgeStation Home page on a Xeneon Edge: a clock, an app launcher, CPU, internet, GPU load and temperature tiles, a timeline of the day and a to-do list."
			monitor={monitorHome}
			monitorAlt="EdgeStation on a normal 1920 × 1080 monitor behind the Edge: a clock, the weather, what's playing, a to-do list, CPU, GPU and internet tiles, and mute buttons."
			eager
		/>
		<p class="mt-4 text-center text-sm text-muted-foreground">
			On the Edge (2560 × 720) and on a normal monitor (1920 × 1080).
		</p>
	</div>
</section>
