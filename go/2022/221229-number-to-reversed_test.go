// https://www.codewars.com/kata/5583090cbe83f4fd8c000051

package codewars_history

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Digitize(n int) []int {
	asString := fmt.Sprintf("%v", n)

	result := []int{}

	for i := 1; i <= len(asString); i++ {
		result = append(result, int(asString[len(asString)-i])-48)
	}

	return result
}

func TestDigitize(t *testing.T) {
	assert.Equal(t, Digitize(35231), []int{1, 3, 2, 5, 3})
	assert.Equal(t, Digitize(0), []int{0})
}
