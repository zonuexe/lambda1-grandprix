// λ-1 translator — Go prelude
package main

import (
	"fmt"
	"os"
)

// 万能型（タグ付き union）: interface に Fun/Num/Str のいずれかを入れる。
// 反射型 `type D func(D) D` のみでは観測できないため union 化する。
type D interface{}
type Fun func(D) D
type Num int
type Str string

func app(g D, x D) D {
	return g.(Fun)(x)
}

func encodeInt(n int) D {
	return Fun(func(f D) D {
		return Fun(func(x D) D {
			acc := x
			for i := 0; i < n; i++ {
				acc = app(f, acc)
			}
			return acc
		})
	})
}

func decodeInt(t D) string {
	incr := Fun(func(v D) D { return Num(int(v.(Num)) + 1) })
	r := app(app(t, incr), Num(0))
	return fmt.Sprintf("%d", int(r.(Num)))
}

func decodeBool(t D) string {
	r := app(app(t, Str("true")), Str("false"))
	return string(r.(Str))
}

var _failures = 0

func check(label string, a string, b string) {
	if a == b {
		fmt.Println("ok   " + label)
	} else {
		_failures++
		fmt.Println("FAIL " + label + ": " + a + " != " + b)
	}
}

//__DEFS__

func main() {
//__ASSERTS__
	if _failures > 0 {
		fmt.Printf("%d failure(s)\n", _failures)
		os.Exit(1)
	}
	fmt.Println("all green")
}
