// https://www.codewars.com/kata/585d7d5adb20cf33cb000235

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func FindUniq(arr []float32) float32 {
	sameValue := arr[0]

	if arr[0] != arr[1] && arr[1] == arr[2] {
		return arr[0]
	}

	for _, elem := range arr {
		if elem != sameValue {
			return elem
		}
	}

	return arr[0]
}

func TestFindUniq(t *testing.T) {
	var result float32 = 2.0
	assert.Equal(t, FindUniq([]float32{1.0, 1.0, 1.0, 2.0, 1.0, 1.0}), result)
}
