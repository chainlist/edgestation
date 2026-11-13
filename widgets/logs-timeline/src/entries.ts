/** One line of the log. It tells what the user does, and when the user typed it. */
export interface Entry {
	id: string;
	/** The time of the entry, in ms since the epoch. */
	at: number;
	text: string;
}

/** Storage key of the days with entries, as `YYYY-MM-DD`, in date order. */
export const DAYS = 'days';
/** Storage key of the entries of one day, oldest first. */
export const dayKey = (day: string) => `day/${day}`;
