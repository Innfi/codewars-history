// https://www.codewars.com/kata/538835ae443aae6e03000547

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Add(input int) func(int) int {
	return func(i int) int {
		return input + i
	}
}

func TestAdd(t *testing.T) {
	assert.Equal(t, Add(3)(4), 7)
}
