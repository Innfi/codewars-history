/**
 * https://www.codewars.com/kata/545434090294935e7d0010ab
 */

import assert from 'assert';


// select
// from
// where
// orderBy
// groupBy
// having 
// execute

let persons = [{
        name: 'Peter',
        profession: 'teacher',
        age: 20,
        maritalStatus: 'married'
      },
      {
        name: 'Michael',
        profession: 'teacher',
        age: 50,
        maritalStatus: 'single'
      },
      {
        name: 'Peter',
        profession: 'teacher',
        age: 20,
        maritalStatus: 'married'
      },
      {
        name: 'Anna',
        profession: 'scientific',
        age: 20,
        maritalStatus: 'married'
      },
      {
        name: 'Rose',
        profession: 'scientific',
        age: 50,
        maritalStatus: 'married'
      },
      {
        name: 'Anna',
        profession: 'scientific',
        age: 20,
        maritalStatus: 'single'
      },
      {
        name: 'Anna',
        profession: 'politician',
        age: 50,
        maritalStatus: 'married'
      }
    ];

const query = () => {
    const select = (arg?: any) => {
        return this;
    };
};



describe('Building blocks', () => {
    it('basic select', () => {
        let input: number[] = [1, 2, 3];

        assert.deepStrictEqual(query().select().from(input).execute(), [1, 2, 3]);
        assert.deepStrictEqual(query().from(input).select().execute(), [1, 2, 3]);
    });
});
