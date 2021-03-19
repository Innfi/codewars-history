/**
 * https://www.codewars.com/kata/539a0e4d85e3425cb0000a88
 */

import assert from 'assert';

function add(x: number): any {
    console.log('args: ', arguments);
    return function(y: number) {
        console.log('args: ', arguments);
        return x+y;
    };
};

describe('Building blocks', () => {

});

describe('ChainAdding', () => {
    it('test case', () => {
        //assert.strictEqual(add(1)(2)(3)(4)(5), 15);
    });
});