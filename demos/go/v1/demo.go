// λ-1 translator — Go prelude
package main

import (
	"fmt"
	"os"
	"strings"
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

// ============ JSON 層（型付き値。ADR-0005） ============
func jK() D  { return Fun(func(a D) D { return Fun(func(b D) D { return a }) }) }
func jKI() D { return Fun(func(a D) D { return Fun(func(b D) D { return b }) }) }
func mkpair(a D, b D) D { return Fun(func(s D) D { return app(app(s, a), b) }) }
func fstp(p D) D        { return app(p, jK()) }
func sndp(p D) D        { return app(p, jKI()) }

func churchToInt(c D) int {
	incr := Fun(func(v D) D { return Num(int(v.(Num)) + 1) })
	return int(app(app(c, incr), Num(0)).(Num))
}
func boolToHost(c D) bool {
	return string(app(app(c, Str("T")), Str("F")).(Str)) == "T"
}

func nilH() D          { return Fun(func(n D) D { return Fun(func(c D) D { return n }) }) }
func consH(h D, t D) D { return Fun(func(n D) D { return Fun(func(c D) D { return app(app(c, h), t) }) }) }
func isNil(lst D) bool {
	cTrue := Fun(func(t D) D { return Fun(func(f D) D { return t }) })
	cFalse := Fun(func(t D) D { return Fun(func(f D) D { return f }) })
	consCase := Fun(func(h D) D { return Fun(func(t D) D { return cFalse }) })
	return boolToHost(app(app(lst, cTrue), consCase))
}
func headL(lst D) D {
	return app(app(lst, Str("")), Fun(func(h D) D { return Fun(func(t D) D { return h }) }))
}
func tailL(lst D) D {
	return app(app(lst, Str("")), Fun(func(h D) D { return Fun(func(t D) D { return t }) }))
}
func walkL(lst D) []D {
	out := []D{}
	for !isNil(lst) {
		out = append(out, headL(lst))
		lst = tailL(lst)
	}
	return out
}

func jInt(n int) D { return mkpair(encodeInt(1), encodeInt(n)) }
func jBool(b D) D  { return mkpair(encodeInt(2), b) }
func jStr(s string) D {
	lst := nilH()
	bs := []byte(s)
	for i := len(bs) - 1; i >= 0; i-- {
		lst = consH(encodeInt(int(bs[i])), lst)
	}
	return mkpair(encodeInt(3), lst)
}
func jArr(lst D) D { return mkpair(encodeInt(4), lst) }
func jObj(lst D) D { return mkpair(encodeInt(5), lst) }
func jNull() D     { return mkpair(encodeInt(6), Fun(func(x D) D { return x })) }

func jsonEscape(s string) string {
	var b strings.Builder
	b.WriteByte('"')
	for _, c := range s {
		switch c {
		case '"':
			b.WriteString("\\\"")
		case '\\':
			b.WriteString("\\\\")
		default:
			b.WriteRune(c)
		}
	}
	b.WriteByte('"')
	return b.String()
}

func decodeJson(v D) string {
	tag := churchToInt(fstp(v))
	payload := sndp(v)
	switch tag {
	case 1:
		return fmt.Sprintf("%d", churchToInt(payload))
	case 2:
		if boolToHost(payload) {
			return "true"
		}
		return "false"
	case 3:
		xs := walkL(payload)
		bs := make([]byte, len(xs))
		for i, x := range xs {
			bs[i] = byte(churchToInt(x))
		}
		return jsonEscape(string(bs))
	case 4:
		xs := walkL(payload)
		parts := make([]string, len(xs))
		for i, x := range xs {
			parts[i] = decodeJson(x)
		}
		return "[" + strings.Join(parts, ",") + "]"
	case 5:
		xs := walkL(payload)
		parts := make([]string, len(xs))
		for i, pr := range xs {
			parts[i] = decodeJson(fstp(pr)) + ":" + decodeJson(sndp(pr))
		}
		return "{" + strings.Join(parts, ",") + "}"
	case 6:
		return "null"
	}
	return "?"
}

var I D = Fun(func(x D) D { return x })
var K D = Fun(func(x D) D { return Fun(func(y D) D { return x }) })
var S D = Fun(func(x D) D { return Fun(func(y D) D { return Fun(func(z D) D { return app(app(x, z), app(y, z)) }) }) })
var zero D = Fun(func(f D) D { return Fun(func(x D) D { return x }) })
var succ D = Fun(func(n D) D { return Fun(func(f D) D { return Fun(func(x D) D { return app(f, app(app(n, f), x)) }) }) })
var add D = Fun(func(m D) D { return Fun(func(n D) D { return Fun(func(f D) D { return Fun(func(x D) D { return app(app(m, f), app(app(n, f), x)) }) }) }) })
var _true D = Fun(func(t D) D { return Fun(func(f D) D { return t }) })
var _false D = Fun(func(t D) D { return Fun(func(f D) D { return f }) })
var _if D = Fun(func(b D) D { return Fun(func(t D) D { return Fun(func(e D) D { return app(app(b, t), e) }) }) })
var and D = Fun(func(p D) D { return Fun(func(q D) D { return app(app(p, q), p) }) })
var not D = Fun(func(b D) D { return app(app(b, _false), _true) })

func main() {
	check("assert 1", "1", decodeInt(app(app(app(S, K), K), encodeInt(1))))
	check("assert 2", "0", decodeInt(zero))
	check("assert 3", "3", decodeInt(app(succ, encodeInt(2))))
	check("assert 4", "2", decodeInt(app(app(add, encodeInt(1)), encodeInt(1))))
	check("assert 5", "true", decodeBool(app(app(and, _true), _true)))
	check("assert 6", "false", decodeBool(app(app(and, _true), _false)))
	check("assert 7", "false", decodeBool(app(app(app(_if, _false), _true), _false)))
	check("assert 8", "true", decodeBool(app(not, _false)))
	if _failures > 0 {
		fmt.Printf("%d failure(s)\n", _failures)
		os.Exit(1)
	}
	fmt.Println("all green")
}
