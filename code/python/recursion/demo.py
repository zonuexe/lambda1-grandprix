from lam1 import *  # ヘルパーは lam1.py

# --- definitions ---
_Z = (lambda _f: (lambda _x: _f((lambda _v: _x(_x)(_v))))((lambda _x: _f((lambda _v: _x(_x)(_v))))))
_one = (lambda _f: (lambda _x: _f(_x)))
_mult = (lambda _m: (lambda _n: (lambda _f: _m(_n(_f)))))
_pred = (lambda _n: (lambda _f: (lambda _x: _n((lambda _g: (lambda _h: _h(_g(_f)))))((lambda _u: _x))((lambda _u: _u)))))
_true = (lambda _t: (lambda _f: _t))
_false = (lambda _t: (lambda _f: _f))
_isZero = (lambda _n: _n((lambda _x: _false))(_true))
_fstep = (lambda _rec: (lambda _n: _isZero(_n)((lambda _u: _one))((lambda _u: _mult(_n)(_rec(_pred(_n)))))(_n)))
_fact = _Z(_fstep)

# --- assertions ---
_check('1', decodeInt(_fact(encodeInt(0))), 'assert 1')
_check('1', decodeInt(_fact(encodeInt(1))), 'assert 2')
_check('2', decodeInt(_fact(encodeInt(2))), 'assert 3')
_check('6', decodeInt(_fact(encodeInt(3))), 'assert 4')
_check('120', decodeInt(_fact(encodeInt(5))), 'assert 5')

_finish()
