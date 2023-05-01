// https://www.codewars.com/kata/5a34af40e1ce0eb1f5000036

package codewars_history

import (
	"fmt"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func ToCsvText(array [][]int) string {
	rows := []string{}

	for _, subarray := range array {
		rowAsString := ""
		for _, elem := range subarray {
			if rowAsString == "" {
				rowAsString = fmt.Sprintf("%d", elem)
				continue
			}
			rowAsString = fmt.Sprintf("%s,%d", rowAsString, elem)
		}

		rows = append(rows, rowAsString)
	}

	return strings.Join(rows, "\n")
}

func TestCsvText(t *testing.T) {
	assert.Equal(t, ToCsvText([][]int{
		{0, 1, 2, 3, 45},
		{10, 11, 12, 13, 14},
		{20, 21, 22, 23, 24},
		{30, 31, 32, 33, 34}}),
		"0,1,2,3,45\n10,11,12,13,14\n20,21,22,23,24\n30,31,32,33,34")
}
