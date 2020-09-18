/**
 * https://www.codewars.com/kata/521c2db8ddc89b9b7a0000c1
 * 
 * one of the optimal solutions was: 
 * 
 * 
 * snail = function(array) {
  var result;
  while (array.length) {
    // Steal the first row.
    result = (result ? result.concat(array.shift()) : array.shift());
    // Steal the right items.
    for (var i = 0; i < array.length; i++)
      result.push(array[i].pop());
    // Steal the bottom row.
    result = result.concat((array.pop() || []).reverse());
    // Steal the left items.
    for (var i = array.length - 1; i >= 0; i--)
      result.push(array[i].shift());
  }
  return result;
}
 */

const assert = require('assert'); 


const snail = (input) => {
    if(input[0].length === 0) return [];
    if(input[0].length === 1) return input[0];

    console.log('test: ', input.length);

    return input;

    return right(input, { x: 0, y: 0});
}

const right = (input, pos) => {
    const result = toRight(input, pos);
    if(result.length === 0) return [];

    return result.concat(down(input, { x: pos.x, y: pos.y+1 }));
}

const toRight = (input, pos) => {
    let result = [];

    while(true) {
        if(input[pos.y][pos.x] === 0) break;

        result.push(input[pos.y][pos.x]);
        input[pos.y][pos.x] = 0;

        if(pos.x+1 >= input.length) break;
        if(input[pos.y][pos.x+1] === 0) break;

        pos.x += 1;
    }

    return result;
}

const down = (input, pos) => {
    const result = toDown(input, pos);
    if(result.length === 0) return [];

    return result.concat(left(input, { x: pos.x-1, y: pos.y }));
}

const toDown = (input, pos) => {
    let result = [];

    while(true) {
        if(input[pos.y][pos.x] === 0) break;

        result.push(input[pos.y][pos.x]);
        input[pos.y][pos.x] = 0;

        if(pos.y+1 >= input.length) break;
        if(input[pos.y+1][pos.x] === 0) break;
        pos.y += 1;
    }

    return result;
}

const left = (input, pos) => {
    const result = toLeft(input, pos);
    if(result.length === 0) return [];

    return result.concat(up(input, { x: pos.x, y: pos.y-1 }));
}

const toLeft = (input, pos) => {
    let result = [];

    while(true) {
        if(input[pos.y][pos.x] === 0) break;

        result.push(input[pos.y][pos.x]);
        input[pos.y][pos.x] = 0;

        if(pos.x-1 < 0) break;
        if(input[pos.y][pos.x-1] === 0) break;
        pos.x -= 1;
    }

    return result;
}

const up = (input, pos) => {
    const result = toUp(input, pos);
    if(result.length === 0) return [];

    return result.concat(right(input, { x: pos.x+1, y: pos.y }));
}

const toUp = (input, pos) => {
    let result = [];

    while(true) {
        if(input[pos.y][pos.x] === 0) break;

        result.push(input[pos.y][pos.x]);
        input[pos.y][pos.x] = 0;

        if(pos.y-1 < 0) break;
        if(input[pos.y-1][pos.x] === 0) break;
        pos.y -= 1;
    }

    return result;
}

/*
describe('Building blocks', () => {
    it('toRight', () => {
        let input = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let startPos = {
            x: 0, y: 0
        };

        const result = toRight(input, startPos);

        assert.deepStrictEqual(result, [1,2,3]);
    });

    it('toDown', () => {
        let input = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let startPos = {
            x: 2, y: 1
        };

        assert.deepStrictEqual(toDown(input, startPos), [6,9]);
    });

    it('toLeft()', () => {
        let input = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let startPos = {
            x: 1, y: 2
        };

        assert.deepStrictEqual(toLeft(input, startPos), [8, 7]);
    });

    it('toUp()', () => {
        let input = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let startPos = {
            x: 0, y: 1
        };

        assert.deepStrictEqual(toUp(input, startPos), [4, 1]);
    });
});
*/

describe('Snail sort', () => {
    //it('test case', () => {
    //    const result = snail([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);

    //    assert.deepStrictEqual(result, [1, 2, 3, 6, 9, 8, 7, 4, 5]);
    //});

    //it('test case extended', () => {
    //    assert.deepStrictEqual(snail([[1, 2, 3, 4, 5], 
    //                            [6, 7, 8, 9, 10], 
    //                            [11, 12, 13, 14, 15], 
    //                            [16, 17, 18, 19, 20], 
    //                            [21, 22, 23, 24, 25]]), 
    //                        [1, 2, 3, 4, 5, 10, 15, 20, 25, 24, 23, 22, 21, 16, 11, 6, 7, 
    //                            8, 9, 14, 19, 18, 17, 12, 13]);
    //    assert.deepStrictEqual(snail([[1, 2, 3, 4, 5, 6], 
    //                            [20, 21, 22, 23, 24, 7], 
    //                            [19, 32, 33, 34, 25, 8], 
    //                            [18, 31, 36, 35, 26, 9], 
    //                            [17, 30, 29, 28, 27, 10], 
    //                            [16, 15, 14, 13, 12, 11]]), 
    //                        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 
    //                            19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 
    //                            34, 35, 36]);
    //});

    it('test edge case', () => {
        const test = [[]];
        assert.deepStrictEqual(test[0] === [], true);

        //assert.deepStrictEqual(snail([[]]), []);
        //assert.deepStrictEqual(snail([[1]]), [1]);
    });
});
