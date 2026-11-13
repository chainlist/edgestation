<script lang="ts">
	const light = [
		{
			title: 'Read on demand',
			text: 'CPU, GPU and network are read when a widget asks. Nothing monitors in between.'
		},
		{
			title: 'Off screen, on hold',
			text: "Widget timers wait while their page isn't shown, and while a game is full screen."
		},
		{
			title: 'Contained failures',
			text: 'Each widget runs within its own boundary. If one fails, the error stays in its tile and EdgeStation keeps running.'
		}
	];
</script>

<section class="mx-auto max-w-6xl px-4 py-24 sm:px-6">
	<div class="max-w-2xl">
		<p class="text-sm font-semibold text-primary">Light by design</p>
		<h2 class="mt-2 text-3xl font-bold tracking-tight sm:text-4xl">Always on, never in the way</h2>
		<p class="mt-4 text-muted-foreground">
			A dashboard runs all day, so EdgeStation keeps its footprint small.
		</p>
	</div>
	<div class="mt-12 grid gap-x-8 gap-y-10 sm:grid-cols-3">
		{#each light as { title, text } (title)}
			<div class="border-l-2 border-primary/60 pl-4">
				<h3 class="font-semibold">{title}</h3>
				<p class="mt-1.5 text-sm text-muted-foreground">{text}</p>
			</div>
		{/each}
	</div>
</section>
