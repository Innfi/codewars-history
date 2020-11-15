/**
 * https://www.codewars.com/kata/52685f7382004e774f0001f7
 */

import assert from 'assert';


const humanReadableTime = (seconds: number): string => {
    const hour = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds%3600) / 60);
    const remainSeconds = seconds - (hour*3600) - (minutes*60);

    return hour.toString().padStart(2, '0') + ':' + 
           minutes.toString().padStart(2, '0') + ':' + 
           remainSeconds.toString().padStart(2, '0');
};

describe('HumanReadableTime', () => {
    it('test case', () => {
        assert.strictEqual(humanReadableTime(359999), '99:59:59');
    });
});