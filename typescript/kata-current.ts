/**
https://www.codewars.com/kata/56a5d994ac971f1ac500003e
 */

import assert from 'assert';

const longest = (input: string[], k: number): string => {
    return "";
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
});


describe('Consecutive strings', () => {
    //it('test case', () => {
    //    assert.strictEqual(
    //        longest(["zone", "abigail", "theta", "form", "libe", "zas"], 2), "abigailtheta");
    //    assert.strictEqual(
    //        longest(["ejjjjmmtthh", "zxxuueeg", "aanlljrrrxx", "dqqqaaabbb", 
    //        "oocccffuucccjjjkkkjyyyeehh"], 1), "oocccffuucccjjjkkkjyyyeehh");
    //    assert.strictEqual(longest(["it","wkppv","ixoyx", "3452", "zzzzzzzzzzzz"], 3), 
    //        "ixoyx3452zzzzzzzzzzzz");
    //});
});