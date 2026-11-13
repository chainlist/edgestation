<script lang="ts">
	import Download from '@lucide/svelte/icons/download';
	import type { Installer } from '#lib/downloads.ts';

	let { installer, text }: { installer: Installer; text: string } = $props();
</script>

<!-- One row of a list of files: a click downloads the file. -->
<a
	href={installer.url}
	class="group -mx-2 flex items-start gap-3 rounded-lg px-2 py-1.5 transition hover:bg-muted/60"
>
	<Download class="mt-0.5 size-4 shrink-0 text-primary" />
	<span class="min-w-0">
		<code class="font-mono text-sm break-all group-hover:text-primary">{installer.file}</code>
		<span class="mt-0.5 block text-sm text-muted-foreground">{text}</span>
	</span>
</a>
