<script lang="ts">
	import Screenshot from '#lib/components/Screenshot.svelte';
	import settingsEditor from '#lib/assets/settings-editor.png';
	import settingsWidget from '#lib/assets/settings-widget-settings.png';
</script>

<section class="grid items-center gap-12 lg:grid-cols-[2fr_3fr]">
	<div>
		<p class="text-sm font-semibold text-primary">Pages</p>
		<h2 class="mt-2 text-3xl font-bold tracking-tight sm:text-4xl">Lay out pages on a grid</h2>
		<p class="mt-4 text-muted-foreground">
			Each page is its own grid: pick how many columns and rows, add widgets, drag one to move it,
			drag a corner to resize it. The Edge follows as you go.
		</p>
		<ul class="mt-6 space-y-3 text-sm">
			<li>
				<strong>Per-widget settings.</strong> Click a widget to set it up, say which metric or how many
				rows of keys.
			</li>
			<li>
				<strong>Your way of swiping.</strong> Pages side by side or stacked, sliding or fading into each
				other.
			</li>
			<li>
				<strong>As many copies as you like.</strong> Place a widget twice, each with its own settings.
			</li>
		</ul>
	</div>
	<div class="relative pb-20 sm:pb-12">
		<Screenshot
			src={settingsEditor}
			alt="The page editor in Settings: the Home page's 8 × 2 grid with each widget's resize corners."
		/>
		<div class="absolute right-2 bottom-0 w-2/5 sm:-right-6 lg:w-1/3">
			<Screenshot
				src={settingsWidget}
				width={384}
				height={360}
				alt="A Performance widget's settings: metric, direction and period."
			/>
		</div>
	</div>
</section>
