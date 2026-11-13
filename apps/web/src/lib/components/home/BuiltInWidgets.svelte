<script lang="ts">
	import { builtInWidgets } from '#lib/components/home/built-in-widgets.ts';
</script>

<section id="widgets" class="border-y bg-card/30">
	<div class="mx-auto max-w-6xl px-4 py-24 sm:px-6">
		<div class="max-w-2xl">
			<p class="text-sm font-semibold text-primary">Widgets</p>
			<h2 class="mt-2 text-3xl font-bold tracking-tight sm:text-4xl">
				{builtInWidgets.length} built in, more to add
			</h2>
			<p class="mt-4 text-muted-foreground">
				Settings has a marketplace built in: browse the widgets others publish, install one in a
				click, and it keeps itself up to date.
			</p>
		</div>
		<div class="mt-12 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
			{#each builtInWidgets as { icon: Icon, name, text } (name)}
				<div class="rounded-2xl border bg-card p-5">
					<div class="grid size-10 place-items-center rounded-lg bg-primary/15 text-primary">
						<Icon class="size-5" />
					</div>
					<h3 class="mt-4 font-semibold">{name}</h3>
					<p class="mt-1.5 text-sm text-muted-foreground">{text}</p>
				</div>
			{/each}
		</div>
	</div>
</section>
