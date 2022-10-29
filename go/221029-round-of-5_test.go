// https://www.codewars.com/kata/55d1d6d5955ec6365400006d

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func RoundToNext5(n int) int {
	if n%5 == 0 {
		return n
	}

	for i := 1; i < 5; i++ {
		if (n+i)%5 == 0 {
			return n + i
		}
	}

	return n
}

func TestDiv(t *testing.T) {
	assert.Equal(t, -7%5, -2)
	assert.Equal(t, 21%5, 1)
	assert.Equal(t, -1%5, -1)
	assert.Equal(t, 3%5, 3)

	var div int = 21 / 5
	assert.Equal(t, div, 4)

	var div2 int = -1 / 5
	assert.Equal(t, div2, 0)
}

func TestPageDigits(t *testing.T) {
	assert.Equal(t, RoundToNext5(2), 5)
	assert.Equal(t, RoundToNext5(12), 15)
	assert.Equal(t, RoundToNext5(21), 25)
	assert.Equal(t, RoundToNext5(30), 30)
	assert.Equal(t, RoundToNext5(-1), 0)
}
