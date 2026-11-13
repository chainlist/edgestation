<script lang="ts">
	import EdgeSetup from '#lib/components/EdgeSetup.svelte';
	import edgeMixer from '#lib/assets/edge-mixer.jpg';
	import monitorMusic from '#lib/assets/monitor-music.jpg';
</script>

<section>
	<div class="max-w-2xl">
		<p class="text-sm font-semibold text-primary">Mixer</p>
		<h2 class="mt-2 text-3xl font-bold tracking-tight sm:text-4xl">
			Every app's volume, at a touch
		</h2>
		<p class="mt-4 text-muted-foreground">
			Faders with live level meters for the master, your microphone and each app playing sound. They
			stand up in a wide cell and lie across in a tall one, or give way to mute buttons: every
			channel at once, or one per placement. Tap an icon to mute it, the output's name to switch
			device, and Mic on to mute your microphone in every app. Or make it an output selector: a
			button for each output device, with the active one lit.
		</p>
	</div>
	<div class="mt-10">
		<EdgeSetup
			edge={edgeMixer}
			edgeAlt="An Edge page of Mixer widgets: faders standing up for every channel in a wide cell, the same faders lying across in a tall one, mute buttons for every channel, and single Master and Microphone buttons."
			monitor={monitorMusic}
			monitorAlt="EdgeStation on a normal monitor behind the Edge: what's playing, a clock, the weather, faders for every channel, Master and Microphone buttons, and CPU and GPU tiles."
			side="right"
		/>
	</div>
</section>
