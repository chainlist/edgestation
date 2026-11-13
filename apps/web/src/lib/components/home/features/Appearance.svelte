<script lang="ts">
	import Screenshot from '#lib/components/Screenshot.svelte';
	import settingsAppearance from '#lib/assets/settings-appearance.png';
</script>

<section class="grid items-center gap-12 lg:grid-cols-[2fr_3fr]">
	<div>
		<p class="text-sm font-semibold text-primary">Appearance</p>
		<h2 class="mt-2 text-3xl font-bold tracking-tight sm:text-4xl">Make it yours</h2>
		<p class="mt-4 text-muted-foreground">
			Dark or light, an accent color from a preset or fine-tuned by hue and saturation, and the
			corner radius of every card. Put an image behind the pages, still while they slide, and give
			the widget cards a frosted glass look.
		</p>
		<p class="mt-4 text-muted-foreground">
			Widgets follow the theme too, through the same CSS variables the app uses.
		</p>
	</div>
	<Screenshot
		src={settingsAppearance}
		alt="Appearance settings: dark or light mode, corner radius, accent color presets, hue and saturation."
	/>
</section>
