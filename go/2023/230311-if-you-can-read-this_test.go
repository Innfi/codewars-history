// https://www.codewars.com/kata/586538146b56991861000293

package codewars_history

import (
	"fmt"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func ToNato(words string) string {
	nato := []string{"Alfa", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India", "Juliett", "Kilo", "Lima", "Mike", "November", "Oscar", "Papa", "Quebec", "Romeo", "Sierra", "Tango", "Uniform", "Victor", "Whiskey", "X-ray", "Yankee", "Zulu"}
	// Your code here :)

	upper := strings.ToUpper(words)
	fmt.Printf("upper: %s", upper)
	result := []string{}

	for _, elem := range upper {
		if elem == rune(' ') {
			continue
		}
		// var targetNato string
		targetNato := string(elem)

		for _, unit := range nato {
			if rune(unit[0]) == elem {
				targetNato = unit
				break
			}
		}

		result = append(result, targetNato)
	}

	return strings.Join(result, " ")
}

func TestToNato(t *testing.T) {
	assert.Equal(t, ToNato("if you can read"),
		"India Foxtrot Yankee Oscar Uniform Charlie Alfa November Romeo Echo Alfa Delta")
}
