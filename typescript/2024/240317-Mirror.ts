// https://www.codewars.com/kata/586305e8916e244b66001a93

import assert from 'assert';

const mirror = (obj: Record<string,any>): Record<string,string> => {
  const out: Record<string, any> = {};
  Object.keys(obj).forEach((v) => out[v] = v.split('').reverse().join(''));

  return out;
};

describe('test', () => {
  it ('test mirror', () => {
    const input = {
      abc: undefined,
      arara: undefined,
      xyz: undefined
    };

    assert.deepStrictEqual(mirror(input), {
      abc: 'cba',
      arara: 'arara',
      xyz: 'zyx',
    });
  });

  it('reverse', () => {
    assert.strictEqual('abcde'.split('').reverse().join(''), 'edcba');
  });

  it ('test record', () => {
    const input: Record<string, any> = {
      first: undefined,
      second: 'exist',
    };

    assert.strictEqual(input.first, undefined);
    assert.strictEqual(input.second, 'exist');

    Object.keys(input).forEach((v) => {
      input[v] = v;
    });

    assert.strictEqual(input.first, 'first');
    assert.strictEqual(input.second, 'second');
  });
});
