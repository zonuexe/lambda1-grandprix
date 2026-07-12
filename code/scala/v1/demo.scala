// λ-1 translator — Scala 3 prelude

sealed trait D
case class Fun(f: D => D) extends D
case class Num(n: Int) extends D
case class Str(s: String) extends D

def app(g: D, x: D): D = g match {
  case Fun(f) => f(x)
  case _ => throw new RuntimeException("applied a non-function")
}

def encodeInt(n: Int): D =            // host int -> チャーチ数
  Fun(f => Fun(x => {
    var acc = x
    for (_ <- 0 until n) acc = app(f, acc)
    acc
  }))

def decodeInt(t: D): String = {       // Num を注入して数える
  val incr = Fun(v => v match { case Num(k) => Num(k + 1); case _ => throw new RuntimeException("incr") })
  app(app(t, incr), Num(0)) match {
    case Num(k) => k.toString
    case _ => throw new RuntimeException("decodeInt")
  }
}

def decodeBool(t: D): String =        // Str を注入
  app(app(t, Str("true")), Str("false")) match {
    case Str(s) => s
    case _ => throw new RuntimeException("decodeBool")
  }

var _failures = 0
def check(label: String, a: String, b: String): Unit =
  if (a == b) println("ok   " + label)
  else { _failures += 1; println("FAIL " + label + ": " + a + " != " + b) }

// ============ JSON 層（型付き値。ADR-0005） ============
val jK: D = Fun(a => Fun(b => a))
val jKI: D = Fun(a => Fun(b => b))
def mkpair(a: D, b: D): D = Fun(s => app(app(s, a), b))
def fstp(p: D): D = app(p, jK)
def sndp(p: D): D = app(p, jKI)
def churchToInt(c: D): Int =
  app(app(c, Fun(v => v match { case Num(k) => Num(k + 1); case _ => throw new RuntimeException("churchToInt") })), Num(0)) match {
    case Num(k) => k
    case _ => throw new RuntimeException("churchToInt")
  }
val cTrue: D = Fun(t => Fun(f => t))
val cFalse: D = Fun(t => Fun(f => f))
def boolToHost(c: D): Boolean =
  app(app(c, Str("T")), Str("F")) match {
    case Str(s) => s == "T"
    case _ => throw new RuntimeException("boolToHost")
  }
val nilH: D = Fun(n => Fun(c => n))
def consH(h: D, t: D): D = Fun(n => Fun(c => app(app(c, h), t)))
def isNil(lst: D): Boolean =
  boolToHost(app(app(lst, cTrue), Fun(h => Fun(t => cFalse))))
def headL(lst: D): D = app(app(lst, Str("")), Fun(h => Fun(t => h)))
def tailL(lst: D): D = app(app(lst, Str("")), Fun(h => Fun(t => t)))
def walkL(lst: D): List[D] = {
  val out = scala.collection.mutable.ListBuffer[D]()
  var l = lst
  while (!isNil(l)) { out += headL(l); l = tailL(l) }
  out.toList
}
def jInt(n: Int): D = mkpair(encodeInt(1), encodeInt(n))
def jBool(b: D): D = mkpair(encodeInt(2), b)
def jStr(s: String): D = {
  var lst = nilH
  val bytes = s.getBytes(java.nio.charset.StandardCharsets.UTF_8)
  for (i <- bytes.length - 1 to 0 by -1) lst = consH(encodeInt(bytes(i) & 0xFF), lst)
  mkpair(encodeInt(3), lst)
}
def jArr(lst: D): D = mkpair(encodeInt(4), lst)
def jObj(lst: D): D = mkpair(encodeInt(5), lst)
def jNull(): D = mkpair(encodeInt(6), Fun(x => x))
def jsonEscape(s: String): String = {
  val sb = new StringBuilder("\"")
  for (c <- s) c match {
    case '"' => sb.append("\\\"")
    case '\\' => sb.append("\\\\")
    case _ => sb.append(c)
  }
  sb.append("\"").toString
}
def decodeJson(v: D): String = {
  val tag = churchToInt(fstp(v))
  val payload = sndp(v)
  tag match {
    case 1 => churchToInt(payload).toString
    case 2 => if (boolToHost(payload)) "true" else "false"
    case 3 =>
      val bs = walkL(payload)
      val arr = new Array[Byte](bs.length)
      for (i <- bs.indices) arr(i) = churchToInt(bs(i)).toByte
      jsonEscape(new String(arr, java.nio.charset.StandardCharsets.UTF_8))
    case 4 =>
      "[" + walkL(payload).map(decodeJson).mkString(",") + "]"
    case 5 =>
      "{" + walkL(payload).map(pr => decodeJson(fstp(pr)) + ":" + decodeJson(sndp(pr))).mkString(",") + "}"
    case 6 => "null"
    case _ => "?"
  }
}

val _I: D = Fun(_x => _x)
val _K: D = Fun(_x => Fun(_y => _x))
val _S: D = Fun(_x => Fun(_y => Fun(_z => app(app(_x, _z), app(_y, _z)))))
val _zero: D = Fun(_f => Fun(_x => _x))
val _succ: D = Fun(_n => Fun(_f => Fun(_x => app(_f, app(app(_n, _f), _x)))))
val _add: D = Fun(_m => Fun(_n => Fun(_f => Fun(_x => app(app(_m, _f), app(app(_n, _f), _x))))))
val _true: D = Fun(_t => Fun(_f => _t))
val _false: D = Fun(_t => Fun(_f => _f))
val _if: D = Fun(_b => Fun(_t => Fun(_e => app(app(_b, _t), _e))))
val _and: D = Fun(_p => Fun(_q => app(app(_p, _q), _p)))
val _not: D = Fun(_b => app(app(_b, _false), _true))

@main def run(): Unit = {
  check("assert 1", "1", decodeInt(app(app(app(_S, _K), _K), encodeInt(1))))
  check("assert 2", "0", decodeInt(_zero))
  check("assert 3", "3", decodeInt(app(_succ, encodeInt(2))))
  check("assert 4", "2", decodeInt(app(app(_add, encodeInt(1)), encodeInt(1))))
  check("assert 5", "true", decodeBool(app(app(_and, _true), _true)))
  check("assert 6", "false", decodeBool(app(app(_and, _true), _false)))
  check("assert 7", "false", decodeBool(app(app(app(_if, _false), _true), _false)))
  check("assert 8", "true", decodeBool(app(_not, _false)))
  if (_failures > 0) { println(_failures.toString + " failure(s)"); System.exit(1) }
  println("all green")
}
