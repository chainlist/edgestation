<script lang="ts">
	import { Input } from '#lib/components/ui/input/index.ts';

	let {
		label,
		value,
		min,
		max,
		onchange
	}: {
		/** Not visible on the field. Screen readers and the tooltip use it. */
		label: string;
		value: number;
		min: number;
		max: number;
		onchange: (value: number) => void;
	} = $props();

	function commit(input: HTMLInputElement) {
		const next = input.valueAsNumber;
		if (Number.isInteger(next)) onchange(Math.min(Math.max(next, min), max));
		else input.valueAsNumber = value;
	}
</script>

<Input
	type="number"
	aria-label={label}
	title={label}
	{min}
	{max}
	{value}
	class="w-16"
	onchange={(event) => commit(event.currentTarget)}
/>
