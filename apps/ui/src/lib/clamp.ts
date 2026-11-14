/** Returns `value` kept in the range from `min` to `max`. */
export const clamp = (value: number, min: number, max: number) =>
	Math.min(Math.max(value, min), max);
