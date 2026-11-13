<script lang="ts">
	import Basics from '#lib/components/home/Basics.svelte';
	import BuiltInWidgets from '#lib/components/home/BuiltInWidgets.svelte';
	import CallToAction from '#lib/components/home/CallToAction.svelte';
	import Developers from '#lib/components/home/Developers.svelte';
	import Appearance from '#lib/components/home/features/Appearance.svelte';
	import DisplayAndShortcuts from '#lib/components/home/features/DisplayAndShortcuts.svelte';
	import Mixer from '#lib/components/home/features/Mixer.svelte';
	import PagesAndGrid from '#lib/components/home/features/PagesAndGrid.svelte';
	import WebPages from '#lib/components/home/features/WebPages.svelte';
	import Hero from '#lib/components/home/Hero.svelte';
	import LightByDesign from '#lib/components/home/LightByDesign.svelte';
</script>

<svelte:head>
	<title>EdgeStation: widgets for your secondary display</title>
	<meta
		name="description"
		content="EdgeStation turns a secondary display into a dashboard of swipeable widget pages. Made for the Corsair Xeneon Edge, works on any display."
	/>
</svelte:head>

<Hero />
<Basics />

<div id="features" class="mx-auto max-w-6xl space-y-32 px-4 py-28 sm:px-6">
	<PagesAndGrid />
	<WebPages />
	<Mixer />
	<Appearance />
	<DisplayAndShortcuts />
</div>

<BuiltInWidgets />
<LightByDesign />
<Developers />
<CallToAction />
