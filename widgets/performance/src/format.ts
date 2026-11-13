const unit = (name: string, digits = 0) =>
	new Intl.NumberFormat(undefined, { style: 'unit', unit: name, maximumFractionDigits: digits });
export const percent = unit('percent');
export const celsius = unit('celsius');
const speeds = ['bit', 'kilobit', 'megabit', 'gigabit'];

/** Formats bytes per second as bits per second, with the Task Manager units (kb/s, Mb/s, ...). */
export function speed(bytes: number) {
	let value = bytes * 8;
	let step = 0;
	while (value >= 1000 && step < speeds.length - 1) {
		value /= 1000;
		step++;
	}
	return unit(`${speeds[step]}-per-second`, value < 10 ? 1 : 0).format(value);
}
