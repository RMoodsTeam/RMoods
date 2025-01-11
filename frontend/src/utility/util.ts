export function camelCaseToTitleCase(text: string): string {
  return text
    .replace(/([A-Z])/g, ' $1')
    .replace(/^./, (str) => str.toUpperCase());
}

export function extractAnalysisKinds(analyses: Record<string, any>) {
  const kinds: string[] = [];

  for (const key of Object.keys(analyses)) {
    kinds.push(key);
  }

  return kinds;
}

export function truncateText(text: string, maxLength: number) {
  if (text.length > maxLength) {
    return text.slice(0, maxLength) + '...';
  }
  return text;
}
