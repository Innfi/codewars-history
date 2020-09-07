/*
https://www.codewars.com/kata/520446778469526ec0000001

Array.prototype.sameStructureAs = function (other) {
    return (this.length === other.length) ? this.every(function(el, i){
      return Array.isArray(el) ? el.sameStructureAs(other[i]) : true;
    }) : false;
};
*/ 

const assert = require('assert'); 

Array.prototype.sameStructureAs = function(other) {
    if(Array.isArray(other) === false) return false;
    if(this.length !== other.length) return false;

    for(let i=0; i<this.length; i++) {
        if(Array.isArray(this[i]) === true) {
            if(this[i].sameStructureAs(other[i]) === false) return false;
        } else {
            if(Array.isArray(other[i]) === true) return false;
        }
    }

    return true;
}

describe('Building blocks', () => {
    it('isArray', () => {
        assert.equal(Array.isArray([]), true);
    });

    it('element by index', () => {
        const testArray = [1, [2, 3]];

        assert.deepEqual(testArray[0], 1);
        assert.deepEqual(testArray[1], [2,3]);
    });
});

describe('Nesting structure', () => {
    it('simple case: not array', () => {
        assert.equal([1,1].sameStructureAs([2,2]), true);
    });

    it('simple case: 2d array 1', () => {
        assert.equal([[]].sameStructureAs([[]]), true);
        assert.equal([[]].sameStructureAs([2]), false);
        assert.equal([9].sameStructureAs([[]]), false);
    });

    it('simple case: 2d array 2', () => {
        assert.equal([[], []].sameStructureAs([[], []]), true);
        assert.equal([2, []].sameStructureAs([[], []]), false);
        assert.equal([[], []].sameStructureAs([[], 1]), false);
        assert.equal([[], []].sameStructureAs([3, 4, []]), false);
    });

    it('test cases', () => {
        assert.equal([1,1,1].sameStructureAs([2,2,2]), true);
        assert.equal([1,[1,1]].sameStructureAs([[2,2],2]), false);
        assert.equal([ [ [ ], [ ] ] ].sameStructureAs([ [ [ ], [ ] ] ]), true);
        assert.equal([ 1, [1,1] ].sameStructureAs([ 2,[2] ]), false);
    });
});