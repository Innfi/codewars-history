/**
 *https://www.codewars.com/kata/5536a85b6ed4ee5a78000035
 */

import assert from 'assert';

/**
 * 
 * var friends1 = ["A1", "A2", "A3", "A4", "A5"];
        var fTowns1 = [["A1", "X1"], ["A2", "X2"], ["A3", "X3"], ["A4", "X4"]];
        var distTable1 = ["X1", 100.0, "X2", 200.0, "X3", 250.0, "X4", 300.0];
        dotest(friends1, fTowns1, distTable1, 889);
 * 
 */

const tour = (friends: string[], fTowns: any, distTable: any): number => {
    return 0; //FIXME
};

const missingLength = (lhs: number, rhs: number): number => {
    return Math.sqrt(Math.pow(lhs, 2) - Math.pow(rhs, 2));
};

describe('Building blocks', () => {
    it('gets missing length of a triangle', () => {
        const x1: number = 3;
        const x3: number = 5;

        assert.strictEqual(missingLength(5, 3), 4);
    });

    it('sample input', () => {
        const testResult = 100 + 
            missingLength(200, 100) + 
            missingLength(250, 200) + 
            missingLength(300, 250) + 300;
        assert.strictEqual(Math.floor(testResult), 889);
    });
});
