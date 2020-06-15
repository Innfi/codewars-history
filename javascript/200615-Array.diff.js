/*

https://www.codewars.com/kata/523f5d21c841566fde000009

one of the optimal solutions was:

function array_diff(a, b) {
  return a.filter(e => !b.includes(e));
}
*/

const assert = require('assert');

function arrayDiff(source, target) {
  if(source.length === 0) return source;
  if(target.length === 0) return source;
  
  let targetDict = {};
  target.forEach(targetElement => {
    targetDict[targetElement] = 1;
  });
  
  let result = [];
  source.forEach(sourceElement => {
    if(sourceElement in targetDict === false) {
      result.push(sourceElement);
    }
  });
  
  return result;
}

describe('kata test', () => {
    it('returns empty with empty source array', () => {
        const source = [];
        const target = [1];

        assert.deepEqual(arrayDiff(source, target), source);
    });

    it('returns empty with empty target array', () => {
        const source = [1, 2, 3];
        const target = [];

        assert.deepEqual(arrayDiff(source, target), source);
    });

    it('returns relative complement case 1', () => {
        const source = [1, 2, 3];
        const target = [2];

        assert.deepEqual(arrayDiff(source, target), [1, 3]);
    });

    it('returns relative complement case 2', () => {
        const source = [1, 2, 2, 5, 3];
        const target = [2];

        assert.deepEqual(arrayDiff(source, target), [1, 5, 3]);
    });
});
