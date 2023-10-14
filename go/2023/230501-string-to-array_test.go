// https://www.codewars.com/kata/57e76bc428d6fbc2d500036d

package codewars_history

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func StringToArray(str string) []string {
	return strings.Split(str, " ")
}

func TestStringToArray(t *testing.T) {
	assert.Equal(t, StringToArray("Robin Singh"), []string{"Robin", "Singh"})
}
