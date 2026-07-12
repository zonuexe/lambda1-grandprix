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

// ============ JSON 層（型付き値。ADR-0005） ============
val jK: D = Fun({ a -> Fun({ b -> a }) })
val jKI: D = Fun({ a -> Fun({ b -> b }) })
fun mkpair(a: D, b: D): D = Fun({ s -> app(app(s, a), b) })
fun fstp(p: D): D = app(p, jK)
fun sndp(p: D): D = app(p, jKI)
fun churchToInt(c: D): Int {
    val r = app(app(c, Fun({ v -> Num((v as Num).n + 1) })), Num(0))
    return (r as Num).n
}
val cTrue: D = Fun({ t -> Fun({ f -> t }) })
val cFalse: D = Fun({ t -> Fun({ f -> f }) })
fun boolToHost(c: D): Boolean =
    (app(app(c, Str("T")), Str("F")) as Str).s == "T"
val nilH: D = Fun({ n -> Fun({ c -> n }) })
fun consH(h: D, t: D): D = Fun({ n -> Fun({ c -> app(app(c, h), t) }) })
fun isNil(lst: D): Boolean =
    boolToHost(app(app(lst, cTrue), Fun({ h -> Fun({ t -> cFalse }) })))
fun headL(lst: D): D = app(app(lst, Str("")), Fun({ h -> Fun({ t -> h }) }))
fun tailL(lst: D): D = app(app(lst, Str("")), Fun({ h -> Fun({ t -> t }) }))
fun walkL(lst: D): List<D> {
    val out = ArrayList<D>()
    var l = lst
    while (!isNil(l)) { out.add(headL(l)); l = tailL(l) }
    return out
}
fun jInt(n: Int): D = mkpair(encodeInt(1), encodeInt(n))
fun jBool(b: D): D = mkpair(encodeInt(2), b)
fun jStr(s: String): D {
    var lst = nilH
    val bytes = s.toByteArray(Charsets.UTF_8)
    for (i in bytes.size - 1 downTo 0) lst = consH(encodeInt(bytes[i].toInt() and 0xFF), lst)
    return mkpair(encodeInt(3), lst)
}
fun jArr(lst: D): D = mkpair(encodeInt(4), lst)
fun jObj(lst: D): D = mkpair(encodeInt(5), lst)
fun jNull(): D = mkpair(encodeInt(6), Fun({ x -> x }))
fun jsonEscape(s: String): String {
    val sb = StringBuilder("\"")
    for (c in s) {
        when (c) {
            '"' -> sb.append("\\\"")
            '\\' -> sb.append("\\\\")
            else -> sb.append(c)
        }
    }
    return sb.append("\"").toString()
}
fun decodeJson(v: D): String {
    val tag = churchToInt(fstp(v))
    val payload = sndp(v)
    when (tag) {
        1 -> return churchToInt(payload).toString()
        2 -> return if (boolToHost(payload)) "true" else "false"
        3 -> {
            val bs = walkL(payload)
            val arr = ByteArray(bs.size)
            for (i in bs.indices) arr[i] = churchToInt(bs[i]).toByte()
            return jsonEscape(String(arr, Charsets.UTF_8))
        }
        4 -> {
            val sb = StringBuilder("[")
            val xs = walkL(payload)
            for (i in xs.indices) { if (i > 0) sb.append(","); sb.append(decodeJson(xs[i])) }
            return sb.append("]").toString()
        }
        5 -> {
            val sb = StringBuilder("{")
            val xs = walkL(payload)
            for (i in xs.indices) {
                if (i > 0) sb.append(",")
                val pr = xs[i]
                sb.append(decodeJson(fstp(pr))).append(":").append(decodeJson(sndp(pr)))
            }
            return sb.append("}").toString()
        }
        6 -> return "null"
        else -> return "?"
    }
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
