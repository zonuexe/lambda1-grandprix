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

//__DEFS__

fun main() {
//__ASSERTS__
    if (_failures > 0) { println("${_failures} failure(s)"); kotlin.system.exitProcess(1) }
    println("all green")
}
