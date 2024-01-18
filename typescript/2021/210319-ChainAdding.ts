/**
 * https://www.codewars.com/kata/539a0e4d85e3425cb0000a88
 * 
 * export default function add(x: number): any {
  const fn = (y: number) => add(x + y);
  fn.valueOf = () => x;
  return fn;
}
 */

import assert from 'assert';


describe('Building blocks', () => {
});

const infiniteCurry = (fn: Function, seed: number) => {
    const reduceValue = (input: number[], seedValue: number) => {
        return input.reduce((acc: number, a: number) => { 
            return fn.call(fn, acc, a);
        }, seedValue);
    };

    const next = (...args: number[]) => {
        console.log('next] input: ', args);
        return (...x) => {
            if(!x.length) {
                const result = reduceValue(args, seed);
                return result;
            }

            return next(...args, reduceValue(x, seed));
        };
    };

    return next();
};

const add = infiniteCurry((x: number, y: number) => x+y, 0);

describe('ChainAdding', () => {
    it('test case', () => {
        console.log(add(1)(2)(3)(4)(5)());
        //assert.strictEqual(add(1)(2)(3)(4)(5), 15);
    });
});