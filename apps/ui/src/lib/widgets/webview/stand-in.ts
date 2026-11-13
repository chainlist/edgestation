import { widgetRoot } from '../shadow';

/**
 * A picture of a native view (`api.webview`) that replaces the view while it hides. It lies
 * over `target`, above the rest of the widget. While the view shows, the veiw covers it. While
 * the view hides, it is in the place of the view. It is part of the page, so it slides with it.
 */
export interface StandIn {
	/** Lays JPEG `picture` over the target when it can draw without a blank frame. */
	show(picture: ArrayBuffer): Promise<void>;
	/** Moves to the target again if the target moved or changed size in the widget. */
	place(): void;
	remove(): void;
}

// The style is inline, so that the `img` rules of a widget cannot move or crop it. It is just
// under the view buttons, which show over it.
const STYLE =
	'position:absolute;margin:0;padding:0;border:0;max-width:none;max-height:none;' +
	'object-fit:fill;pointer-events:none;user-select:none;z-index:2147483646';

export function createStandIn(target: HTMLElement): StandIn {
	let img: HTMLImageElement | null = null;
	let removed = false;

	const drop = (el: HTMLImageElement) => {
		el.remove();
		URL.revokeObjectURL(el.src);
	};

	/** Puts the picture over the target, relative to the widget root that holds it. */
	const place = () => {
		const root = img?.parentElement;
		if (!img || !root) return;
		const t = target.getBoundingClientRect();
		const r = root.getBoundingClientRect();
		img.style.left = `${t.left - r.left}px`;
		img.style.top = `${t.top - r.top}px`;
		img.style.width = `${t.width}px`;
		img.style.height = `${t.height}px`;
	};

	return {
		async show(picture) {
			const root = widgetRoot(target);
			if (!root) throw new Error('The web view target is not in a widget');
			const next = new Image();
			next.alt = '';
			next.draggable = false;
			next.style.cssText = STYLE;
			next.src = URL.createObjectURL(new Blob([picture], { type: 'image/jpeg' }));
			try {
				await next.decode();
			} catch (error) {
				URL.revokeObjectURL(next.src);
				throw error;
			}
			if (removed) return URL.revokeObjectURL(next.src);
			if (img) drop(img);
			img = next;
			root.append(img);
			place();
		},
		place,
		remove() {
			removed = true;
			if (img) drop(img);
			img = null;
		}
	};
}
