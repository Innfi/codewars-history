// https://www.codewars.com/kata/513e08acc600c94f01000001

package codewars_history

import (
	"fmt"
	"strconv"
	"strings"

	"testing"

	"github.com/stretchr/testify/assert"
)

func ToHex(rgb int) string {
	if rgb <= 0 {
		return "00"
	}
	if rgb >= 255 {
		return "ff"
	}

	return fmt.Sprintf("%02s", strconv.FormatInt(int64(rgb), 16))
}

func RGB(r, g, b int) string {
	return strings.ToUpper(ToHex(r) + ToHex(g) + ToHex(b))
}

func TestToUpper(t *testing.T) {
	assert.Equal(t, strings.ToUpper("ff120d"), "FF120D")
}

func TestConvertHex(t *testing.T) {
	assert.Equal(t, strconv.FormatInt(255, 16), "ff")
}

func TestRGBToHex(t *testing.T) {
	assert.Equal(t, RGB(0, 0, 0), "000000")
	assert.Equal(t, RGB(255, 255, 255), "FFFFFF")
	assert.Equal(t, RGB(1, 2, 3), "010203")
	assert.Equal(t, RGB(-20, 275, 125), "00FF7D")
	assert.Equal(t, RGB(254, 253, 252), "FEFDFC")
}
