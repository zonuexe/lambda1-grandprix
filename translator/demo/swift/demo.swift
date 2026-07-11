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

let _I: D = .fun { _x in _x }
let _K: D = .fun { _x in .fun { _y in _x } }
let _S: D = .fun { _x in .fun { _y in .fun { _z in app(app(_x, _z), app(_y, _z)) } } }
let _zero: D = .fun { _f in .fun { _x in _x } }
let _succ: D = .fun { _n in .fun { _f in .fun { _x in app(_f, app(app(_n, _f), _x)) } } }
let _add: D = .fun { _m in .fun { _n in .fun { _f in .fun { _x in app(app(_m, _f), app(app(_n, _f), _x)) } } } }
let _true: D = .fun { _t in .fun { _f in _t } }
let _false: D = .fun { _t in .fun { _f in _f } }
let _if: D = .fun { _b in .fun { _t in .fun { _e in app(app(_b, _t), _e) } } }
let _and: D = .fun { _p in .fun { _q in app(app(_p, _q), _p) } }
let _not: D = .fun { _b in app(app(_b, _false), _true) }

check("assert 1", "1", decodeInt(app(app(app(_S, _K), _K), encodeInt(1))))
check("assert 2", "0", decodeInt(_zero))
check("assert 3", "3", decodeInt(app(_succ, encodeInt(2))))
check("assert 4", "2", decodeInt(app(app(_add, encodeInt(1)), encodeInt(1))))
check("assert 5", "true", decodeBool(app(app(_and, _true), _true)))
check("assert 6", "false", decodeBool(app(app(_and, _true), _false)))
check("assert 7", "false", decodeBool(app(app(app(_if, _false), _true), _false)))
check("assert 8", "true", decodeBool(app(_not, _false)))
if _failures > 0 { print("\(_failures) failure(s)"); exit(1) }
print("all green")
