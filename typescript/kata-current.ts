/**
 * https://www.codewars.com/kata/5269452810342858ec000951
 */

import assert from 'assert';


const isValidCoordinates = (coordinates: string): boolean => {
    const parsed = coordinates.split(',');
    const latitude = parseFloat(parsed[0]);
    if(latitude === NaN) return false;
    if(latitude < -90 || latitude > 90) return false;

    const longitude = parseFloat(parsed[1]);
    if(longitude === NaN) return false;
    if(longitude < -180 || longitude > 180) return false;

    return true;
}

describe('Building block', () => {
    const validCoords: string[] = [
      "-23, 25",
      "4, -3",
      "24.53525235, 23.45235",
      "04, -23.234235",
      "43.91343345, 143"
    ];

    it('splits input', () => {
        const coords = validCoords[0].trim();
        const result = coords.split(',');
        assert.strictEqual(parseFloat(result[0]), -23);
        assert.strictEqual(parseFloat(result[1]), 25);
    });

    it('parsefloat fails with alphabet letter', () => {
        assert.strictEqual(parseFloat('N23.43345'), NaN);
    });
});

describe('CoordValidator', () => {
    it('test with valid cases', () => {
        var ValidCoordinates = [
          "-23, 25",
          "4, -3",
          "24.53525235, 23.45235",
          "04, -23.234235",
          "43.91343345, 143"
        ];

        ValidCoordinates.forEach((coord) => {
            assert.strictEqual(isValidCoordinates(coord), true);
        });
    });

    it('test with invalid cases', () => {
       var InvalidCoordinates = [
          "23.234, - 23.4234",
          "2342.43536, 34.324236",
          "N23.43345, E32.6457",
          "99.234, 12.324",
          "6.325624, 43.34345.345",
          "0, 1,2",
          "0.342q0832, 1.2324",
          "23.245, 1e1"
        ];

        InvalidCoordinates.forEach((coord) => {
            assert.strictEqual(isValidCoordinates(coord), false);
        });
    });
});