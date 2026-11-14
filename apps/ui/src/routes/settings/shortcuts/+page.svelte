<script lang="ts">
	import RotateCcw from '@lucide/svelte/icons/rotate-ccw';
	import PageHeader from '#lib/components/settings/PageHeader.svelte';
	import SettingsGroup from '#lib/components/settings/SettingsGroup.svelte';
	import ShortcutRow from '#lib/components/settings/ShortcutRow.svelte';
	import { Button } from '#lib/components/ui/button/index.ts';
	import {
		ACTIONS,
		changeShortcut,
		resolveShortcuts,
		SHORTCUT_ACTIONS,
		usedBy,
		type ShortcutAction
	} from '#lib/shortcuts/actions.ts';
	import { shortcuts, shortcutsFailed } from '#lib/state/settings.svelte.ts';

	/** The actions by group, in catalogue order. */
	const GROUPS = [...new Set(ACTIONS.map((action) => SHORTCUT_ACTIONS[action].group))].map(
		(group) => ({ group, actions: ACTIONS.filter((a) => SHORTCUT_ACTIONS[a].group === group) })
	);

	/** Notes that show below the shortcuts of a group. */
	const NOTES: Record<string, string> = {
		Pages:
			'From the last page, Next page wraps around to the first. Scrolling over the page dots on the Edge turns pages too.'
	};

	const current = $derived(resolveShortcuts(shortcuts.current));
	const changed = $derived(Object.keys(shortcuts.current).length > 0);

	function taken(action: ShortcutAction, shortcut: string) {
		const other = usedBy(current, shortcut, action);
		return other ? `${SHORTCUT_ACTIONS[other].label} already uses these keys.` : null;
	}
</script>

<PageHeader
	title="Shortcuts"
	description="Keyboard shortcuts that work from any app, so you can drive the Edge without reaching for it. Click a shortcut, then press the keys you want."
>
	{#snippet actions()}
		<Button variant="outline" disabled={!changed} onclick={() => (shortcuts.current = {})}>
			<RotateCcw />Restore defaults
		</Button>
	{/snippet}
</PageHeader>

<div class="grid gap-8">
	{#each GROUPS as { group, actions } (group)}
		{#snippet groupNote()}{NOTES[group]}{/snippet}
		<SettingsGroup title={group} note={NOTES[group] ? groupNote : undefined}>
			{#each actions as action (action)}
				{@const { label, description } = SHORTCUT_ACTIONS[action]}
				<ShortcutRow
					{label}
					{description}
					value={current[action]}
					fallback={SHORTCUT_ACTIONS[action].default}
					onchange={(value) =>
						(shortcuts.current = changeShortcut(shortcuts.current, action, value))}
					failed={shortcutsFailed.current.includes(action)}
					check={(shortcut) => taken(action, shortcut)}
				/>
			{/each}
		</SettingsGroup>
	{/each}
</div>
