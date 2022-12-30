// https://www.codewars.com/kata/5513795bd3fafb56c200049e

package codewars_history

import (
	"reflect"
	"testing"

	"github.com/stretchr/testify/assert"
)

func CountBy(x, n int) []int {
	data := []int{}

	for i := 1; i <= n; i++ {
		data = append(data, x*i)
	}

	return data
}

func TestCase1(t *testing.T) {
	result := CountBy(2, 5)
	expected := []int{2, 4, 6, 8, 10}

	assert.Equal(t, reflect.DeepEqual(result, expected), true)
}

func TestCase2(t *testing.T) {
	result := CountBy(1, 3)
	expected := []int{1, 2, 3}

	assert.Equal(t, reflect.DeepEqual(result, expected), true)
}
