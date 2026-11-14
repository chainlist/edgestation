<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { Attachment } from 'svelte/attachments';
	import type { HTMLAttributes } from 'svelte/elements';
	import { fade, scale } from 'svelte/transition';

	let {
		backdropClass,
		attach,
		onbackdroppointerdown,
		class: className,
		children,
		...card
	}: HTMLAttributes<HTMLDivElement> & {
		/** Classes of the backdrop that covers the Edge: its position and where the card sits. */
		backdropClass: string;
		/** An attachment for the backdrop, for example `overlay` over the web views. */
		attach?: Attachment<HTMLDivElement>;
		onbackdroppointerdown?: (event: PointerEvent) => void;
		children: Snippet;
	} = $props();
</script>

<!-- A glass card over a dimmed Edge, for a question or a picker. The other attributes go on the
     card. -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	{@attach attach}
	class={['inset-0 flex justify-center bg-background/40 p-6', backdropClass]}
	transition:fade={{ duration: 150 }}
	onpointerdown={onbackdroppointerdown}
>
	<div
		{...card}
		class={['rounded-3xl border bg-card/90 shadow-lg backdrop-blur-md', className]}
		in:scale={{ start: 0.96, duration: 150 }}
	>
		{@render children()}
	</div>
</div>
