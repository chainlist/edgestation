import type { Monitor } from '@tauri-apps/api/window';
import { expect, it } from 'vitest';
import { fillsMonitor, monitorAt, pickEdgeMonitor } from './edge-window';

const monitor = (name: string | null, x: number, width = 1920, height = 1080) =>
	({ name, position: { x, y: 0 }, size: { width, height } }) as Monitor;

const main = monitor('DISPLAY1', 0);
const edge = monitor('DISPLAY2', 1920, 2560, 720);
const other = monitor('DISPLAY3', -1920);

it('auto-detects the first screen that is not the main one, whatever its size', () => {
	expect(pickEdgeMonitor([main, edge, other], main, null)).toBe(edge);
	expect(pickEdgeMonitor([main, other, edge], main, null)).toBe(other);
	expect(pickEdgeMonitor([edge, main], main, null)).toBe(edge);
});

it('prefers the monitor picked in the settings, main screen included', () => {
	expect(pickEdgeMonitor([main, edge, other], main, 'DISPLAY3')).toBe(other);
	expect(pickEdgeMonitor([main, edge], main, 'DISPLAY1')).toBe(main);
});

it('auto-detects when the picked monitor is unplugged', () => {
	expect(pickEdgeMonitor([main, edge], main, 'DISPLAY9')).toBe(edge);
});

it('picks nothing with the main screen alone, or no main screen known', () => {
	expect(pickEdgeMonitor([main], main, null)).toBeUndefined();
	expect(pickEdgeMonitor([main, edge], null, null)).toBeUndefined();
});

it('never takes an unnamed monitor for the one picked', () => {
	const unnamed = monitor(null, 0);
	expect(pickEdgeMonitor([unnamed, edge], unnamed, null)).toBe(edge);
});

it('knows a window full screen on the monitor needs no placing', () => {
	expect(fillsMonitor(true, edge, edge)).toBe(true);
});

it('places a window that is not full screen, or not on that monitor', () => {
	expect(fillsMonitor(false, edge, edge)).toBe(false);
	expect(fillsMonitor(true, main, edge)).toBe(false);
	// The corner is the same, but the resolution of the monitor changed.
	expect(fillsMonitor(true, monitor('DISPLAY2', 1920), edge)).toBe(false);
});

it('finds the monitor a window is on from its center', () => {
	const monitors = [main, edge, other];
	expect(monitorAt(monitors, edge)).toBe(edge);
	expect(monitorAt(monitors, other)).toBe(other);
	// A 1280×360 window that is mostly on the main screen and partly on the Edge.
	expect(monitorAt(monitors, monitor(null, 1000, 1280, 360))).toBe(main);
});

it('finds no monitor for a window off every screen', () => {
	expect(monitorAt([main, edge], monitor(null, 9000, 1280, 360))).toBeUndefined();
});
