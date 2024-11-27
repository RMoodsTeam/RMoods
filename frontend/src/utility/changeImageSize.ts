export const changeImageSize = (url: string, newSize: number): string => {
  return url.replace(/s\d+-c/, `s${newSize}-c`);
};