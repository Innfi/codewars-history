import assert from 'assert';

/**
 * https://www.codewars.com/kata/54b724efac3d5402db00065e
 */

import { MORSE_CODE } from './preloaded';
export function decodeMorse(morseCode: string): string {
  return morseCode
  .trim()
  .replace(/\s{3}/g, ' T ')
  .split(' ').map((value: string) => {
      if(value === 'T') return ' ';

      return MORSE_CODE[value];
  }).join('');
}