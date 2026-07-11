// λ-1 translator — Kotlin prelude

sealed class D
class Fun(val f: (D) -> D) : D()
class Num(val n: Int) : D()
class Str(val s: String) : D()

fun app(g: D, x: D): D = when (g) {
    is Fun -> g.f(x)
    else -> throw RuntimeException("applied a non-function")
}

fun encodeInt(n: Int): D = Fun({ f -> Fun({ x ->     // host int -> チャーチ数
    var acc = x
    for (i in 0 until n) acc = app(f, acc)
    acc
}) })

fun decodeInt(t: D): String {                        // Num を注入して数える
    val incr = Fun({ v -> Num((v as Num).n + 1) })
    val r = app(app(t, incr), Num(0))
    return (r as Num).n.toString()
}

fun decodeBool(t: D): String {                       // Str を注入
    val r = app(app(t, Str("true")), Str("false"))
    return (r as Str).s
}

var _failures = 0
fun check(label: String, a: String, b: String) {
    if (a == b) println("ok   " + label)
    else { _failures++; println("FAIL " + label + ": " + a + " != " + b) }
}

val _I: D = Fun({ _x -> _x })
val _K: D = Fun({ _x -> Fun({ _y -> _x }) })
val _S: D = Fun({ _x -> Fun({ _y -> Fun({ _z -> app(app(_x, _z), app(_y, _z)) }) }) })
val _zero: D = Fun({ _f -> Fun({ _x -> _x }) })
val _succ: D = Fun({ _n -> Fun({ _f -> Fun({ _x -> app(_f, app(app(_n, _f), _x)) }) }) })
val _add: D = Fun({ _m -> Fun({ _n -> Fun({ _f -> Fun({ _x -> app(app(_m, _f), app(app(_n, _f), _x)) }) }) }) })
val _true: D = Fun({ _t -> Fun({ _f -> _t }) })
val _false: D = Fun({ _t -> Fun({ _f -> _f }) })
val _if: D = Fun({ _b -> Fun({ _t -> Fun({ _e -> app(app(_b, _t), _e) }) }) })
val _and: D = Fun({ _p -> Fun({ _q -> app(app(_p, _q), _p) }) })
val _not: D = Fun({ _b -> app(app(_b, _false), _true) })

fun main() {
    check("assert 1", "1", decodeInt(app(app(app(_S, _K), _K), encodeInt(1))))
    check("assert 2", "0", decodeInt(_zero))
    check("assert 3", "3", decodeInt(app(_succ, encodeInt(2))))
    check("assert 4", "2", decodeInt(app(app(_add, encodeInt(1)), encodeInt(1))))
    check("assert 5", "true", decodeBool(app(app(_and, _true), _true)))
    check("assert 6", "false", decodeBool(app(app(_and, _true), _false)))
    check("assert 7", "false", decodeBool(app(app(app(_if, _false), _true), _false)))
    check("assert 8", "true", decodeBool(app(_not, _false)))
    if (_failures > 0) { println("${_failures} failure(s)"); kotlin.system.exitProcess(1) }
    println("all green")
}
