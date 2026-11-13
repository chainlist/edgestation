import { prefersReducedMotion } from 'svelte/motion';

/** The time for a tab highlight to glide to another tab, the same as in the sidebar. */
const GLIDE_MS = 150;

/**
 * Makes the highlight of the open tab glide from the tab that was open before (FLIP). Call it
 * when the component initializes, with a function that reads the open tab. Attach the result to
 * the highlight, which only the open tab draws. The old position of the highlight is messured
 * before the tabs update. If it still glides, this position is part of the way.
 */
export function tabGlide(open: () => unknown) {
	let highlight: HTMLElement | undefined;
	let highlighted: DOMRect | undefined;
	$effect.pre(() => {
		void open();
		highlighted = highlight?.getBoundingClientRect();
	});

	return (node: HTMLElement) => {
		highlight = node;
		const from = highlighted;
		highlighted = undefined;
		if (!from || prefersReducedMotion.current) return;
		const to = node.getBoundingClientRect();
		node.animate(
			[
				{
					translate: `${from.left - to.left}px ${from.top - to.top}px`,
					width: `${from.width}px`,
					height: `${from.height}px`
				},
				{ translate: '0 0', width: `${to.width}px`, height: `${to.height}px` }
			],
			{ duration: GLIDE_MS, easing: 'ease-out' }
		);
	};
}
