/**
 *https://www.codewars.com/kata/55b3425df71c1201a800009c
 */

import assert from 'assert';


const stat = (strg: string): string => {
    if(strg === '') return '';

    const sortedRecords = strg.split(', ')
    .map(record => {
        return record.split('|').reverse()
        .map(x => Number.parseInt(x))
        .reduce((prev: number, curr: number, index: number) => {
            return prev + curr*Math.pow(60, index);
        });
    }).sort((a, b) => { return a-b; });

    const range = sortedRecords[sortedRecords.length-1] - sortedRecords[0];
    const average = Math.floor(
        sortedRecords.reduce((acc, curr) => acc+curr) / sortedRecords.length);
    const middleIndex = Math.floor(sortedRecords.length/2);
    const median = sortedRecords.length % 2 === 1 ? sortedRecords[middleIndex] : 
        Math.floor((sortedRecords[middleIndex] + sortedRecords[middleIndex-1]) / 2);

    return 'Range: ' + toRecordFormat(range) + 
           ' Average: ' + toRecordFormat(average) + 
           ' Median: ' + toRecordFormat(median);
};

const toRecordFormat = (input: number): string => {
    const hour = Math.floor(input / 3600);
    const minutes = Math.floor((input - hour*3600)/60);
    const seconds = input - (hour*3600) - (minutes*60);
    //return hour.toString().padStart(2, '0') + '|' + 
    //       minutes.toString().padStart(2, '0') + '|' + 
    //       seconds.toString().padStart(2, '0'); //ES7.string.padStart(). :)
    return addPadding(hour) + '|' + 
           addPadding(minutes) + '|' + 
           addPadding(seconds);
};

const addPadding = (input: number): string => {
    return input< 10 ? '0' + input.toString() 
        : input.toString();
};

describe('Building blocks', () => {
    it('tests split', () => {
        const input = '01|59|59';
        const result = input.split('|');
        assert.strictEqual(result[0], '01');
    });

    it('to seconds', () => {
        const input = '01|59|59';
        const result = input
        .split('|')
        .reverse()
        .map(x => Number.parseInt(x)).reduce(
            (prev: number, curr: number, index: number) => {
                return prev + curr * Math.pow(60, index);
            });

        assert.strictEqual(result, 3600 + (59*60) + 59);
    });

    it('to seconds array', () => {
        //const input = '02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|17|17, 2|22|00, 2|31|41';
        const input = '02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|17|17, 2|22|00';

        const test = input.split(', ')
        .map(record => {
            return record.split('|').reverse()
            .map(x => Number.parseInt(x))
            .reduce((prev: number, curr: number, index: number) => {
                return prev + curr * Math.pow(60, index);
            });
        }).sort((a, b) => { return a-b; });

        assert.strictEqual(test[0], 8159);
    });

    it('string padstart', () => {
        const testNumber = 5;
        assert.strictEqual(testNumber.toString().padStart(2, '0'), '05');
    });

    it('to record format', () => {
        const inputSeconds = 8520;
        assert.strictEqual(toRecordFormat(inputSeconds), '02|22|00');
    });
});

describe('RecordStat', () => {
    it('test case', () => {
        assert.strict(stat("01|15|59, 1|47|16, 01|17|20, 1|32|34, 2|17|17"), 
            "Range: 01|01|18 Average: 01|38|05 Median: 01|32|34");
        assert.strict(stat("02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|17|17, 2|22|00, 2|31|41"), 
            "Range: 00|31|17 Average: 02|26|18 Median: 02|22|00");
        assert.strict(stat("02|15|59, 2|47|16, 02|17|20, 2|32|34, 2|32|34, 2|17|17"), 
            "Range: 00|31|17 Average: 02|27|10 Median: 02|24|57");
    });
});