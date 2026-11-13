<script lang="ts">
	import CircleAlert from '@lucide/svelte/icons/circle-alert';
	import RotateCcw from '@lucide/svelte/icons/rotate-ccw';
	import X from '@lucide/svelte/icons/x';
	import { Button } from '#lib/components/ui/button/index.ts';
	import { keyLabels, record, type Shortcut } from '#lib/shortcuts/keys.ts';

	let {
		label,
		description,
		value,
		fallback,
		onchange,
		failed = false,
		check
	}: {
		label: string;
		description?: string;
		value: Shortcut | null;
		/** The default shortcut. A button sets it again when the shortcut is different. */
		fallback: Shortcut | null;
		/** Gets the new shortcut after a recording, or `null` when the user turns it off. */
		onchange: (value: Shortcut | null) => void;
		/** The app cannot register the shortcut, usually because another app has it. */
		failed?: boolean;
		/** Returns why a recorded shortcut is not usable, for example if another action has it. */
		check?: (shortcut: Shortcut) => string | null;
	} = $props();

	/** True while the field records keys. `held` has the modifiers that are down. */
	let recording = $state(false);
	let held = $state<string[]>([]);
	let error = $state<string | null>(null);

	const message = $derived.by(() => {
		if (recording && error) return { error: true, text: error };
		if (recording) {
			return {
				error: false,
				text: 'Press Esc to cancel. Keys another shortcut uses stay with it: turn that one off first.'
			};
		}
		if (failed && value) {
			return {
				error: true,
				text: 'Another app already uses these keys, so this shortcut does nothing. Pick others.'
			};
		}
		return null;
	});

	function start() {
		recording = true;
		held = [];
		error = null;
	}

	function stop() {
		recording = false;
		held = [];
	}

	function keydown(e: KeyboardEvent) {
		if (!recording) return;
		const plain = !e.ctrlKey && !e.altKey && !e.shiftKey && !e.metaKey;
		// Escape cancels. Tab moves the focus, and the blur then stops the recording.
		if (plain && e.code === 'Tab') return;
		e.preventDefault();
		if (plain && e.code === 'Escape') return stop();
		const result = record(e);
		if ('held' in result) {
			held = result.held;
			return;
		}
		error = 'error' in result ? result.error : (check?.(result.shortcut) ?? null);
		if (error || !('shortcut' in result)) return;
		stop();
		onchange(result.shortcut);
	}

	/** When a modifier is released before the key, show the modifers that are still down. */
	function keyup(e: KeyboardEvent) {
		if (!recording) return;
		const result = record(e);
		if ('held' in result) held = result.held;
	}
</script>

{#snippet keys(labels: string[])}
	{#each labels as key, i (i)}
		<kbd
			class="inline-flex h-6 min-w-6 items-center justify-center rounded-md border border-b-2 bg-muted px-1.5 font-sans text-xs font-medium text-foreground"
		>
			{key}
		</kbd>
	{/each}
{/snippet}

<div class="grid gap-2 px-4 py-3">
	<div class="flex items-center gap-4">
		<div class="grid min-w-0 flex-1 gap-0.5">
			<span class="font-medium">{label}</span>
			{#if description}
				<span class="text-muted-foreground">{description}</span>
			{/if}
		</div>

		<!-- The keys of the shortcut. A click records new keys. -->
		<button
			type="button"
			class={[
				'flex h-10 w-52 shrink-0 items-center justify-center gap-1 rounded-lg border bg-background px-3 transition-[border-color,box-shadow,background-color] outline-none hover:bg-muted/60 focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 dark:bg-input/30 dark:hover:bg-input/50',
				recording && 'border-primary! ring-3 ring-primary/25!',
				!recording && failed && value && 'border-destructive/70',
				!recording && !value && 'border-dashed'
			]}
			aria-label="{label}: {value ? keyLabels(value).join(' ') : 'off'}. Change"
			onclick={() => !recording && start()}
			onkeydown={keydown}
			onkeyup={keyup}
			onblur={stop}
		>
			{#if recording}
				{@render keys(held)}
				<span class="animate-pulse text-muted-foreground">
					{held.length ? '…' : 'Press keys…'}
				</span>
			{:else if value}
				{@render keys(keyLabels(value))}
			{:else}
				<span class="text-muted-foreground">Off</span>
			{/if}
		</button>

		<!-- The two buttons keep their place when hidden, so the keys of all rows allign. -->
		<div class="flex shrink-0">
			<Button
				variant="ghost"
				size="icon-sm"
				title="Restore default"
				aria-label="Restore default"
				class={[value === fallback && 'invisible']}
				onclick={() => onchange(fallback)}
			>
				<RotateCcw />
			</Button>
			<Button
				variant="ghost"
				size="icon-sm"
				title="Turn off"
				aria-label="Turn off"
				class={[value === null && 'invisible']}
				onclick={() => onchange(null)}
			>
				<X />
			</Button>
		</div>
	</div>

	{#if message}
		<p
			class={[
				'flex items-center gap-1.5',
				message.error ? 'text-destructive' : 'text-muted-foreground'
			]}
		>
			{#if message.error}<CircleAlert class="size-4 shrink-0" />{/if}
			{message.text}
		</p>
	{/if}
</div>
