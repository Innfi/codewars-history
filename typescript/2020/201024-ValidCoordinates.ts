/**
 * https://www.codewars.com/kata/5269452810342858ec000951
 * 
 * optimal solutions: 
 * 
 * export function isValidCoordinates(coordinates:string):boolean{
  const pattern = /^-?(90|[0-8]\d|\d)(?:\.\d*)?,\s-?(1[0-7]\d|180|\d{1,2})(?:\.\d*)?$/
  return pattern.test(coordinates);
}

  export function isValidCoordinates(coordinates: string): boolean {
    const [lat, lng] = coordinates.split(', ').map(Number);
    return lat > -90 && lat < 90 && lng > -180 && lng < 180;
}
 */

import assert from 'assert';


const isValidCoordinates = (coordinates: string): boolean => {
    const regExp = '^\\-?\\d+(\\.\\d+)?,\\s?(\\-?\\d+(\\.\\d+)?)$';
    const result = coordinates.match(regExp);
    if(result === null) return false;
    if(result[0] !== coordinates) return false;

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
    const regExp = '^\\-?\\d+(\\.\\d+)?,\\s?(\\-?\\d+(\\.\\d+)?)$';
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

    it('regular expression with valid case', () => {
        const testString = '24.535, 23.455';
        const result = testString.match(regExp);
        console.log('result: ', result);
        
        assert.strictEqual(result[0], testString);
    });

    it('regular expression with invalid case', () => {
        const testString = 'N20.535, 23.455';

        assert.strictEqual(testString.match(regExp) === null, true);
    });

    it('regular expression with minus', () => {
        const testString = '-10.0, -20.0';

        assert.strictEqual(testString.match(regExp)[0], testString);
    });

    it('regular expression without dots', () => {
        const testString = '10, 20';

        assert.strictEqual(testString.match(regExp)[0], testString);
    });

    it('regular expression edge case', () => {
        const testString = '1, 2,3';
        const result = testString.match(regExp);

        assert.strictEqual(result === null, true);
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