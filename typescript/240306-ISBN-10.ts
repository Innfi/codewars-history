// https://www.codewars.com/kata/51fc12de24a9d8cb0e000001

import assert from 'assert';

describe('test', () => {
  function validISBN10(isbn: string): boolean {
    // TODO: return true if (and only if) isbn is a valid 10-digit ISBN.
    try {
      return isbn.length !== 10 ? 
        false :
        isbn.split('').map((value: string, index: number) => {
          if (index === isbn.length-1 && value === 'X') return 10*(index+1);
          if (isNaN(value as any)) throw new Error();
          return Number.parseInt(value)*(index+1);
        }).reduce((a, b) => a+b, 0) % 11 === 0;
    } catch (_) { return false; }
  }

  it ('basic cases', () => {
    const sampleTests: [string, boolean][] = [
      ["1112223339" ,true ],
      ["048665088X" ,true ],
      ["1293000000" ,true ],
      ["1234554321" ,true ],
      ["1234512345" ,false],
      ["1293"       ,false],
      ["X123456788" ,false],
      ["ABCDEFGHIJ" ,false],
      ["XXXXXXXXXX" ,false],
      ["048665088XZ",false]
    ];

    sampleTests.forEach((v) => assert.strictEqual(validISBN10(v[0]), v[1]));
  });
});
