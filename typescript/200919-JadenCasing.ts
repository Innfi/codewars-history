/**
 * https://www.codewars.com/kata/5390bac347d09b7da40006f6
*/

import assert from 'assert';

declare global {
    interface String {
        toJadenCase(): void;
    }
}

String.prototype.toJadenCase = function (): String {
    return this.split(' ').map(function(x: String) {
        return x.charAt(0).toUpperCase() + x.slice(1);
    }).reduce(function(a: String, b: String) {
        return a + ' ' + b;
    });
};

describe('buildig blocks', () => {
    const str: String = "How can mirrors be real if our eyes aren't real";
    it('split string', () => {
       const result = str.split(' ');

       assert.strictEqual(result[0], "How");
    });

    it('test map', () => {
       const result = str.split(' ');

       const capResult = result.map(x => {
           return x.charAt(0).toUpperCase() + x.slice(1);
       });

       console.log('capResult', capResult);
    });
});

describe('JadenCasing', () => {
   it('test case', () => {
       const str: String = "How can mirrors be real if our eyes aren't real";

       assert.strictEqual(str.toJadenCase(), "How Can Mirrors Be Real If Our Eyes Aren't Real");
   });
});