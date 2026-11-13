import { expect, it } from 'vitest';
import { blurRadius, coverSize, widgetGlassVars } from './background';

const size = (width: number, height: number) => ({ width, height });
const xeneonEdge = size(2560, 720);

it('scales a large image down to just cover the screen', () => {
	expect(coverSize(size(3840, 2160), xeneonEdge)).toEqual(size(2560, 1440));
	expect(coverSize(size(7680, 4320), size(3840, 2160))).toEqual(size(3840, 2160));
	// A portrait screen, for example a rotated Edge, needs the image to cover its heigth.
	expect(coverSize(size(7680, 4320), size(720, 2560))).toEqual(size(4551, 2560));
});

it('keeps every pixel of a tall image a wide screen crops', () => {
	expect(coverSize(size(1440, 5120), xeneonEdge)).toEqual(size(1440, 5120));
});

it('never scales a small image up', () => {
	expect(coverSize(size(1920, 540), xeneonEdge)).toEqual(size(1920, 540));
});

it('never leaves the screen short of a pixel', () => {
	for (const [image, screen] of [
		[size(3001, 1999), size(1366, 768)],
		[size(4000, 3000), size(1080, 1920)],
		[size(2999, 851), xeneonEdge]
	]) {
		const { width, height } = coverSize(image, screen);
		expect(width).toBeGreaterThanOrEqual(Math.min(screen.width, image.width));
		expect(height).toBeGreaterThanOrEqual(Math.min(screen.height, image.height));
	}
});

it('maps blur from 0 to 1 onto a radius, clamped', () => {
	expect(blurRadius(0)).toBe(0);
	expect(blurRadius(0.5)).toBe(blurRadius(1) / 2);
	expect(blurRadius(2)).toBe(blurRadius(1));
	expect(blurRadius(-1)).toBe(0);
});

it('turns widget opacity and blur into card variables', () => {
	expect(widgetGlassVars(1, 0)).toEqual({
		'--widget-opacity': '100%',
		'--widget-backdrop': 'none'
	});
	expect(widgetGlassVars(0.6, 1)).toEqual({
		'--widget-opacity': '60%',
		'--widget-backdrop': `blur(${blurRadius(1)}cqmin)`
	});
	expect(widgetGlassVars(1.5, -1)['--widget-opacity']).toBe('100%');
});
