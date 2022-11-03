// https://www.codewars.com/kata/536a155256eb459b8700077e

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func MakeSpirial(n int) [][]int {
	spiral := make([][]int, n)
	for i := range spiral {
		spiral[i] = make([]int, n)
	}

	return spiral
}

func MarkLeft(spiral *[][]int, row *int, col *int, current *int) {
	for *row < len(*spiral)-1 {
		(*spiral)[*col][*row] = *current
		*row += 1
		*current += 1
	}
}

func TestSpiralSize(t *testing.T) {
	spiral := MakeSpirial(3)

	assert.Equal(t, len(spiral), 3)

	for i := range spiral {
		assert.Equal(t, len(spiral[i]), 3)
	}
}

func TestMarkLeft(t *testing.T) {
	spiral := MakeSpirial(5)

	row := 0
	col := 0
	current := 1

	MarkLeft(&spiral, &row, &col, &current)

	assert.Equal(t, row, 4)
	assert.Equal(t, col, 0)
	assert.Equal(t, current, 5)

	assert.Equal(t, spiral[0][0], 1)
	assert.Equal(t, spiral[0][2], 3)
	assert.Equal(t, spiral[0][4], 5)
}

// func CreateSpiral(n int) [][]int {
// 	// your code here

// 	count := n*n
// 	row := 0
// 	col := 0
// 	for i := 0; i< count ;i++ {

// 	}

// 	return spiral
// }

// func TestSpiral(t *testing.T) {
// 	assert.Equal(t, CreateSpiral(1), [][]int{{1}})
// 	// assert.Equal(t, CreateSpiral(2), [][]int{{1, 2}, {4, 3}})
// 	// assert.Equal(t, CreateSpiral(3), [][]int{{1, 2, 3}, {8, 9, 4}, {7, 6, 5}})
// }
