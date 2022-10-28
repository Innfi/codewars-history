// https://www.codewars.com/kata/52bc74d4ac05d0945d00054e

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"

	"strings"
)

func FirstNonRepeating(str string) string {
	if len(str) == 0 {
		return ""
	}

	charMap := make(map[string]int)

	for _, elem := range str {
		charMap[strings.ToLower(string(elem))] += 1
	}

	for _, elem := range str {
		key := string(elem)
		if charMap[strings.ToLower(key)] == 1 {
			return key
		}
	}

	return ""
}

func TestCharIntMap(t *testing.T) {
	charMap := make(map[string]int)

	charMap["a"] = 1
	charMap["a"] += 3

	charMap["b"] += 2

	assert.Equal(t, charMap["a"], 4)
	assert.Equal(t, charMap["b"], 2)
}

func TestNoneRepeating(t *testing.T) {
	assert.Equal(t, FirstNonRepeating("a"), "a")
	assert.Equal(t, FirstNonRepeating("stress"), "t")
	assert.Equal(t, FirstNonRepeating("moonmen"), "e")
	assert.Equal(t, FirstNonRepeating(""), "")
}
