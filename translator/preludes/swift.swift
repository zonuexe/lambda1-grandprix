// λ-1 translator — Swift prelude (system swift)
import Foundation

// 万能型（タグ付き union）。関数型は参照的なので indirect 不要。
enum D {
    case fun((D) -> D)
    case num(Int)
    case str(String)
}

func app(_ g: D, _ x: D) -> D {
    switch g {
    case .fun(let f): return f(x)
    default: fatalError("applied a non-function")
    }
}

func encodeInt(_ n: Int) -> D {          // host int -> チャーチ数
    return .fun { f in
        .fun { x in
            var acc = x
            for _ in 0..<n { acc = app(f, acc) }
            return acc
        }
    }
}

func decodeInt(_ t: D) -> String {       // Num を注入して数える
    let incr = D.fun { v in
        if case .num(let k) = v { return .num(k + 1) } else { fatalError("incr") }
    }
    let r = app(app(t, incr), .num(0))
    if case .num(let k) = r { return String(k) } else { fatalError("decodeInt") }
}

func decodeBool(_ t: D) -> String {      // Str を注入
    let r = app(app(t, .str("true")), .str("false"))
    if case .str(let s) = r { return s } else { fatalError("decodeBool") }
}

var _failures = 0
func check(_ label: String, _ a: String, _ b: String) {
    if a == b { print("ok   " + label) }
    else { _failures += 1; print("FAIL " + label + ": " + a + " != " + b) }
}

// ============ JSON 層（型付き値。ADR-0005） ============
func selK() -> D { .fun { a in .fun { _ in a } } }   // λa.λb.a
func selKI() -> D { .fun { _ in .fun { b in b } } }  // λa.λb.b
func cTrue() -> D { .fun { t in .fun { _ in t } } }
func cFalse() -> D { .fun { _ in .fun { f in f } } }

func mkpair(_ a: D, _ b: D) -> D { .fun { s in app(app(s, a), b) } }
func fstp(_ p: D) -> D { app(p, selK()) }
func sndp(_ p: D) -> D { app(p, selKI()) }

func churchToInt(_ c: D) -> Int {
    let incr = D.fun { v in
        if case .num(let k) = v { return .num(k + 1) } else { fatalError("incr") }
    }
    if case .num(let k) = app(app(c, incr), .num(0)) { return k } else { fatalError("churchToInt") }
}
func boolToHost(_ c: D) -> Bool {
    if case .str(let s) = app(app(c, .str("T")), .str("F")) { return s == "T" } else { return false }
}

func nilH() -> D { .fun { n in .fun { _ in n } } }
func consH(_ h: D, _ t: D) -> D { .fun { _ in .fun { c in app(app(c, h), t) } } }
func isNil(_ lst: D) -> Bool {
    let consCase = D.fun { _ in .fun { _ in cFalse() } }
    return boolToHost(app(app(lst, cTrue()), consCase))
}
func headL(_ lst: D) -> D { app(app(lst, .str("")), selK()) }
func tailL(_ lst: D) -> D { app(app(lst, .str("")), selKI()) }
func walkL(_ lst: D) -> [D] {
    var out: [D] = []
    var cur = lst
    while !isNil(cur) {
        out.append(headL(cur))
        cur = tailL(cur)
    }
    return out
}

func jInt(_ n: Int) -> D { mkpair(encodeInt(1), encodeInt(n)) }
func jBool(_ b: D) -> D { mkpair(encodeInt(2), b) }
func jStr(_ s: String) -> D {
    var lst = nilH()
    for byte in Array(s.utf8).reversed() {
        lst = consH(encodeInt(Int(byte)), lst)
    }
    return mkpair(encodeInt(3), lst)
}
func jArr(_ lst: D) -> D { mkpair(encodeInt(4), lst) }
func jObj(_ lst: D) -> D { mkpair(encodeInt(5), lst) }
func jNull() -> D { mkpair(encodeInt(6), .fun { x in x }) }

func jsonEscape(_ s: String) -> String {
    var out = "\""
    for c in s {
        if c == "\"" { out += "\\\"" }
        else if c == "\\" { out += "\\\\" }
        else { out.append(c) }
    }
    out += "\""
    return out
}

func decodeJson(_ v: D) -> String {
    let tag = churchToInt(fstp(v))
    let payload = sndp(v)
    switch tag {
    case 1:
        return String(churchToInt(payload))
    case 2:
        return boolToHost(payload) ? "true" : "false"
    case 3:
        let bytes = walkL(payload).map { UInt8(churchToInt($0)) }
        return jsonEscape(String(decoding: bytes, as: UTF8.self))
    case 4:
        return "[" + walkL(payload).map { decodeJson($0) }.joined(separator: ",") + "]"
    case 5:
        return "{" + walkL(payload).map { decodeJson(fstp($0)) + ":" + decodeJson(sndp($0)) }.joined(separator: ",") + "}"
    case 6:
        return "null"
    default:
        return "?"
    }
}

//__DEFS__

//__ASSERTS__
if _failures > 0 { print("\(_failures) failure(s)"); exit(1) }
print("all green")
