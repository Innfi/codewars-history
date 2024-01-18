/**
 * https://www.codewars.com/kata/55695bc4f75bbaea5100016b
 */

import assert from 'assert';


const fibonacciSequence = () => {
    let value1 = 0, value2 = 1, current = 1;
    let index = 0;
    return {
        [Symbol.iterator]: function(): Iterator<number> { return this; },
        next: function() { 
            index++;
            if(index < 2) current = 1;
            else {
                current = value1+value2;
                value1 = value2;
                value2 = current;
            }

            return {
                done: false, 
                value: current
            };
        }
    };
};

describe('Building blocks', () => {
});

describe('fibonacci streaming', () => {
    it('test case', () => {
        const stream: Iterator<number> = fibonacciSequence();
        assert.strictEqual(stream.next().value, 1);
        assert.strictEqual(stream.next().value, 1);
        assert.strictEqual(stream.next().value, 2);
        assert.strictEqual(stream.next().value, 3);
        assert.strictEqual(stream.next().value, 5);
    });
});
