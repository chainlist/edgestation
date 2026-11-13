<script lang="ts" module>
	export interface Choice {
		value: string;
		label: string;
	}
</script>

<script lang="ts">
	import * as Select from '#lib/components/ui/select/index.ts';

	let {
		choices,
		value,
		placeholder = 'Select…',
		class: className = 'w-64',
		onchange
	}: {
		choices: Choice[];
		value: string;
		placeholder?: string;
		class?: string;
		onchange: (value: string) => void;
	} = $props();

	const selected = $derived(choices.find((choice) => choice.value === value));
</script>

<Select.Root type="single" {value} onValueChange={onchange}>
	<Select.Trigger class={className}>{selected?.label ?? placeholder}</Select.Trigger>
	<Select.Content>
		{#each choices as choice (choice.value)}
			<Select.Item value={choice.value} label={choice.label} />
		{/each}
	</Select.Content>
</Select.Root>
