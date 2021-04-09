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

const toMatrix = (input: string): string[][] => {
    return input.split('\n').map((value: string) => value.split(''));
};

const doReverse = (input: string[][]): string[][] => {
    return input.map((value: string[]) => value.reverse());
};

const doTranspose = (input: string[][]): string[][] => {
    return Object.keys(input[0]).map((column: string) => {
        return input.map((row: string[]) => row[column]);
    });
};

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

    it('array reverse', () => {
        assert.deepStrictEqual([1, 2, 3].reverse(), [3, 2, 1]);

        const input: string[][] = [['a','b','c'], ['d','e','f'], ['g','h','i']];

        const output: string[][] = input.map((value: string[]) => value.reverse());
        assert.deepStrictEqual(output, [['c','b','a'],['f','e','d'],['i','h','g']]);
    });

    it('array transpose', () => {
        const input: number[][] = [[1,2], [3,4]];
        const output: number[][] = Object.keys(input[0]).map((column: string) => {
            return input.map((row: number[]) => row[column]);
        });

        assert.deepStrictEqual(output, [[1,3],[2,4]]);
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