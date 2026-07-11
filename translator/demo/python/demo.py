from lam1 import *  # ヘルパーは lam1.py

# --- definitions ---
_I = (lambda _x: _x)
_K = (lambda _x: (lambda _y: _x))
_S = (lambda _x: (lambda _y: (lambda _z: _x(_z)(_y(_z)))))
_zero = (lambda _f: (lambda _x: _x))
_succ = (lambda _n: (lambda _f: (lambda _x: _f(_n(_f)(_x)))))
_add = (lambda _m: (lambda _n: (lambda _f: (lambda _x: _m(_f)(_n(_f)(_x))))))
_true = (lambda _t: (lambda _f: _t))
_false = (lambda _t: (lambda _f: _f))
_if = (lambda _b: (lambda _t: (lambda _e: _b(_t)(_e))))
_and = (lambda _p: (lambda _q: _p(_q)(_p)))
_not = (lambda _b: _b(_false)(_true))

# --- assertions ---
_check('1', decodeInt(_S(_K)(_K)(encodeInt(1))), 'assert 1')
_check('0', decodeInt(_zero), 'assert 2')
_check('3', decodeInt(_succ(encodeInt(2))), 'assert 3')
_check('2', decodeInt(_add(encodeInt(1))(encodeInt(1))), 'assert 4')
_check('true', decodeBool(_and(_true)(_true)), 'assert 5')
_check('false', decodeBool(_and(_true)(_false)), 'assert 6')
_check('false', decodeBool(_if(_false)(_true)(_false)), 'assert 7')
_check('true', decodeBool(_not(_false)), 'assert 8')

_finish()
