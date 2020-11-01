/**
 * https://www.codewars.com/kata/56af1a20509ce5b9b000001e
 */

import assert from 'assert';


const travel = (r: string, zipCode: string): string => {
    let result = zipCode + ':';
    const addrs = r.split(',')
    .filter((addr) => new RegExp('\\d+.+\\s' + zipCode + '$').test(addr));

    if(zipCode === '') return result + '/';
    if(addrs.length === 0) return result + '/';

    addrs
    .map((addr, index, array) => {
        result += addr
        .replace(new RegExp('\\s' + zipCode), '')
        .replace(new RegExp('^\\d+\\s'), '');
        result += index+1 < array.length ? ',' : '/';
        return addr;
    })
    .map((addr, index, array) => {
        result += addr.match(new RegExp('^\\d+'))[0];
        result += index+1 < array.length ? ',' : '';
    });

    return result;
};

const ad = 
    "123 Main Street St. Louisville OH 43071," 
  + "432 Main Long Road St. Louisville OH 43071," 
  + "786 High Street Pollocksville NY 56432," 
  + "54 Holy Grail Street Niagara Town ZP 32908," 
  + "3200 Main Rd. Bern AE 56210,1 Gordon St. Atlanta RE 13000,"
  + "10 Pussy Cat Rd. Chicago EX 34342,10 Gordon St. Atlanta RE 13000,58 Gordon Road Atlanta RE 13000,"
  + "22 Tokyo Av. Tedmondville SW 43098,674 Paris bd. Abbeville AA 45521,10 Surta Alley Goodtown GG 30654,"
  + "45 Holy Grail Al. Niagara Town ZP 32908,320 Main Al. Bern AE 56210,14 Gordon Park Atlanta RE 13000,"
  + "100 Pussy Cat Rd. Chicago EX 34342,2 Gordon St. Atlanta RE 13000,5 Gordon Road Atlanta RE 13000,"
  + "2200 Tokyo Av. Tedmondville SW 43098,67 Paris St. Abbeville AA 45521,11 Surta Avenue Goodtown GG 30654,"
  + "45 Holy Grail Al. Niagara Town ZP 32918,320 Main Al. Bern AE 56215,14 Gordon Park Atlanta RE 13200,"
  + "100 Pussy Cat Rd. Chicago EX 34345,2 Gordon St. Atlanta RE 13222,5 Gordon Road Atlanta RE 13001,"
  + "2200 Tokyo Av. Tedmondville SW 43198,67 Paris St. Abbeville AA 45522,11 Surta Avenue Goodville GG 30655,"
  + "2222 Tokyo Av. Tedmondville SW 43198,670 Paris St. Abbeville AA 45522,114 Surta Avenue Goodville GG 30655,"
  + "2 Holy Grail Street Niagara Town ZP 32908,3 Main Rd. Bern AE 56210,77 Gordon St. Atlanta RE 13000";

describe('Building block', () => {
    it('string split', () => {
        const addrs = ad.split(',');

        assert.strictEqual(addrs[0], '123 Main Street St. Louisville OH 43071');
    });

    it('find with zip code', () => {
        const addrs = ad.split(',');
        for(const addr in addrs) {
            if(addr.match('EX 34342') != null) {
                assert.strictEqual(addr, '100 Pussy Cat Rd. Chicago EX 34342');
            }
        }
    });

    it('filter followed by map', () => {
        let input = 'SW 43199';
        let resultString = input + ':';

        ad.split(',')
        .filter((addr) => new RegExp('\\d+.+\\s' + input).test(addr))
        .map((addr, index, array) => {
            const another = addr
            .replace(new RegExp('\\s' + input), '')
            .replace(new RegExp('^\\d+\\s'), '');
            console.log('another: ', another);
            resultString += another;
            resultString += index+1 === array.length ? '/' : ','

            return addr;
        })
        .map((addr, index, array) => {
            resultString += addr.match(new RegExp('^\\d+'))[0];
            resultString += index+1 === array.length ? '' : ',';
        });

        console.log('resultString: ', resultString);
    });

    it('to rearranged string', () => {
        const testAddr = "123 Main Street St. Louisville OH 43071" ;
        const result = testAddr.replace(/123/,'');

        assert.strictEqual(result, " Main Street St. Louisville OH 43071");
    });
});

describe('Salemans travel', () => {
    it('tests valid zipcode', () => {
        assert.strictEqual(travel(ad, 'AA 45522'), 
            'AA 45522:Paris St. Abbeville,Paris St. Abbeville/67,670');
        assert.strictEqual(travel(ad, 'EX 34345'), 
            'EX 34345:Pussy Cat Rd. Chicago/100');
    });

    it('tests invalid cases', () => {
        assert.strictEqual(travel(ad, 'AA 45422'), 'AA 45422:/');
    });

    it('tests invalid cases 2', () => {
        const testAddrs = '123 Main Street St. Louisville OH 43071,' 
        + '432 Main Long Road St. Louisville OH 43071';

        assert.strictEqual(travel(testAddrs, 'OH 430'), 
            'OH 430:/');
    });
});