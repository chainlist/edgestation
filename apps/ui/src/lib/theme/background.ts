interface Size {
	width: number;
	height: number;
}

/**
 * The blur radius at full blur, in hundredths of the shorter side of the screen. In a size
 * container, such as the preview in settings, this is `cqmin`. Outside one, as in the Edge
 * window, it is the `vmin` of the viewport. Thus a preview of any size blurs like the Edge.
 */
const MAX_BLUR = 5;

const clamp01 = (value: number) => Math.min(Math.max(value, 0), 1);

/** The blur radius in `cqmin` for a blur setting from 0 (sharp) to 1. */
export const blurRadius = (blur: number) => clamp01(blur) * MAX_BLUR;

/**
 * The CSS variables for the widget cards on the Edge (see WidgetSlot). `opacity` is how much the
 * card color covers what is behind it. `blur` is how blurred that content is. Both go from 0 to 1.
 * With no blur there is no backdrop filter, so opaque or sharp cards cost nothing more.
 */
export function widgetGlassVars(opacity: number, blur: number): Record<string, string> {
	const radius = blurRadius(blur);
	return {
		'--widget-opacity': `${Math.round(clamp01(opacity) * 100)}%`,
		'--widget-backdrop': radius > 0 ? `blur(${radius}cqmin)` : 'none'
	};
}

/**
 * Scales `image` down until it just covers `screen` pixel for pixel, as the background covers the
 * Edge (`object-fit: cover`). A smaller image looks enlarged on the Edge. A larger image only
 * makes the settings file bigger. The image never scales up.
 */
export function coverSize(image: Size, screen: Size): Size {
	const scale = Math.min(1, Math.max(screen.width / image.width, screen.height / image.height));
	return { width: Math.round(image.width * scale), height: Math.round(image.height * scale) };
}

/**
 * Reads an image file into a WebP data URL for the Edge background. The image scales down to
 * cover `screen`, the size in pixels of the Edge monitor. An animated image keeps only its first
 * frame. On a larger display later, the image looks enlarged, so the user must pick it agian.
 */
export async function readBackground(file: File, screen: Size): Promise<string> {
	const url = URL.createObjectURL(file);
	try {
		const image = new Image();
		image.src = url;
		await image.decode();
		if (!image.naturalWidth || !image.naturalHeight) throw new Error('The image has no size');

		const { width, height } = coverSize(
			{ width: image.naturalWidth, height: image.naturalHeight },
			screen
		);
		const canvas = document.createElement('canvas');
		canvas.width = width;
		canvas.height = height;
		const context = canvas.getContext('2d');
		if (!context) throw new Error('No canvas to scale the image');
		context.imageSmoothingQuality = 'high';
		context.drawImage(image, 0, 0, width, height);
		return canvas.toDataURL('image/webp', 0.9);
	} finally {
		URL.revokeObjectURL(url);
	}
}
