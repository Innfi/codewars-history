// https://www.codewars.com/kata/54207f9677730acd490000d1

/*
func PassHash(str string) string {
	return fmt.Sprintf("%x",md5.Sum([]byte(str)))
}
*/

package codewars_history

import (
	"crypto/md5"
	"fmt"
	"io"
	"testing"

	"github.com/stretchr/testify/assert"
)

func PassHash(str string) string {
	instance := md5.New()
	io.WriteString(instance, str)

	result := fmt.Sprintf("%x", instance.Sum(nil))

	return result
}

func TestPassHash(t *testing.T) {
	assert.Equal(t, PassHash("password"), "5f4dcc3b5aa765d61d8327deb882cf99")
	assert.Equal(t, PassHash("abc123"), "e99a18c428cb38d5f260853678922e03")
	assert.Equal(t, PassHash(""), "d41d8cd98f00b204e9800998ecf8427e")
}
