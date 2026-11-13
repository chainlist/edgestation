/** Rounds a temprature and adds a degree sign, with no unit. */
export const degrees = (temperature: number) => `${Math.round(temperature)}°`;

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
