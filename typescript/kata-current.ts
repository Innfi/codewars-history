/**
 * https://www.codewars.com/kata/534d2f5b5371ecf8d2000a08
 */

import assert from 'assert';


const multiplicationTable = (size: number): number[][] => {
    let falseResult: number[][] = [];


    return falseResult;
}

describe('Building blocks', () => {
    it('test push subarray', () => {
        let multiArray: number[][] = [];

        let subArray1: number[] = [];
        subArray1.push(1);
        subArray1.push(2);
        let subArray2: number[] = [];
        subArray2.push(2);
        subArray2.push(4);
        
        multiArray.push(subArray1);
        multiArray.push(subArray2);

        assert.deepStrictEqual(multiArray, [[1, 2], [2, 4]]);
    });

    it('old style iteration', () => {
        const size: number = 3;

        let multiArray: number[][] = [];

        for(let i=0;i<size;i++) {
            let starter = i+1;

            let subArray: number[] = [];
            for(let j=0;j<size;j++) subArray.push(starter * (j+1)); 

            multiArray.push(subArray);
        }

        assert.deepStrictEqual(multiArray, [[1,2,3], [2,4,6], [3,6,9]]);
    });

    it('create number array', () => {
        const testArray: number[] = Array.from({length: 4}, (v, k) => k);
        assert.deepStrictEqual(testArray, [0,1,2,3]);
    });
});
