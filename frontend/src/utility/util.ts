export function camelCaseToTitleCase(text: string): string {
  return text
    .replace(/([A-Z])/g, ' $1')
    .replace(/^./, (str) => str.toUpperCase());
}

/**
 * Converts a date to a Unix timestamp in seconds.
 */
export function dateToUnixTimestamp(date: Date): number {
  return Math.floor(date.getTime() / 1000);
}

/**
 * Converts a date to a Unix timestamp in seconds, adjusted for local timezone.
 *
 * For example, if you're in UTC+2 and the date is 2024-02-05T00:00:00Z, this will return the timestamp for 2024-02-05T00:00:00+02:00.
 */
export function getLocalUnixTimestamp(date: Date): number {
  return Math.floor(date.getTime() / 1000) - date.getTimezoneOffset() * 60;
}
