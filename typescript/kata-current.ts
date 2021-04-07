/**
 * https://www.codewars.com/kata/56dbf59b0a10feb08c000227
 */

import assert from 'assert';


function rot90Counter(input: string): string {
    return "JANEmw\nXchZpg\ncgsCqC\nGyIrEx\ncagOBk\nEaNyeN";
}

function diag2Sym(input: string): string {
    return '';
}

function selfieDiag2Counterclock(input: string): string {
    return '';
}

function oper(fct: (s: string) => string, s: string): string {
    return fct(s);
}

describe('Building blocks', () => {
    it('split input into arrays', () => {
        const input: string = 'EcGcXJ\naaygcA\nNgIshN\nyOrCZE\neBEqpm\nNkxCgw';
        assert.strictEqual(input.split('\n')[0], 'EcGcXJ');
    });

    it('oneline matrix', () => {
        const input: string = 'EcGcXJ\naaygcA\nNgIshN\nyOrCZE\neBEqpm\nNkxCgw';
        
    });

    it('array handling', () => {
        const testInput: string[][] = [['a','b'], ['c','d']];
        const testOutput: string[][] = [];
        const newRow: string[] = [];
        newRow.push(testInput[0][0]);
        newRow.push(testInput[1][0]);
        testOutput.push(newRow);
        const newRowNext: string[] = [];
        newRowNext.push(testInput[0][1]);
        newRowNext.push(testInput[1][1]);
        testOutput.push(newRowNext);

        assert.deepStrictEqual(testOutput, [['a','c'], ['b','d']]);
    });
});

describe('Moves in squared string', () => {
    it('test case', () => {
       assert.strictEqual(
          oper(rot90Counter, "EcGcXJ\naaygcA\nNgIshN\nyOrCZE\neBEqpm\nNkxCgw"),
          "JANEmw\nXchZpg\ncgsCqC\nGyIrEx\ncagOBk\nEaNyeN"
        );
    });
});