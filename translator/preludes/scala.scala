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

//__DEFS__

@main def run(): Unit = {
//__ASSERTS__
  if (_failures > 0) { println(_failures.toString + " failure(s)"); System.exit(1) }
  println("all green")
}
