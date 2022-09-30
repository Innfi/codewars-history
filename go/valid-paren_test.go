// https://www.codewars.com/kata/52774a314c2333f0a7000688

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func ValidParentheses(parens string) bool {
	flag := 0

	for _, elem := range parens {
		if elem == '(' {
			flag += 1
			continue
		}

		flag -= 1

		if flag < 0 {
			return false
		}
	}

	return flag == 0
}

func TestBaseic(t *testing.T) {
	assert.Equal(t, ValidParentheses("()"), true)
	assert.Equal(t, ValidParentheses("("), false)
}
