<script lang="ts">
	import Glyph from './Glyph.svelte';

	let {
		editing,
		draft = $bindable(),
		field = $bindable(),
		onsubmit,
		oncancel
	}: {
		editing: boolean;
		draft: string;
		field?: HTMLInputElement;
		onsubmit: (event: SubmitEvent) => void;
		oncancel: () => void;
	} = $props();
</script>

<form {onsubmit}>
	{#if editing}
		<button type="button" class="round" aria-label="Cancel" onclick={oncancel}>
			<Glyph name="cancel" />
		</button>
	{/if}
	<input
		bind:this={field}
		bind:value={draft}
		placeholder={editing ? 'Rewrite the entry' : 'What are you doing?'}
		aria-label={editing ? 'Rewrite the entry' : 'What are you doing?'}
		enterkeyhint="done"
		maxlength="300"
		onkeydown={(event) => event.key === 'Escape' && editing && oncancel()}
	/>
	<button class="round primary" aria-label={editing ? 'Save' : 'Log'} disabled={!draft.trim()}>
		<Glyph name={editing ? 'save' : 'add'} />
	</button>
</form>

<style>
	form {
		display: flex;
		gap: 0.5em;
	}

	input {
		flex: 1;
		min-width: 0;
		height: 2.2em;
		padding: 0 0.7em;
		border: 1px solid var(--border);
		border-radius: calc(var(--radius) * 0.75);
		background: transparent;
		color: inherit;
		font: inherit;
	}

	input::placeholder {
		color: var(--muted-foreground);
	}

	input:focus {
		border-color: var(--ring);
		outline: none;
	}

	.round {
		display: grid;
		flex: none;
		place-items: center;
		width: 2.2em;
		height: 2.2em;
		border-radius: 50%;
		background: var(--secondary);
		color: var(--secondary-foreground);
	}

	.round :global(svg) {
		width: 50%;
		stroke-width: 3;
	}

	.primary {
		background: var(--primary);
		color: var(--primary-foreground);
	}

	.primary:disabled {
		opacity: 0.35;
	}
</style>
