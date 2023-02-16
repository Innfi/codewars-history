// https://www.codewars.com/kata/545af3d185166a3dec001190

/**
func EachCons(arr []int, n int) (res [][]int) {
  for i := 0; i + n <= len(arr); i++ {
    res = append(res,arr[i:i + n])
  }
  return
}
*/

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func EachCons(arr []int, n int) [][]int {

	result := [][]int{}
	arrLen := len(arr)
	current := 0

	for arrLen >= n {
		result = append(result, arr[current:n+current])
		current += 1
		arrLen -= 1
	}

	return result
}

func TestEachCons(t *testing.T) {
	arr := []int{3, 5, 8, 13}

	assert.Equal(t, EachCons(arr, 2), [][]int{{3, 5}, {5, 8}, {8, 13}})
	assert.Equal(t, EachCons(arr, 3), [][]int{{3, 5, 8}, {5, 8, 13}})
}
