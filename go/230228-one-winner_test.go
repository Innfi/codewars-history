// https://www.codewars.com/kata/577bd8d4ae2807c64b00045b

package codewars_history

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

type Fighter struct {
	Name            string
	Health          int
	DamagePerAttack int
}

func DeclareWinner(fighter1 Fighter, fighter2 Fighter, firstAttacker string) string {
	var lhs *Fighter
	var rhs *Fighter

	if fighter1.Name == firstAttacker {
		lhs = &fighter1
		rhs = &fighter2
	} else {
		rhs = &fighter1
		lhs = &fighter2
	}

	rhs.Health = rhs.Health - lhs.DamagePerAttack

	for lhs.Health > 0 && rhs.Health > 0 {
		lhs.Health = lhs.Health - rhs.DamagePerAttack
		rhs.Health = rhs.Health - lhs.DamagePerAttack
	}

	if lhs.Health <= 0 {
		return rhs.Name
	}

	return lhs.Name
}

func TestDeclareWiiner(t *testing.T) {
	fighter1 := Fighter{
		"aa",
		10,
		2,
	}

	fighter2 := Fighter{
		"bb",
		5,
		4,
	}

	assert.Equal(t, DeclareWinner(fighter1, fighter2, "aa"), "aa")
}
