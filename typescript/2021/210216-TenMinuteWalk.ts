/**
 * https://www.codewars.com/kata/54da539698b8a2ad76000228
 */

import assert from 'assert';


const isValidWalk = (walk: string[]): boolean => {
    if(walk.length !== 10) return false;
    let posX: number = 0;
    let posY: number = 0;

    walk.forEach((value: string) => {
        switch(value) {
            case 'n':  posY += 1; break;
            case 's':  posY -= 1; break;
            case 'w':  posX += 1; break;
            case 'e':  posX -= 1; break;
        }
    });

    return posX === 0 && posY === 0;
};

describe('Building blocks', () => {
    it('count for same element', () => {
        const input: string[] = ['1', '2', '3', '1'];

        assert.strictEqual(input.filter((value) => value ==='1').length, 2);
    });
});

describe('TenMinuteWalk', () => {
    it('test case', () => {
        assert.strictEqual(isValidWalk(['n','s','n','s','n','s','n','s','n','s']), true);
        assert.strictEqual(isValidWalk(['w','e','w','e','w','e','w','e','w','e','w','e']), false);
    });
});