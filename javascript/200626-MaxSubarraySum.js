/*
https://www.codewars.com/kata/54521e9ec8e60bc4de000d6c

optimal solution: 

var maxSequence = function(arr){
  var min = 0, ans = 0, i, sum = 0;
  for (i = 0; i < arr.length; ++i) {
    sum += arr[i];
    min = Math.min(sum, min);
    ans = Math.max(ans, sum - min);
  }
  return ans;
}

*/

const assert = require('assert');

var maxSequence = (arr) => {
    let maxSum = 0;

    for(let i=0;i<arr.length;i++) {
        let partialSum = partialMaxSum(arr, i);
        if(partialSum > maxSum) maxSum = partialSum;
    }

    return maxSum;
}

var partialMaxSum = (arr, index) => {
    var maxNumber = 0;

    for(let i=index;i<arr.length;i++) {
        const subArray = arr.slice(index, i+1);
        const partialSum = subArray.reduce((acc, cur) => acc + cur);
        if(partialSum > maxNumber) maxNumber = partialSum;
    }

    return maxNumber;
}

describe('maximum subarray sum', () => {    
    const testArray = [-2, 1, -3, 4, -1, 2, 1, -5, 4];

    it('empty input', () => {
        assert.equal(maxSequence([]), 0);
    });

    it('tests partial sum', () => {
        assert.equal(partialMaxSum(testArray, 0), 2);
    });

    it('tests slice for small arrays', () => {
        const testArray1 = [1, 2, 3];

        assert.deepEqual(testArray1.slice(1, 3), [2, 3]);
    });

    it('tests maximum partial sum', () => {
        assert.equal(maxSequence(testArray), 6);
    });
});