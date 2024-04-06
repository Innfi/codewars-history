// https://www.codewars.com/kata/59d0ee709f0cbcf65400003b

import assert from 'assert';

describe('test', () => {
  function byState(str: string): string {
    const stateMap: {[key: string]: string} = {
      'AZ': 'Arizona',
      'CA': 'California',
      'ID': 'Idaho',
      'IN': 'Indiana',
      'MA': 'Massachusetts',
      'OK': 'Oklahoma',
      'PA': 'Pennsylvania',
      'VA': 'Virginia'
    };

    const addrMap: {[key: string]: string[]} = {};

    str.split('\n').forEach((v) => {
      const state = v.substring(v.length - 2);
      if (!addrMap[state]) {
        addrMap[state] = [];
      }

      addrMap[state].push(v);
    });

    let result = '';
    Object.keys(stateMap).forEach((v) => {
      if (!addrMap[v]) {
        return;
      }

      if (result.length > 0) {
        result += ' ' + stateMap[v] + '\r\n';
      } else {
        result += stateMap[v] + '\r\n';
      }
    
      addrMap[v].sort((a, b) => a.split(',')[0].localeCompare(b.split(',')[0]));

      addrMap[v].forEach((addr) => {
        result += '..... ' + 
          addr.substring(0, addr.length - 3).split(',').join('') + 
          ' ' + stateMap[v] + '\r\n';
      });
    });

    return result.substring(0, result.length - 2);
  }

  it ('check substring', () => {
    const input = 'John Daggett, 341 King Road, Plymouth MA';

    assert.strictEqual(input.substring(input.length - 2), 'MA');
  });

  it ('case 1', () => {
    const input = `John Daggett, 341 King Road, Plymouth MA
Alice Ford, 22 East Broadway, Richmond VA
Sal Carpenter, 73 6th Street, Boston MA`;
    const expected = "Massachusetts\r\n..... John Daggett 341 King Road Plymouth Massachusetts\r\n..... Sal Carpenter 73 6th Street Boston Massachusetts\r\n Virginia\r\n..... Alice Ford 22 East Broadway Richmond Virginia"

    assert.strictEqual(byState(input), expected);
  });

  it ('case 2', () => {
    const input = `John Daggett, 341 King Road, Plymouth MA
Alice Ford, 22 East Broadway, Richmond VA
Orville Thomas, 11345 Oak Bridge Road, Tulsa OK
Terry Kalkas, 402 Lans Road, Beaver Falls PA
Eric Adams, 20 Post Road, Sudbury MA
Hubert Sims, 328A Brook Road, Roanoke MA
Amy Wilde, 334 Bayshore Pkwy, Mountain View CA
Sal Carpenter, 73 6th Street, Boston MA`;

    const expected = "California\r\n..... Amy Wilde 334 Bayshore Pkwy Mountain View California\r\n Massachusetts\r\n..... Eric Adams 20 Post Road Sudbury Massachusetts\r\n..... Hubert Sims 328A Brook Road Roanoke Massachusetts\r\n..... John Daggett 341 King Road Plymouth Massachusetts\r\n..... Sal Carpenter 73 6th Street Boston Massachusetts\r\n Oklahoma\r\n..... Orville Thomas 11345 Oak Bridge Road Tulsa Oklahoma\r\n Pennsylvania\r\n..... Terry Kalkas 402 Lans Road Beaver Falls Pennsylvania\r\n Virginia\r\n..... Alice Ford 22 East Broadway Richmond Virginia";

    assert.strictEqual(byState(input), expected);
  });

  it ('case 3', () => {
    const input = `
Ama Zon, 328A Brook Road, Beaver Falls PA
Chris Maker, 8A River Street, Westbury AZ
Paul Cartney, 213 Cap Bono, Hill View IN
Mac Bud, 22 Prin Broadway, Plymouth MA
Raf Johnson, 11345 Oak Bridge Road, Beaver Falls PA
Chris Maker, 8A River Street, Hill View IN
Ama Zon, 321 King Street, Westbury AZ
Bill Joke, 1C Hilary Main Street, Mountain View CA
Antony None, 5AA Clear Bd, Richmond ID
`;

    console.log(byState(input));
    console.log('----------');

    const input2 = `
Richard Stall, 90 Pen Avenue, Richmond ID
John Deere, 45 Bridge Port, Mountain View CA
Burt Lane, 328A Brook Road, Mountain View CA
Paul Pep, 213 Cap Bono, Hill View IN
Laurel Tango, 328A Brook Road, Mountain View CA
Antony None, 11354 East Bridge Road, Plymouth MA
John Deere, 11354 East Bridge Road, Mountain View CA
Louis Tor, 5AA Clear Bd, Plymouth MA
Richard Stall, 11345 Oak Bridge Road, Richmond ID
`;
    console.log(byState(input2));
  });
});

/**
 * California\r\n
 * ..... Amy Wilde 334 Bayshore Pkwy Mountain View California\r\n
 *  Massachusetts\r\n
 * ..... Eric Adams 20 Post Road Sudbury Massachusetts\r\n
 * ..... Hubert Sims 328A Brook Road Roanoke Massachusetts\r\n
 * ..... John Daggett 341 King Road Plymouth Massachusetts\r\n
 * ..... Sal Carpenter 73 6th Street Boston Massachusetts\r\n
 *  Oklahoma\r\n
 * ..... Orville Thomas 11345 Oak Bridge Road Tulsa Oklahoma\r\n
 *  Pennsylvania\r\n
 * ..... Terry Kalkas 402 Lans Road Beaver Falls Pennsylvania\r\n
 *  Virginia\r\n
 * ..... Alice Ford 22 East Broadway Richmond Virginia
 * 
 */
