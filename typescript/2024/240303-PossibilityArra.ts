// https://www.codewars.com/kata/59b710ed70a3b7dd8f000027

import assert from 'assert';

describe('test', () => {
  function isAllPossibilities(x: number[]): boolean {
    x.sort((a, b) => a-b);

    return x.every((value: number, index: number) => value === index);
  }

  it ('test cases', () => {
    const tests: [number[], boolean][] = [
      [[0,2,19,4,4], false],
      [[3,2,1,0], true],
      [[0,1,2,3], true],
      [[1,2,3,4], false],
      [[0,2,3], false],
      [[0], true],
      [[0,1,2,3,4,5,6,7,8,9], true],
      [[0,1,3,-2,5,4], false],
      [[1,-1,2,-2,3,-3,6], false],
      [[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27], true],
    ];

    tests.forEach((v) => {
      assert.strictEqual(isAllPossibilities(v[0]), v[1]);
    });
  });
});