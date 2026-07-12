from lam1 import *  # ヘルパーは lam1.py

# --- definitions ---
_pair = (lambda _a: (lambda _b: (lambda _s: _s(_a)(_b))))
_nil = (lambda _n: (lambda _c: _n))
_cons = (lambda _h: (lambda _t: (lambda _n: (lambda _c: _c(_h)(_t)))))
_true = (lambda _t: (lambda _f: _t))
_false = (lambda _t: (lambda _f: _f))
_snd = (lambda _p: _p(_false))
_one = (lambda _f: (lambda _x: _f(_x)))
_pred = (lambda _n: (lambda _f: (lambda _x: _n((lambda _g: (lambda _h: _h(_g(_f)))))((lambda _u: _x))((lambda _u: _u)))))
_tint = (lambda _k: _pair(_one)(_k))
_step = (lambda _p: _p((lambda _k: (lambda _l: _pair(_pred(_k))(_cons(_tint(_k))(_l))))))
_range = (lambda _n: _snd(_n(_step)(_pair(_n)(_nil))))

# --- assertions ---
_check('1', decodeJson(jInt(1)), 'assert 1')
_check('true', decodeJson(jBool(_true)), 'assert 2')
_check('false', decodeJson(jBool(_false)), 'assert 3')
_check('"hi"', decodeJson(jStr('hi')), 'assert 4')
_check('null', decodeJson(jNull()), 'assert 5')
_check('[1,true]', decodeJson(jArr(_cons(jInt(1))(_cons(jBool(_true))(_nil)))), 'assert 6')
_check('{"k":1}', decodeJson(jObj(_cons(_pair(jStr('k'))(jInt(1)))(_nil))), 'assert 7')
_check('[1,[2,3]]', decodeJson(jArr(_cons(jInt(1))(_cons(jArr(_cons(jInt(2))(_cons(jInt(3))(_nil))))(_nil)))), 'assert 8')
_check('[]', decodeJson(jArr(_range(encodeInt(0)))), 'assert 9')
_check('[1]', decodeJson(jArr(_range(encodeInt(1)))), 'assert 10')
_check('[1,2,3]', decodeJson(jArr(_range(encodeInt(3)))), 'assert 11')

_finish()
