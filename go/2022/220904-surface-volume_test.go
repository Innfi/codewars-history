// https://www.codewars.com/kata/565f5825379664a26b00007c

package codewars_history

import (
	"reflect"
	"testing"

	"github.com/stretchr/testify/assert"
)

func GetSize(w, h, d int) [2]int {
	return [2]int{
		2 * (w*h + h*d + d*w),
		w * h * d,
	}
}

func TestBasic(t *testing.T) {
	assert.Equal(t, reflect.DeepEqual(GetSize(4, 2, 6), [2]int{88, 48}), true)
	assert.Equal(t, reflect.DeepEqual(GetSize(1, 1, 1), [2]int{6, 1}), true)
	assert.Equal(t, reflect.DeepEqual(GetSize(1, 2, 1), [2]int{10, 2}), true)
}
