/**
 * https://www.codewars.com/kata/521c2db8ddc89b9b7a0000c1
 */

const assert = require('assert'); 


const snail = (input) => {
    return [];
}

const right = (input, pos) => {
    return toRight(input, pos).concat(down(input, { x: pos.x, y: pos.y+1}));
}

const toRight = (input, pos) => {
    let result = [];

    while(true) {
        if(input[pos.y][pos.x] === 0) break;

        result.push(input[pos.y][pos.x]);
        input[pos.y][pos.x] = 0;

        if(pos.x+1 >= input.length) break;

        pos.x += 1;
    }

    return result;
}

const down = (input, pos) => {
    const result = toDown(input, pos);

    return result;
}

const toDown = (input, pos) => {
    let result = [];

    while(true) {
        if(input[pos.x][pos.y] === 0) break;

        result.push(input[pos.y][pos.x]);
        input[pos.y][pos.x] = 0;

        if(pos.y+1 >= input.length) break;
        pos.y += 1;
    }

    return result;
}

describe('Building blocks', () => {
    it('toRight', () => {
        let input = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let startPos = {
            x: 0, y: 0
        };

        const result = toRight(input, startPos);
        console.log('input: ', input);
        console.log('pos: ', startPos);
        
        assert.deepStrictEqual(result, [1,2,3]);
    });

    it('toDown', () => {
        let input = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let startPos = {
            x: 2, y: 1
        };

        assert.deepStrictEqual(toDown(input, startPos), [6,9]);
    });

    it('test right()', () => {
        let input = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let startPos = {
            x: 0, y: 0
        };

        assert.deepStrictEqual(right(input, startPos), [1,2,3,6,9]);
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