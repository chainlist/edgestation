<script lang="ts">
	import { Slider } from '#lib/components/ui/slider/index.ts';

	let {
		id,
		value,
		min,
		max,
		step,
		format = String,
		disabled = false,
		onchange
	}: {
		id?: string;
		value: number;
		min: number;
		max: number;
		step: number;
		format?: (value: number) => string;
		disabled?: boolean;
		onchange: (value: number) => void;
	} = $props();
</script>

<!-- A slider with its value beside it, sized for the control side of a SettingsRow. -->
<div class={['flex w-64 items-center gap-3', disabled && 'opacity-50']}>
	<Slider {id} type="single" {value} {min} {max} {step} {disabled} onValueChange={onchange} />
	<span class="w-12 shrink-0 text-right text-muted-foreground tabular-nums">{format(value)}</span>
</div>
