export function camelCaseToTitleCase(text: string): string {
  return text
    .replace(/([A-Z])/g, ' $1')
    .replace(/^./, (str) => str.toUpperCase());
}

/**
 * Converts a date to a Unix timestamp in seconds, adjusted for local timezone.
 */
export function shiftToUTC(date: Date): number {
  const newDate = new Date(date.getTime() + date.getTimezoneOffset());
  return Math.ceil(newDate.getTime() / 1000);
}
