<script lang="ts">
	import Check from '@lucide/svelte/icons/check';
	import Copy from '@lucide/svelte/icons/copy';

	/** Shell commands, one per line. The copy button copies them without the prompt. */
	let { lines }: { lines: string[] } = $props();

	let box: HTMLElement;
	let copied = $state(false);
	let timer: ReturnType<typeof setTimeout> | undefined;

	async function copy() {
		try {
			await navigator.clipboard.writeText(lines.join('\n'));
		} catch {
			// The browser can refuse the clipboard, for example outside HTTPS. Then select the
			// commands (the prompts cannot be selected), so that Ctrl+C copies them.
			getSelection()?.selectAllChildren(box);
			return;
		}
		copied = true;
		clearTimeout(timer);
		timer = setTimeout(() => (copied = false), 1500);
	}
</script>

<!-- A long command wraps, with the next lines indented past the prompt. The button sits beside
the commands, not over them. -->
<div class="flex items-start rounded-xl border bg-background">
	<div
		bind:this={box}
		class="min-w-0 flex-1 py-3.5 pl-4 font-mono text-sm leading-relaxed [overflow-wrap:anywhere]"
	>
		{#each lines as line, i (i)}
			<p class="pl-[2ch] -indent-[2ch]">
				<!-- An expression, because Svelte drops the space at the end of a tag. -->
				<span class="text-muted-foreground select-none">{'$ '}</span>{line}
			</p>
		{/each}
	</div>
	<button
		type="button"
		onclick={copy}
		class="m-2 grid size-8 shrink-0 place-items-center rounded-lg text-muted-foreground transition hover:bg-muted hover:text-foreground"
		aria-label={copied ? 'Copied' : 'Copy the commands'}
	>
		{#if copied}
			<Check class="size-4 text-primary" />
		{:else}
			<Copy class="size-4" />
		{/if}
	</button>
</div>
