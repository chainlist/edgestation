/** Hour options for each value of the `hours` setting. `auto` uses the language format. */
export const hourOptions: Record<string, Intl.DateTimeFormatOptions> = {
	auto: { hour: 'numeric' },
	'12': { hour: 'numeric', hourCycle: 'h12' },
	// Two digits, as 24-hour clocks usually show the hour: 09:05.
	'24': { hour: '2-digit', hourCycle: 'h23' }
};

export const dateFormat = new Intl.DateTimeFormat(undefined, {
	weekday: 'short',
	day: 'numeric',
	month: 'short'
});
const yearFormat = new Intl.DateTimeFormat(undefined, {
	weekday: 'short',
	day: 'numeric',
	month: 'short',
	year: 'numeric'
});

const pad = (n: number) => String(n).padStart(2, '0');

/** The local calendar day of `date` as `YYYY-MM-DD`. This fromat sorts in date order. */
export const dayOf = (date: Date) =>
	`${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}`;

/** Local midnight at the start of `day`. */
export function dateOf(day: string) {
	const [year, month, date] = day.split('-').map(Number);
	return new Date(year, month - 1, date);
}

export function dayBefore(day: string) {
	const date = dateOf(day);
	date.setDate(date.getDate() - 1);
	return dayOf(date);
}

/** The label of `day` in the day selector: a word for today and yesterday, or the date. */
export function dayName(day: string, today: string) {
	if (day === today) return 'Today';
	if (day === dayBefore(today)) return 'Yesterday';
	const format = day.slice(0, 4) === today.slice(0, 4) ? dateFormat : yearFormat;
	return format.format(dateOf(day));
}
