// https://www.codewars.com/kata/57f8ee485cae443c4d000127

package codewars_history

import (
	"fmt"
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func MaxRot(n int64) int64 {
	asString := fmt.Sprintf("%d", n)
	max := asString

	for i := 0; i < len(asString)-1; i++ {

		asString = Rotate(asString, i)
		if asString > max {
			max = asString
		}
	}

	result, _ := strconv.ParseInt(max, 10, 64)

	return result
}

func Rotate(input string, offset int) string {
	if offset == 0 {
		return fmt.Sprintf("%s%s", input[1:], input[:1])
	}

	return fmt.Sprintf("%s%s%s", input[0:offset], input[offset+1:], string(input[offset]))
}

func TestRotate(t *testing.T) {
	input := "12345"

	assert.Equal(t, input[1:2], "2")
	assert.Equal(t, input[0:2], "12")
	assert.Equal(t, string(input[2]), "3")
	assert.Equal(t, input[3:], "45")

	assert.Equal(t, Rotate(input, 0), "23451")
	assert.Equal(t, Rotate(input, 2), "12453")
	assert.Equal(t, Rotate(input, 3), "12354")
}

func TestMaxRot(t *testing.T) {
	// assert.Equal(t, MaxRot(56789), int64(68957))
	assert.Equal(t, MaxRot(38458215), int64(85821534))
}
