/**
 * Changes the requested size of a Google profile picture URL.
 * @param {string} url - The original URL of the Google profile picture.
 * @param {number} newSize - The new size to request for the profile picture.
 * @returns {string} The modified URL with the new size.
 * @example
 * // Original URL: https://lh3.googleusercontent.com/a-/AOh14Gg6s9c=s96-c
 * const newUrl = changeDefaultGoogleProfilePicureSize('https://lh3.googleusercontent.com/a-/AOh14Gg6s9c=s96-c', 250);
 * // newUrl: https://lh3.googleusercontent.com/a-/AOh14Gg6s9c=s250-c
 */

export const changeDefaultGoogleProfilePicureSize = (url: string, newSize: number): string => {
  return url.replace(/s\d+-c/, `s${newSize}-c`);
};