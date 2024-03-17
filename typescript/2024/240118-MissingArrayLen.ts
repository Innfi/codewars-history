// https://www.codewars.com/kata/57b6f5aadb5b3d0ae3000611

import assert from 'assert';

export function getLengthOfMissingArray(arrayOfArrays:any[]):number {
  if (arrayOfArrays.length <= 0) return 0;
  
  const counters = arrayOfArrays.map((v) => v.length).sort((a, b) => a-b);
  console.log(`counters: ${JSON.stringify(counters)}`);
  
  let current = counters[0];
  for (const subset of counters) {
    if (subset <= 0) return 0;
    if (subset - current > 1) return current+1;
    
    current = subset;
  }
  
  return 0;
}

describe('test', () => {
  it('random case', () => {
    let input = [[3, 2, 0, 4, 3, 1],
      [2, 2, 1, 0, 1],
      [2, 0, 2, 0, 1, 1, 1],
      [3, 4, 0, 2, 4, 0, 2, 3, 4, 0, 4, 4],
      [0, 1, 0, 3, 2, 3, 0, 0, 3, 1, 1],
      [0, 1, 4, 1, 4, 0, 0, 1, 0, 0, 2, 2, 4, 0],
      [0, 2, 1, 3, 1, 1, 3, 3, 3],
      [0, 3, 2, 0],
      [4, 2, 2],
      [4, 2, 3, 4, 3, 0, 2, 2],
      [3, 0, 4, 0, 2, 4, 0, 0, 4, 2, 0, 1, 3]];

    assert.strictEqual(getLengthOfMissingArray(input), 10);
  });
});
