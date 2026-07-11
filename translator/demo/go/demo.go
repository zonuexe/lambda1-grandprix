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

var _I D = Fun(func(_x D) D { return _x })
var _K D = Fun(func(_x D) D { return Fun(func(_y D) D { return _x }) })
var _S D = Fun(func(_x D) D { return Fun(func(_y D) D { return Fun(func(_z D) D { return app(app(_x, _z), app(_y, _z)) }) }) })
var _zero D = Fun(func(_f D) D { return Fun(func(_x D) D { return _x }) })
var _succ D = Fun(func(_n D) D { return Fun(func(_f D) D { return Fun(func(_x D) D { return app(_f, app(app(_n, _f), _x)) }) }) })
var _add D = Fun(func(_m D) D { return Fun(func(_n D) D { return Fun(func(_f D) D { return Fun(func(_x D) D { return app(app(_m, _f), app(app(_n, _f), _x)) }) }) }) })
var _true D = Fun(func(_t D) D { return Fun(func(_f D) D { return _t }) })
var _false D = Fun(func(_t D) D { return Fun(func(_f D) D { return _f }) })
var _if D = Fun(func(_b D) D { return Fun(func(_t D) D { return Fun(func(_e D) D { return app(app(_b, _t), _e) }) }) })
var _and D = Fun(func(_p D) D { return Fun(func(_q D) D { return app(app(_p, _q), _p) }) })
var _not D = Fun(func(_b D) D { return app(app(_b, _false), _true) })

func main() {
	check("assert 1", "1", decodeInt(app(app(app(_S, _K), _K), encodeInt(1))))
	check("assert 2", "0", decodeInt(_zero))
	check("assert 3", "3", decodeInt(app(_succ, encodeInt(2))))
	check("assert 4", "2", decodeInt(app(app(_add, encodeInt(1)), encodeInt(1))))
	check("assert 5", "true", decodeBool(app(app(_and, _true), _true)))
	check("assert 6", "false", decodeBool(app(app(_and, _true), _false)))
	check("assert 7", "false", decodeBool(app(app(app(_if, _false), _true), _false)))
	check("assert 8", "true", decodeBool(app(_not, _false)))
	if _failures > 0 {
		fmt.Printf("%d failure(s)\n", _failures)
		os.Exit(1)
	}
	fmt.Println("all green")
}
