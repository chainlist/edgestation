/** Rounds a temprature and adds a degree sign, with no unit. */
export const degrees = (temperature: number) => `${Math.round(temperature)}°`;

/** Hour options for each value of the `hours` setting. `auto` uses the language format. */
export const hourOptions: Record<string, Intl.DateTimeFormatOptions> = {
	auto: { hour: 'numeric' },
	'12': { hour: 'numeric', hourCycle: 'h12' },
	// Two digits, as 24-hour clocks usually show the hour: 09:05.
	'24': { hour: '2-digit', hourCycle: 'h23' }
};

/** Returns a formatter for the unit `name`, such as `percent`, with no decimals. */
export const unit = (name: string) =>
	new Intl.NumberFormat(undefined, { style: 'unit', unit: name, maximumFractionDigits: 0 });

/** Returns the 8-point compass direction of `degrees`. 0 is north. */
export const compass = (degrees: number) =>
	['N', 'NE', 'E', 'SE', 'S', 'SW', 'W', 'NW'][Math.round(degrees / 45) % 8];

/** Returns the WHO name for a UV index. */
export function uvLevel(uv: number) {
	const index = Math.round(uv);
	if (index < 3) return 'Low';
	if (index < 6) return 'Moderate';
	if (index < 8) return 'High';
	if (index < 11) return 'Very high';
	return 'Extreme';
}
