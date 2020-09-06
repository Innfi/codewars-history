/*
https://www.codewars.com/kata/520446778469526ec0000001


*/ 

const assert = require('assert'); 


Array.prototype.sameStructureAs = (other) => {

}


describe('Nesting structure', () => {
    it('test cases', () => {
        assert.equal([1,1,1].sameStructureAs([2,2,2]), true);
        assert.equal([1,[1,1]].sameStructureAs([[2,2],2]), false);
    });
});