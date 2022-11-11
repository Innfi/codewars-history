// https://www.codewars.com/kata/51e04f6b544cf3f6550000c1

package codewars_history

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Beeramid(bonus int, price float64) int {
	fmt.Printf("bonus: %d, price: %f\n", bonus, price)
	if bonus < 0 {
		return 0
	}

	height := 0
	for bonus >= 0 {
		height += 1
		bonus -= int(float64(height*height) * price)
	}

	return height - 1
}

func TestSquare(t *testing.T) {
	bonus := 1500
	price := 2

	height := 0

	for bonus > 0 {
		height += 1
		bonus = bonus - (height * height * price)
	}

	assert.Equal(t, height-1, 12)
}

func TestBeeramid(t *testing.T) {
	assert.Equal(t, Beeramid(9, 2.0), 1)
	assert.Equal(t, Beeramid(-1, 2.0), 0)
	assert.Equal(t, Beeramid(10, 2.0), 2)
}
