/**
 * https://www.codewars.com/kata/529bf0e9bdf7657179000008
*/

const assert = require('assert');

const validSolution = (board) => {
    const startPos = [
        [0,0],
        [0,3],
        [0,6],
        [3,0],
        [3,3],
        [6,6],
        [6,0],
        [6,3],
        [6,6],
    ];

    for(let i=0;i<startPos.length;i++) {
        if(subsetSum(board, startPos[i]) !== 45) return false;
    }

    return true;
};

const subsetSum = (board, centerIndex) => {
    const posX = centerIndex[0];
    const posY = centerIndex[1];

    let sum = 0;
    for(let i=posY; i < posY+3; i++) {
        const row = board[i];
        sum += row.slice(posX, posX+3).reduce((a,b) => a+b, 0);
    }

    return sum;
};

describe('Building Blocks', () => {
    it('simple board', () => {
        const testBoard = [
            [1,2,3],
            [4,5,6],
            [7,8,9]
        ];

        assert.equal(testBoard[1][1], 5);
    });   

    it('array slice', () => {
        const testBoard = [
            [1,2,3],
            [4,5,6],
            [7,8,9]
        ];

        assert.deepEqual(testBoard[0].slice(0, 2), [1,2]);
    });

    it('array reduce', () => {
        const testArray = [1,2,3];

        assert.equal(testArray.reduce((a, b) => a+b, 0), 6);
    });

    it('sums number from board subset', () => {
        const testBoard = [
            [1,2,3],
            [4,5,6],
            [7,8,9]
        ];
        assert.equal(subsetSum(testBoard, [0, 0]), 45);

        const invalidBoard = [
            [9,2,3],
            [4,5,6],
            [7,8,9]
        ];
        assert.equal(subsetSum(invalidBoard, [0, 0]), 53);
    });

    it('position array', () => {
        const testPos = [
            [0,0],
            [0,3],
            [0,6],
            [3,0],
            [3,3],
            [6,6],
            [6,0],
            [6,3],
            [6,6],
        ];

        assert.deepEqual(testPos[2], [0,6]);
    });
});

describe('Sudoku', () => {
    it('runs input validation', () => {
        assert.equal(validSolution([
            [5, 3, 4, 6, 7, 8, 9, 1, 2], 
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9]
        ]), true);

        assert.equal(validSolution([
            [5, 3, 4, 6, 7, 8, 9, 1, 2], 
            [6, 7, 2, 1, 9, 0, 3, 4, 8],
            [1, 0, 0, 3, 4, 2, 5, 6, 0],
            [8, 5, 9, 7, 6, 1, 0, 2, 0],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 0, 1, 5, 3, 7, 2, 1, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 0, 0, 4, 8, 1, 1, 7, 9]
        ]), false);
    });
});