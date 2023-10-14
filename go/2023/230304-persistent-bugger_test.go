// https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func ToArray(input int) []int {
	output := []int{}

	for input > 0 {
		output = append(output, input%10)
		input = input / 10
	}

	return output
}

func TestToArray(t *testing.T) {
	assert.Equal(t, ToArray(999), []int{9, 9, 9})
	assert.Equal(t, ToArray(123), []int{3, 2, 1})
}

func Reduce(input []int) int {
	output := 1

	for _, elem := range input {
		output = output * elem
	}

	return output
}

func Persistence(n int) int {
	current := n
	count := 0

	for {
		if current < 10 {
			break
		}

		numbers := ToArray(current)

		current = Reduce(numbers)
		count = count + 1
	}

	return count
}

func TestPersistence(t *testing.T) {
	assert.Equal(t, Persistence(999), 4)
}
