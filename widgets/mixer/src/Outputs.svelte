<script lang="ts">
	import type { AudioOutput } from '@edgestation/sdk';
	import Glyph from './Glyph.svelte';
	import PadBody from './PadBody.svelte';
	import PadGrid from './PadGrid.svelte';

	let {
		outputs,
		current,
		pick
	}: {
		outputs: AudioOutput[];
		/** The id of the default output. */
		current: string | undefined;
		pick: (id: string) => void;
	} = $props();

	/**
	 * Splits `Speakers (Realtek(R) Audio)` into the name of the output and the name of its
	 * hardware, which Windows adds in brackets.
	 */
	function parts(name: string) {
		const match = /^(.+?)\s*\((.+)\)$/.exec(name);
		return match ? { device: match[1], hardware: match[2] } : { device: name, hardware: '' };
	}
</script>

<PadGrid count={outputs.length}>
	{#each outputs as output (output.id)}
		{@const { device, hardware } = parts(output.name)}
		{@const active = output.id === current}
		<button
			class="pad"
			class:idle={!active}
			aria-pressed={active}
			title={output.name}
			onclick={() => pick(output.id)}
		>
			<PadBody name={device} detail={hardware}><Glyph name="speaker" /></PadBody>
		</button>
	{:else}
		<button class="pad off" disabled>
			<PadBody name="No sound output" detail=""><Glyph name="speakerOff" /></PadBody>
		</button>
	{/each}
</PadGrid>

<style>
	/* An output that is not the default. It stays dark, so that the lit pad shows the default. */
	.pad.idle {
		border-color: color-mix(in oklch, var(--card-foreground) 18%, transparent);
		background: color-mix(in oklch, var(--card-foreground) 6%, transparent);
		color: var(--card-foreground);
		box-shadow: none;
	}
</style>
