/**
 * https://www.codewars.com/kata/521c2db8ddc89b9b7a0000c1
 */

const assert = require('assert'); 


const snail = (input) => {
    let result = [input[0][0]];
    input[0][0] = 0;

    console.log(input);

    return result.concat(right(input, [0,0])); 
}

const right = (input, pos) => {
    const result = toRight(input, [pos[0]+1, pos[1]]);

    return result;
}

const toRight = (input, pos) => {
    let result = [];

    let posX = pos[0];
    let posY = pos[1];
    while(true) {
        if(posX >= input.length) break;
        if(input[posX][posY] === 0) break;

        result.push(input[posY][posX]);
        input[posY][posX] = 0;
        posX += 1;
    }

    return result;
}

describe('Building blocks', () => {
    it('toRight', () => {
        let input = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        assert.deepEqual(toRight(input, [1,0]), [2,3]);
    });
});

/*
describe('Snail sort', () => {
    it('test case', () => {
        assert.deepEqual(snail([[1, 2, 3], [4, 5, 6], [7, 8, 9]]), [1, 2, 3, 6, 9, 8, 7, 4, 5]);
        assert.deepEqual(snail([[1, 2, 3, 4, 5], 
                                [6, 7, 8, 9, 10], 
                                [11, 12, 13, 14, 15], 
                                [16, 17, 18, 19, 20], 
                                [21, 22, 23, 24, 25]]), 
                            [1, 2, 3, 4, 5, 10, 15, 20, 25, 24, 23, 22, 21, 16, 11, 6, 7, 
                                8, 9, 14, 19, 18, 17, 12, 13]);
        assert.deepEqual(snail([[1, 2, 3, 4, 5, 6], 
                                [20, 21, 22, 23, 24, 7], 
                                [19, 32, 33, 34, 25, 8], 
                                [18, 31, 36, 35, 26, 9], 
                                [17, 30, 29, 28, 27, 10], 
                                [16, 15, 14, 13, 12, 11]]), 
                            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 
                                19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 
                                34, 35, 36]);
    });
});
*/