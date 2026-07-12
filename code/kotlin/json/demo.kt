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

val pair: D = Fun({ a -> Fun({ b -> Fun({ s -> app(app(s, a), b) }) }) })
val nil: D = Fun({ n -> Fun({ c -> n }) })
val cons: D = Fun({ h -> Fun({ t -> Fun({ n -> Fun({ c -> app(app(c, h), t) }) }) }) })
val _true: D = Fun({ t -> Fun({ f -> t }) })
val _false: D = Fun({ t -> Fun({ f -> f }) })
val snd: D = Fun({ p -> app(p, _false) })
val one: D = Fun({ f -> Fun({ x -> app(f, x) }) })
val pred: D = Fun({ n -> Fun({ f -> Fun({ x -> app(app(app(n, Fun({ g -> Fun({ h -> app(h, app(g, f)) }) })), Fun({ u -> x })), Fun({ u -> u })) }) }) })
val tint: D = Fun({ k -> app(app(pair, one), k) })
val step: D = Fun({ p -> app(p, Fun({ k -> Fun({ l -> app(app(pair, app(pred, k)), app(app(cons, app(tint, k)), l)) }) })) })
val range: D = Fun({ n -> app(snd, app(app(n, step), app(app(pair, n), nil))) })

fun main() {
    check("assert 1", "1", decodeJson(jInt(1)))
    check("assert 2", "true", decodeJson(jBool(_true)))
    check("assert 3", "false", decodeJson(jBool(_false)))
    check("assert 4", "\"hi\"", decodeJson(jStr("hi")))
    check("assert 5", "null", decodeJson(jNull()))
    check("assert 6", "[1,true]", decodeJson(jArr(app(app(cons, jInt(1)), app(app(cons, jBool(_true)), nil)))))
    check("assert 7", "{\"k\":1}", decodeJson(jObj(app(app(cons, app(app(pair, jStr("k")), jInt(1))), nil))))
    check("assert 8", "[1,[2,3]]", decodeJson(jArr(app(app(cons, jInt(1)), app(app(cons, jArr(app(app(cons, jInt(2)), app(app(cons, jInt(3)), nil)))), nil)))))
    check("assert 9", "[]", decodeJson(jArr(app(range, encodeInt(0)))))
    check("assert 10", "[1]", decodeJson(jArr(app(range, encodeInt(1)))))
    check("assert 11", "[1,2,3]", decodeJson(jArr(app(range, encodeInt(3)))))
    if (_failures > 0) { println("${_failures} failure(s)"); kotlin.system.exitProcess(1) }
    println("all green")
}
