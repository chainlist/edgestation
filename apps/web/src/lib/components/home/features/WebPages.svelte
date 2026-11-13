<script lang="ts">
	import EdgeSetup from '#lib/components/EdgeSetup.svelte';
	import Screenshot from '#lib/components/Screenshot.svelte';
	import edgeWeb from '#lib/assets/edge-web.jpg';
	import monitorWeb from '#lib/assets/monitor-web.jpg';
	import settingsProfiles from '#lib/assets/settings-browser-profiles.png';
</script>

<section>
	<div class="max-w-2xl">
		<p class="text-sm font-semibold text-primary">Web widgets</p>
		<h2 class="mt-2 text-3xl font-bold tracking-tight sm:text-4xl">Any web page, as a widget</h2>
		<p class="mt-4 text-muted-foreground">
			The URL and YouTube widgets show sites in a native browser view, so even the ones that refuse
			to be embedded work. Follow a link and a Home button takes you back. A mobile layout fits
			narrow cells.
		</p>
	</div>
	<div class="mt-10">
		<EdgeSetup
			edge={edgeWeb}
			edgeAlt="A second Edge page: a map in a URL widget next to a clock and an internet usage chart."
			monitor={monitorWeb}
			monitorAlt="EdgeStation on a normal monitor behind the Edge: a map in a large URL widget, the EdgeStation docs in the mobile layout of a narrow URL widget, a clock and an internet usage chart."
			side="left"
		/>
	</div>
	<div class="mt-12 grid items-center gap-12 lg:grid-cols-[3fr_2fr]">
		<Screenshot
			src={settingsProfiles}
			alt="Browser profiles in Settings: the Default profile and a Work profile used by a URL widget."
		/>
		<div>
			<h3 class="text-2xl font-bold tracking-tight">Browser profiles</h3>
			<p class="mt-3 text-muted-foreground">
				Each profile keeps its own sign-ins and cookies, so two widgets can show the same site with
				different accounts: your work calendar next to your personal one. Sign-ins last across
				restarts.
			</p>
		</div>
	</div>
</section>
