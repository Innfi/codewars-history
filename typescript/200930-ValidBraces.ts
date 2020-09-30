/*
https://www.codewars.com/kata/5277c8a221e209d3f6000b56
 */

import assert from 'assert';


const validBraces = (braces: string): boolean => {
    interface Dictionary<T> { [key: string]: T; }
    let braceDict: Dictionary<string> = {};
    braceDict[')'] = '(';
    braceDict[']'] = '[';
    braceDict['}'] = '{';

    let stack = [];

    for(const brace of braces.split('')) {
        if(isOpeningBrace(brace) === true) stack.push(brace);
        else {
            if(braceDict[brace] !== stack.pop()) return false;
        }
    }

    return stack.length === 0;
};

const isOpeningBrace = (brace: string): boolean => {
    if(brace === '(' || brace === '[' || brace === '{') return true;

    return false;
};

describe('Building blocks', () => {
    it('brace dicts', () => {
        interface Dictionary<T> { [key: string]: T; }
        let braceDict: Dictionary<string> = {};
        braceDict[')'] = '(';
        braceDict[']'] = '[';
        braceDict['}'] = '{';

        assert.strictEqual(braceDict[']'], '[');
    });

    it('opening braces', () => {
        assert.strictEqual(isOpeningBrace('{'), true);
    });
});

describe('Valid Braces', () => {
    it('test case', () => {
        assert.strictEqual(validBraces("()"), true); 
        assert.strictEqual(validBraces("[(])"), false); 
    });
});