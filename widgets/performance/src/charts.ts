import type { SystemInfo, WidgetSettingValues } from '@edgestation/sdk';
import { celsius, percent, speed } from './format.ts';

/** One line of the chart. `icon` marks the value of the line if the chart has two lines. */
interface Series {
	icon?: string;
	read: (reading: SystemInfo) => number | undefined;
}

interface Chart {
	label: string;
	format: (value: number) => string;
	/** Top of the chart. If not set, the top is the highest value shown, but not below `min`. */
	max?: number;
	min?: number;
	series: Series[];
	/** The number of programs with the highest load to show over the chart. */
	top?: number;
}

type Period = Exclude<keyof SystemInfo['net'], 'down' | 'up'>;

/** The width of the chart in readings, one each second. */
export const HISTORY = 60;

const PERIODS: Record<string, string> = {
	now: '',
	avg1m: ' · 1 min avg',
	avg5m: ' · 5 min avg',
	avg15m: ' · 15 min avg'
};
const DIRECTIONS: Record<string, string> = { down: 'Download', up: 'Upload', both: 'Internet' };

/** Returns the chart of a numeric metric. `undefined` for text metrics (Wi-Fi, machine name). */
export function numeric({
	metric,
	direction,
	period,
	top
}: Readonly<WidgetSettingValues>): Chart | undefined {
	switch (metric) {
		case 'cpuLoad':
			return {
				label: 'CPU load',
				format: percent.format,
				max: 100,
				series: [{ read: (r) => r.cpu.load }],
				top: Number(top)
			};
		case 'gpuLoad':
			return {
				label: 'GPU load',
				format: percent.format,
				max: 100,
				series: [{ read: (r) => r.gpu?.load }]
			};
		case 'gpuTemp':
			return {
				label: 'GPU temperature',
				format: celsius.format,
				max: 100,
				series: [{ read: (r) => r.gpu?.temperature }]
			};
		case 'net': {
			const rate = (r: SystemInfo) => (period === 'now' ? r.net : r.net[period as Period]);
			const down: Series = { icon: '↓', read: (r) => rate(r).down };
			const up: Series = { icon: '↑', read: (r) => rate(r).up };
			const series: Record<string, Series[]> = { down: [down], up: [up], both: [down, up] };
			return {
				label: DIRECTIONS[direction] + PERIODS[period],
				format: speed,
				// No fixed `max`. The minimum top is 100 kb/s (in bytes), so the line of an idle
				// network stays flat.
				min: 12_500,
				series: series[direction]
			};
		}
	}
}
