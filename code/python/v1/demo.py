from lam1 import *  # ヘルパーは lam1.py

# --- definitions ---
I = (lambda x: x)
K = (lambda x: (lambda y: x))
S = (lambda x: (lambda y: (lambda z: x(z)(y(z)))))
zero = (lambda f: (lambda x: x))
succ = (lambda n: (lambda f: (lambda x: f(n(f)(x)))))
add = (lambda m: (lambda n: (lambda f: (lambda x: m(f)(n(f)(x))))))
true = (lambda t: (lambda f: t))
false = (lambda t: (lambda f: f))
_if = (lambda b: (lambda t: (lambda e: b(t)(e))))
_and = (lambda p: (lambda q: p(q)(p)))
_not = (lambda b: b(false)(true))

# --- assertions ---
_check('1', decodeInt(S(K)(K)(encodeInt(1))), 'assert 1')
_check('0', decodeInt(zero), 'assert 2')
_check('3', decodeInt(succ(encodeInt(2))), 'assert 3')
_check('2', decodeInt(add(encodeInt(1))(encodeInt(1))), 'assert 4')
_check('true', decodeBool(_and(true)(true)), 'assert 5')
_check('false', decodeBool(_and(true)(false)), 'assert 6')
_check('false', decodeBool(_if(false)(true)(false)), 'assert 7')
_check('true', decodeBool(_not(false)), 'assert 8')

_finish()
