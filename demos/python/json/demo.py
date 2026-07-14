from lam1 import *  # ヘルパーは lam1.py

# --- definitions ---
pair = (lambda a: (lambda b: (lambda s: s(a)(b))))
nil = (lambda n: (lambda c: n))
cons = (lambda h: (lambda t: (lambda n: (lambda c: c(h)(t)))))
true = (lambda t: (lambda f: t))
false = (lambda t: (lambda f: f))
snd = (lambda p: p(false))
one = (lambda f: (lambda x: f(x)))
pred = (lambda n: (lambda f: (lambda x: n((lambda g: (lambda h: h(g(f)))))((lambda u: x))((lambda u: u)))))
tint = (lambda k: pair(one)(k))
step = (lambda p: p((lambda k: (lambda l: pair(pred(k))(cons(tint(k))(l))))))
range = (lambda n: snd(n(step)(pair(n)(nil))))

# --- assertions ---
_check('1', decodeJson(jInt(1)), 'assert 1')
_check('true', decodeJson(jBool(true)), 'assert 2')
_check('false', decodeJson(jBool(false)), 'assert 3')
_check('"hi"', decodeJson(jStr('hi')), 'assert 4')
_check('null', decodeJson(jNull()), 'assert 5')
_check('[1,true]', decodeJson(jArr(cons(jInt(1))(cons(jBool(true))(nil)))), 'assert 6')
_check('{"k":1}', decodeJson(jObj(cons(pair(jStr('k'))(jInt(1)))(nil))), 'assert 7')
_check('[1,[2,3]]', decodeJson(jArr(cons(jInt(1))(cons(jArr(cons(jInt(2))(cons(jInt(3))(nil))))(nil)))), 'assert 8')
_check('[]', decodeJson(jArr(range(encodeInt(0)))), 'assert 9')
_check('[1]', decodeJson(jArr(range(encodeInt(1)))), 'assert 10')
_check('[1,2,3]', decodeJson(jArr(range(encodeInt(3)))), 'assert 11')

_finish()
