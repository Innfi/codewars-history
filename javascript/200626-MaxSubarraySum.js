/*
https://www.codewars.com/kata/54521e9ec8e60bc4de000d6c

*/

const assert = require('assert');


var maxSequence = (arr) => {
    return [ 0 ];
}

var partialMaxSum = (arr, index) => {
    let sumArray = [];

    for(let i=0;i<arr.length-index;i++) {
        const subArray = arr.slice(index, i+1);
        sumArray.push(subArray.reduce((acc, cur) => acc + cur));
    }

    return sumArray;
}

describe('maximum subarray sum', () => {    
    const testArray = [-2, 1, -3, 4, -1, 2, 1, -5, 4];

    it('empty input', () => {
        assert.equal(maxSequence([]), 0);
    });

    // it('test case 1', () => {
    //     assert.equal(maxSequence([-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    // });

    // it('test subroutine case 1', () => {
    //     assert.equal(partialMaxSum([-2, 1, -3, 4, -1, 2, 1, -5, 4], 0), 0);
    // });

    it('tests array.slice', () => {
        assert.deepEqual(testArray.slice(0, 1), [-2]);
        assert.deepEqual(testArray.slice(0, 2), [-2, 1]);
    });

    it('tests index', () => {
        assert.deepEqual(partialMaxSum(testArray, 0), 
            [-2, -1, -4, 0, -1, 1, 2, -3, 1]);
    });
});