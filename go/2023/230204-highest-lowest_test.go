// https://www.codewars.com/kata/554b4ac871d6813a03000035

package codewars_history

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func HighAndLow(in string) string {
	slice := strings.Split(in, " ")

	result := []int{}

	for _, elem := range slice {
		toNumber, _ := strconv.Atoi(elem)
		result = append(result, toNumber)
	}

	sort.Ints(result)

	asString := fmt.Sprintf("%d %d", result[len(result)-1], result[0])

	return asString
}
func TestParseString(t *testing.T) {
	input := "1 2 3 4 5"

	slice := strings.Split(input, " ")

	assert.Equal(t, slice[0], "1")
}

func TestXxx(t *testing.T) {
	assert.Equal(t, HighAndLow("1 2 3 4 5"), "5 1")
}
