/**
https://www.codewars.com/kata/56a5d994ac971f1ac500003e
 */

import assert from 'assert';

const longest = (input: string[], k: number): string => {
    let out: string = '';

    if(k > input.length) return out;
    if(k < 0) return out;

    for(let i=0;i<input.length;i++) {
        let compound: string = input.slice(i,k+i).join('');
        console.log('str: ', compound);
        if(compound.length > out.length) out = compound;
    }

    return out;
};

describe('Building blocks', () => {
    const input: string[] = ["zone", "abigail", "theta", "form", "libe", "zas"];
    it('array slice', () => {
        assert.deepStrictEqual(input.slice(0, 2), ["zone", "abigail"]);
        assert.deepStrictEqual(input.slice(2, 4), ["theta", "form"]);
    });

    it('make consecutive string', () => {
        assert.strictEqual(input.slice(0, 3).join(''), 'zoneabigailtheta');
    });

    it('slice exceeding the length', () => {
        assert.strictEqual(input.slice(0, 9999).join(''), input.join(''));
    });

    it('slice with length 1', () => {
        assert.strictEqual(input.slice(0, 1).join(''), 'zone');
    });
});


describe('Consecutive strings', () => {
    it('test case', () => {
        assert.strictEqual(
            longest(["zone", "abigail", "theta", "form", "libe", "zas"], 2), "abigailtheta");
        assert.strictEqual(
            longest(["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", 
            "oocccffuucccjjjkkkjyyyeehh"], 1), "oocccffuucccjjjkkkjyyyeehh");
        assert.strictEqual(longest(["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 3), 
            "ixoyx3452zzzzzzzzzzzz");
    });
});
