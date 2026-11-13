/** The range of the meters, in decibels below full scale. A quieter sound shows nothing. */
const METER_DB = 60;
/**
 * The fraction of a meter that a level falls in each frame of levels. When there is sound, a
 * frame comes every 40 ms, so a level falls three quarters of a meter each second. So peaks
 * stay long enough to see. A level rises immediatly.
 */
export const FALL = 0.03;
/** The number of steps for the meter height. A meter moves only when its step changes. */
export const STEPS = 32;

export const clamp = (value: number) => Math.min(1, Math.max(0, value));

/** Changes a peak sample (0 to 1) to a meter height (0 to 1), on the decibel scale of the ear. */
export const meter = (peak: number) =>
	peak > 0 ? clamp(1 + (20 * Math.log10(peak)) / METER_DB) : 0;

export const sameLevels = (a: Record<string, number>, b: Record<string, number>) =>
	Object.keys(a).length === Object.keys(b).length &&
	Object.entries(b).every(([id, level]) => a[id] === level);
