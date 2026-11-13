<script lang="ts">
	import type { WidgetApi } from '@edgestation/sdk';

	interface Task {
		id: string;
		text: string;
		done: boolean;
	}

	let props: { api: WidgetApi } = $props();
	// `api` does not change. A changed setting mounts the widget again.
	// svelte-ignore state_referenced_locally
	const { api } = props;

	/** `undefined` until the widget reads the saved list. */
	let tasks = $state.raw<Task[]>();
	let draft = $state('');
	const left = $derived(tasks?.filter((task) => !task.done).length ?? 0);
	const anyDone = $derived(tasks?.some((task) => task.done) ?? false);

	api.guard(async () => {
		tasks = (await api.storage.widget.get<Task[]>('tasks')) ?? [];
	})();

	const save = api.guard(async (next: Task[]) => {
		tasks = next;
		await api.storage.widget.set('tasks', next);
	});

	function add(event: SubmitEvent) {
		event.preventDefault();
		const text = draft.trim();
		if (!tasks || !text) return;
		save([...tasks, { id: crypto.randomUUID(), text, done: false }]);
		draft = '';
	}

	const toggle = (id: string) =>
		save(tasks!.map((task) => (task.id === id ? { ...task, done: !task.done } : task)));

	const clearDone = () => save(tasks!.filter((task) => !task.done));
</script>

<!-- `el` is the size container, so its padding cannot use cqh or cqw. Put the layout in it. -->
{#if tasks}
	<div class="todo">
		<header>
			<h2>
				To-do{#if tasks.length}<span class="left">{left} left</span>{/if}
			</h2>
			{#if anyDone}<button class="clear" onclick={clearDone}>Clear done</button>{/if}
		</header>

		{#if tasks.length}
			<!-- This holds the overlay scrollbars of the list. -->
			<div class="list">
				<ul {@attach (ul) => api.scrollbars(ul)}>
					{#each tasks as task (task.id)}
						<li>
							<button
								class="task"
								class:done={task.done}
								aria-pressed={task.done}
								onclick={() => toggle(task.id)}
							>
								<span class="check">
									<svg viewBox="0 0 24 24" aria-hidden="true"
										><path d="M5 12.5l4.5 4.5L19 7.5" /></svg
									>
								</span>
								<span class="text">{task.text}</span>
							</button>
						</li>
					{/each}
				</ul>
			</div>
		{:else}
			<p class="empty">Nothing to do</p>
		{/if}

		<form onsubmit={add}>
			<input bind:value={draft} placeholder="Add a task" enterkeyhint="done" maxlength="200" />
			<button class="add" aria-label="Add" disabled={!draft.trim()}>
				<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M12 5v14M5 12h14" /></svg>
			</button>
		</form>
	</div>
{/if}

<style>
	/* A bigger cell shows more tasks, not bigger text. The text size follows only the width of
	   the cell, with limits that keep it readible. */
	.todo {
		display: flex;
		flex-direction: column;
		gap: 0.5em;
		height: 100%;
		padding: 0.75em;
		font-size: clamp(13px, 5cqw, 22px);
	}

	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.5em;
		min-height: 1.8em;
	}

	/* In a narrow cell, an ellipsis cuts the title. The header stays on one line. */
	h2 {
		min-width: 0;
		margin: 0;
		overflow: hidden;
		font-size: 1em;
		font-weight: 600;
		white-space: nowrap;
		text-overflow: ellipsis;
	}

	.left {
		margin-left: 0.6em;
		font-size: 0.8em;
		font-weight: 400;
		color: var(--muted-foreground);
	}

	button {
		padding: 0;
		border: none;
		background: none;
		color: inherit;
		font: inherit;
		cursor: pointer;
	}

	.clear {
		flex: none;
		padding: 0.3em 0.8em;
		border-radius: 999px;
		background: var(--secondary);
		color: var(--secondary-foreground);
		font-size: 0.8em;
		white-space: nowrap;
	}

	.list {
		flex: 1;
		min-height: 0;
	}

	/* The list scrolls inside `.list` and fills it (see `api.scrollbars`). */
	ul {
		margin: 0;
		padding: 0;
		overscroll-behavior: contain;
		list-style: none;
	}

	.task {
		display: flex;
		align-items: center;
		gap: 0.6em;
		width: 100%;
		min-height: 2.2em;
		padding: 0.2em 0.3em;
		border-radius: calc(var(--radius) * 0.5);
		text-align: left;
	}

	.task:active {
		background: var(--muted);
	}

	.check {
		display: grid;
		flex: none;
		place-items: center;
		width: 1.1em;
		height: 1.1em;
		border: 2px solid var(--muted-foreground);
		border-radius: 50%;
	}

	.check svg {
		width: 80%;
		fill: none;
		stroke: currentColor;
		stroke-width: 3;
		stroke-linecap: round;
		stroke-linejoin: round;
		visibility: hidden;
	}

	.done .check {
		border-color: var(--primary);
		background: var(--primary);
		color: var(--primary-foreground);
	}

	.done .check svg {
		visibility: visible;
	}

	.text {
		min-width: 0;
		line-height: 1.25;
		overflow-wrap: anywhere;
	}

	.done .text {
		color: var(--muted-foreground);
		text-decoration: line-through;
	}

	.empty {
		display: grid;
		flex: 1;
		place-items: center;
		margin: 0;
		color: var(--muted-foreground);
	}

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

	.add {
		display: grid;
		flex: none;
		place-items: center;
		width: 2.2em;
		height: 2.2em;
		border-radius: 50%;
		background: var(--primary);
		color: var(--primary-foreground);
	}

	.add svg {
		width: 50%;
		fill: none;
		stroke: currentColor;
		stroke-width: 3;
		stroke-linecap: round;
	}

	.add:disabled {
		opacity: 0.35;
		cursor: default;
	}
</style>
