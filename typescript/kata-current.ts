/**
 * https://www.codewars.com/kata/52b7ed099cdc285c300001cd
 */

import assert from 'assert';


class Interval {
    start: number = 0;
    end: number = 0;

    constructor(start: number, end: number) {
        this.start = start;
        this.end = end;
    } 

    isOverlapping(start: number, end: number) {
        if(this.start <= start && end <= this.end) return true;
        if(start <= this.start && this.end <= end) return true;
        if(start <= this.start && this.start <= end) return true;
        if(start <= this.end && this.end <= end) return true;

        return false;
    }

    overlap(start: number, end: number) {
        this.start = this.start < start ? this.start : start;
        this.end = this.end > end ? this.end : end;
    }
};

const sumOfIntervals = (intervals: number[][]): number => {
    let filteredIntervals: Interval[] = [];

    intervals.forEach((value: number[]) => {
        let isOverlapped: boolean = false;
        for(let i=0;i<filteredIntervals.length;i++) {                
            if(filteredIntervals[i].isOverlapping(value[0], value[1])) {
                filteredIntervals[i].overlap(value[0], value[1]);
                isOverlapped = true;
                break;
            }
        }
        if(!isOverlapped) filteredIntervals.push(
            new Interval(value[0], value[1]));
    });
    return filteredIntervals
    .map((element: Interval) => { return element.end - element.start; })
    .reduce((prev: number, current: number) => prev + current);
};


describe('Building blocks', () => {
    it('overlapping case: subset', () => {
        const interval = new Interval(1, 100);

        assert.strictEqual(interval.isOverlapping(2, 10), true);
        interval.overlap(2, 10);

        assert.strictEqual(interval.start, 1);
        assert.strictEqual(interval.end, 100);
    });

    it('overlapping case: superset', () => {
        const interval = new Interval(5, 10);

        assert.strictEqual(interval.isOverlapping(1, 50), true);
        interval.overlap(1, 50);

        assert.strictEqual(interval.start, 1);
        assert.strictEqual(interval.end, 50);
    });

    it('overlapping case: left intersection', () => {
        const interval = new Interval(5, 10);

        assert.strictEqual(interval.isOverlapping(1, 7), true);
        interval.overlap(1, 7);

        assert.strictEqual(interval.start, 1);
        assert.strictEqual(interval.end, 10);
    });

    it('overlapping case: right intersection', () => {
        const interval = new Interval(5, 10);

        assert.strictEqual(interval.isOverlapping(9, 15), true);
        interval.overlap(9, 15);

        assert.strictEqual(interval.start, 5);
        assert.strictEqual(interval.end, 15);
    });

    it('overlapping case: seperated intervals', () => {
        const interval = new Interval(1, 5);
        assert.strictEqual(interval.isOverlapping(9, 15), false);
    });
});

describe('SumOfIntervals', () => {
    it('simple case', () => {
        assert.strictEqual(sumOfIntervals([[1, 5]]), 4);
    });

    it('case 1', () => {
        assert.strictEqual(sumOfIntervals([[1, 5], [10, 15], [-1, 3]]), 11);
    });

    it('case many', () => {
        const many: number[][] = [
            [-401 , 286],
            [-33 , 269],
            [-407 , -272],
            [-229 , -110],
            [-238 , 223],
            [-278 , -28],
            [124 , 403],
            [88 , 243],
            [-211 , 469],
            [237 , 343],
            [279 , 459],
            [-432 , 116],
            [260 , 323],
            [257 , 372],
            [472 , 475],
            [-28 , -23],
            [-331 , 244],
            [337 , 500]
        ];

        assert.strictEqual(sumOfIntervals(many), 932);
    });
});