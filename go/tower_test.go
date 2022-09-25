// https://www.codewars.com/kata/576757b1df89ecf5bd00073b

package codewars_history

import (
	"reflect"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func ToFloor(floor, len int) string {
	starCount := floor*2 - 1
	spaceCount := (len - starCount) / 2

	space := strings.Repeat(" ", spaceCount)
	star := strings.Repeat("*", starCount)

	return space + star + space
}

func TowerBuilder(nFloors int) []string {
	result := []string{}

	len := nFloors*2 - 1

	for i := 1; i <= nFloors; i++ {
		result = append(result, ToFloor(i, len))
	}

	return result
}

func TestToFloor(t *testing.T) {
	assert.Equal(t, ToFloor(1, 5), "  *  ")
	assert.Equal(t, ToFloor(3, 9), "  *****  ")
}

func TestStringPadding(t *testing.T) {
	result := strings.Repeat("*", 3)

	assert.Equal(t, result, "***")
}

func TestSingleFloor(t *testing.T) {
	assert.Equal(t, reflect.DeepEqual(TowerBuilder(1), []string{"*"}), true)
}

func TestTwoFloors(t *testing.T) {
	assert.Equal(t,
		reflect.DeepEqual(TowerBuilder(2), []string{" * ", "***"}),
		true)
}

func TestHighFloors(t *testing.T) {
	assert.Equal(t,
		reflect.DeepEqual(TowerBuilder(6),
			[]string{
				"     *     ",
				"    ***    ",
				"   *****   ",
				"  *******  ",
				" ********* ",
				"***********",
			}),
		true)
}
