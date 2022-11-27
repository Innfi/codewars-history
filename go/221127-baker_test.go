// https://www.codewars.com/kata/525c65e51bf619685c000059

package codewars_history

import (
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Cakes(recipe, available map[string]int) int {
	minAvailable := 999999.0

	for key := range recipe {
		if available[key] == 0 {
			return 0
		}

		unit := math.Floor(float64(available[key]) / float64(recipe[key]))
		if minAvailable > unit {
			minAvailable = unit
		}
	}

	return int(minAvailable)
}

func TestCakes(t *testing.T) {
	assert.Equal(t,
		Cakes(
			map[string]int{"flour": 500, "sugar": 200, "eggs": 1},
			map[string]int{"flour": 1200, "sugar": 1200, "eggs": 5, "milk": 200},
		), 2)
}
