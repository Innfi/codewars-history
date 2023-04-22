// https://www.codewars.com/kata/55f2b110f61eb01779000053

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

/*
func GetSum(a, b int) int {
  if a > b {
    a, b = b, a
  }
  return (a + b) * (b - a + 1) / 2
}
*/

func GetSum(a, b int) int {
	left := a
	right := b
	if left > right {
		left = b
		right = a
	}

	result := 0
	for i := left; i <= right; i++ {
		result += i
	}

	return result
}

func TestGetSum(t *testing.T) {
	assert.Equal(t, GetSum(2, -1), 2)
}
