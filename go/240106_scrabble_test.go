// https://www.codewars.com/kata/558fa34727c2d274c10000ae

package codewars_history

import "strings"

func ScrabbleScore(st string) int {
	sum := 0
	for _, elem := range strings.ToUpper(st) {
		sum += DictScores[string(elem)]
	}

	return sum
}
