/**
 * https://www.codewars.com/kata/54ff3102c1bad923760001f3
 * 
 * export class Kata {
  static getCount(str: string) {
    let list = str.match(/[aeiou]/gi);
    return list ? list.length : 0;
  }
}
 */

import assert from 'assert';

const getCount = (str: string): number => {
    return str.length - str.replace(/[e|i|o|u]/g, 'a').replace(/a/g, '').length;
};

describe('Building blocks', () => {
    it('find single letter', () => {
       const input: string = 'abeeriiacayyyyooohdabruuua';

       const result = input.replace(/[e|i|o|u]/g, 'a');
       console.log('result: ', result);
       console.log('result2: ', result.replace(/a/g, ''));
    });
});

describe('Vowel count', () => {
    it('test case', () => {
        assert.strictEqual(getCount('abracadabra'), 5);
    });
});