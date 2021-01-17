/**
https://www.codewars.com/kata/54de3257f565801d96001200


one of the optimal solutions: 

export class G964 {
    public static balanceStatements = (orders) => {
      if (orders.length == 0) return 'Buy: 0 Sell: 0';
      let buy = 0, sell = 0, malformed = [];
      for (let order of orders.split(', ')) {
        let [quote, quantity, price, status] = order.trim().split(' ');
        if (quote === undefined || quantity === undefined || price === undefined || status === undefined || quantity.toString().includes('.') || !price.toString().includes('.')) {
          malformed.push(order);
        } else if (status == 'B') {
          buy += Math.round(quantity * price);
        } else if (status == 'S') {
          sell += Math.round(quantity * price);
        }
      }
      return `Buy: ${buy} Sell: ${sell}${malformed.length > 0 ? `; Badly formed ${malformed.length}: ${malformed.join(' ;')} ;` : ''}`;
    }
}
 */

import assert from 'assert';

const balanceStatements = (input: string): string => {
    const regex = RegExp('^.+\\s\\d+\\s(\\d+)?\\.\\d+\\s[SB]$');

    let buy: number = 0;
    let sell: number = 0;
    let invalidOrder: string[] = [];
    input.split(', ')
    .forEach((value: string) => {
        if(!regex.test(value)) {
            invalidOrder.push(value + ' ;');
        } else {
            const parsed: string[] = value.split(' ');
            const delta: number = 
                Number.parseInt(parsed[1]) * Number.parseFloat(parsed[2]);

            if(parsed[3] === 'B') buy += delta;
            else sell += delta;
        }
    })

    return 'Buy: ' + Math.floor(buy) + ' Sell: ' + Math.ceil(sell) + ';' + 
        (invalidOrder.length >= 1 ? 
        ' Badly formed ' + invalidOrder.length + ': ' + invalidOrder.join('')
        : '');
};

describe('Building blocks', () => {
    it('check input sanity ', () => {
        const regex = RegExp('^[A-Z]+\\s\\d+\\s(\\d+)?\\.\\d+\\s[SB]$');

        const input: string = 'GOOG 300 542.0 B';
        assert.strictEqual(regex.test(input), true);

        const invalid: string = 'GOOG 300.0 12 B';
        assert.strictEqual(regex.test(invalid), false);

        const valid2: string = 'APPL 200 123.5 S';
        assert.strictEqual(regex.test(valid2), true);

        const invalid2: string = 'GOOG 300 12.0 O';
        assert.strictEqual(regex.test(invalid2), false);
    });

    it('parse weird letters', () => {
        assert.strictEqual(Number.parseFloat('.2'), 0.2);
    });

    it('parse multiple orders', () => {
        const input: string = "GOOG 300 542.0 B, AAPL 50 145.0 B, CSCO 250.0 29 B, GOOG 200 580.0 S";
        const regex = RegExp('^[A-Z]+\\s\\d+\\s\\d+\\.\\d+\\s[SB]$');

        let buy: number = 0;
        let sell: number = 0;
        let invalidOrder: string[] = [];
        input.split(', ')
        .forEach((value: string) => {
            if(!regex.test(value)) invalidOrder.push(value + ' ;');
            else {
                const parsed: string[] = value.split(' ');
                const delta: number = 
                    Number.parseInt(parsed[1]) * Number.parseInt(parsed[2]);

                if(parsed[3] === 'B') buy += delta;
                else sell += delta;
            }
        })

        let result: string = 'Buy: ' + Math.round(buy) + 
            ' Sell: ' + Math.round(sell) + ';' + 
            (invalidOrder.length >= 1 ? 
                ' Badly formed ' + invalidOrder.length + ': ' + invalidOrder.join()
             : '');

        assert.strictEqual(result, 'Buy: 169850 Sell: 116000; Badly formed 1: CSCO 250.0 29 B ;');
    });
});

describe('BalanceStatements', () => {
    it('test case', () => {
        const input: string ="GOOG 90 160.45 B, JPMC 67 12.8 S, MYSPACE 24.0 210 B, CITI 50 450 B, CSCO 100 55.5 S";
        const result: string = "Buy: 14440 Sell: 6408; Badly formed 2: MYSPACE 24.0 210 B ;CITI 50 450 B ;";
        assert.strictEqual(balanceStatements(input), result);

        const input2: string = "ZNGA 1300 2.66, CLH15.NYM 50 56.32 S, OWW 1000 11.623 S, OGG 20 580.1 S";
        const result2: string = "Buy: 0 Sell: 26041; Badly formed 1: ZNGA 1300 2.66 ;";
        assert.strictEqual(balanceStatements(input2), result2);

        const input3: string = 'CAP 1300 .2 B, CLH16.NYM 50 56 S, OWW 1000 11 S, OGG 20 580.1 S';
        const result3: string = 'Buy: 260 Sell: 11602; Badly formed 2: CLH16.NYM 50 56 S ;OWW 1000 11 S ;'
        assert.strictEqual(balanceStatements(input3), result3);
    });
});