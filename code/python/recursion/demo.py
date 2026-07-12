from lam1 import *  # ヘルパーは lam1.py

# --- definitions ---
Z = (lambda f: (lambda x: f((lambda v: x(x)(v))))((lambda x: f((lambda v: x(x)(v))))))
one = (lambda f: (lambda x: f(x)))
mult = (lambda m: (lambda n: (lambda f: m(n(f)))))
pred = (lambda n: (lambda f: (lambda x: n((lambda g: (lambda h: h(g(f)))))((lambda u: x))((lambda u: u)))))
true = (lambda t: (lambda f: t))
false = (lambda t: (lambda f: f))
isZero = (lambda n: n((lambda x: false))(true))
fstep = (lambda rec: (lambda n: isZero(n)((lambda u: one))((lambda u: mult(n)(rec(pred(n)))))(n)))
fact = Z(fstep)

# --- assertions ---
_check('1', decodeInt(fact(encodeInt(0))), 'assert 1')
_check('1', decodeInt(fact(encodeInt(1))), 'assert 2')
_check('2', decodeInt(fact(encodeInt(2))), 'assert 3')
_check('6', decodeInt(fact(encodeInt(3))), 'assert 4')
_check('120', decodeInt(fact(encodeInt(5))), 'assert 5')

_finish()
