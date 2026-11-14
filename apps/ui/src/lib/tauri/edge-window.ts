import { invoke } from '@tauri-apps/api/core';
import {
	availableMonitors,
	getCurrentWindow,
	primaryMonitor,
	Window,
	type Monitor
} from '@tauri-apps/api/window';
import type { PhysicalSize } from '@tauri-apps/api/dpi';
import { cleanupOf } from './events';

type Bounds = Pick<Monitor, 'position' | 'size'>;

/** Monitors do not overlap on the desktop, so the top-left corner identifies a monitor. */
const samePosition = (a: Bounds, b: Bounds) =>
	a.position.x === b.position.x && a.position.y === b.position.y;

export const fillsMonitor = (fullscreen: boolean, bounds: Bounds, monitor: Monitor) =>
	fullscreen &&
	samePosition(bounds, monitor) &&
	bounds.size.width === monitor.size.width &&
	bounds.size.height === monitor.size.height;

interface MonitorName {
	x: number;
	y: number;
	name: string;
}

/**
 * Lists the monitors, each with a name that no other monitor has. The settings save this name.
 * On Linux, Tauri names a monitor after its model, so the host adds the connector of the monitor
 * (`XENEON EDGE (DP-2)`). Monitors do not overlap, so the top-left corner finds the name.
 */
export async function listMonitors(): Promise<Monitor[]> {
	const [monitors, names] = await Promise.all([
		availableMonitors(),
		invoke<MonitorName[]>('monitor_names')
	]);
	return monitors.map((monitor) => {
		const { x, y } = monitor.position;
		const named = names.find((name) => name.x === x && name.y === y);
		return { ...monitor, name: named?.name ?? monitor.name };
	});
}

/**
 * Finds the monitor for the Edge window. This is the monitor with the name `monitorName`. If
 * there is no such monitor, it is the first monitor that is not the main screen, of any size.
 * Returns `undefined` if the main screen is the only monitor, or if the main screen is not known.
 */
export function pickEdgeMonitor(
	monitors: Monitor[],
	primary: Monitor | null,
	monitorName: string | null
): Monitor | undefined {
	const named = monitorName === null ? undefined : monitors.find((m) => m.name === monitorName);
	return named ?? (primary ? monitors.find((m) => !samePosition(m, primary)) : undefined);
}

/**
 * Puts the Edge window full screen on the monitor from `pickEdgeMonitor`. If there is no monitor,
 * the window stays a normal window, so you can develop the app on one screen.
 */
export async function placeEdgeWindow(monitorName: string | null) {
	const win = getCurrentWindow();
	const [monitors, primary, fullscreen, position, size] = await Promise.all([
		listMonitors(),
		primaryMonitor(),
		win.isFullscreen(),
		win.outerPosition(),
		win.outerSize()
	]);
	const target = pickEdgeMonitor(monitors, primary, monitorName);
	// When the window leaves full screen, it shows at its normal size for a moment. After a
	// reload, the window is already full screen in place, so do this only for a window that moves.
	if (target && !fillsMonitor(fullscreen, { position, size }, target)) {
		const { x, y } = target.position;
		await invoke('edge_fullscreen', { x, y });
	}
	await win.show();
}

/**
 * Puts the Edge window above the window of another app that covers it. The Edge window has no
 * taskbar buton, so the settings window and the Bring to front shortcut are the only ways back.
 * The window goes above all windows for a moment, then back to the normal order, on top. This
 * raises the window, but the focus stays with the app in front.
 */
export async function bringEdgeToFront() {
	const edge = await Window.getByLabel('edge');
	await edge?.setAlwaysOnTop(true);
	await edge?.setAlwaysOnTop(false);
}

/**
 * Shows or hides the Edge window if `visible` is set, and does not take the focus. Resolves to
 * true if the window shows. The host keeps this state across page reloads (see `edge_visible`).
 * Call it only from the Edge page, because it acts on the calling window.
 */
export const setEdgeVisible = (visible?: boolean) => invoke<boolean>('edge_visible', { visible });

/** Finds the monitor that holds the center of `bounds`. */
export function monitorAt(monitors: Monitor[], bounds: Bounds): Monitor | undefined {
	const x = bounds.position.x + bounds.size.width / 2;
	const y = bounds.position.y + bounds.size.height / 2;
	return monitors.find(
		({ position, size }) =>
			x >= position.x &&
			x < position.x + size.width &&
			y >= position.y &&
			y < position.y + size.height
	);
}

/**
 * Gets the size in pixels of the monitor that holds the Edge window. The window fills this
 * monitor when it is full screen. Returns `undefined` if there is no Edge window, or if no
 * monitor holds it.
 */
export async function edgeMonitorSize(): Promise<PhysicalSize | undefined> {
	const edge = await Window.getByLabel('edge');
	if (!edge) return undefined;
	const [monitors, position, size] = await Promise.all([
		availableMonitors(),
		edge.outerPosition(),
		edge.outerSize()
	]);
	return monitorAt(monitors, { position, size })?.size;
}

/** A size in CSS pixels, the unit of the Edge page layout. */
export interface CssSize {
	width: number;
	height: number;
}

/**
 * Calls `fn` with the size of the Edge window in CSS pixels, first immediatly and then after each
 * change. For example, the size changes when the window moves to another monitor, or when the
 * scaling of that monitor changes. Returns the function that stops the watch.
 */
export function watchEdgeSize(fn: (size: CssSize) => void): () => void {
	const listening = Window.getByLabel('edge')
		.then(async (edge) => {
			if (!edge) return undefined;
			let scale = await edge.scaleFactor();
			const report = ({ width, height }: PhysicalSize) =>
				fn({ width: width / scale, height: height / scale });
			const offs = await Promise.all([
				edge.onResized(({ payload }) => report(payload)),
				edge.onScaleChanged(({ payload }) => {
					scale = payload.scaleFactor;
					report(payload.size);
				})
			]);
			// Read the size after the listeners start, so that no resize between the two is lost.
			edge.innerSize().then(report, console.error);
			return () => offs.forEach((off) => off());
		})
		.catch((error: unknown) => {
			console.error(error);
			return undefined;
		});
	return cleanupOf(listening);
}
