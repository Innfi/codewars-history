/**
 * https://www.codewars.com/kata/515f51d438015969f7000013
 */

import assert from 'assert';

const pyramid = (n:number): Array<Array<number>> => {
    return Array<number>(n).fill(null)
        .map((value: number, index: number) => Array<number>(index+1).fill(1));
}

describe('Pyramid Array', () => {
    it('gen array', () => {
        let testArray: Array<number> = [2,3,4];

        assert.deepStrictEqual(testArray.fill(1, 0, 3), [1, 1, 1]);
    });

    it('array fill', () => {
        let testArray = Array<number>(3).fill(null)
            .map((value: number, index: number) => {
                console.log('index: ', index);
                return Array<number>(index+1).fill(1);
            });

        console.log(testArray);
    });

    it('test case', () => {
        assert.deepStrictEqual(pyramid(0), []);
        assert.deepStrictEqual(pyramid(1), [[1]]);
        assert.deepStrictEqual(pyramid(2), [[1],[1,1]]);
        assert.deepStrictEqual(pyramid(3), [[1],[1,1],[1,1,1]]);
    });
});