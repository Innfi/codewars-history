// https://www.codewars.com/kata/54edbc7200b811e956000556
package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func CountSheeps(numbers []bool) int {
	count := 0
	for _, n := range numbers {
		if n == true {
			count += 1
		}
	}
	return count
}

func TestCountingSheeps(t *testing.T) {
	assert.Equal(t, CountSheeps([]bool{true, false}), 0)
}
