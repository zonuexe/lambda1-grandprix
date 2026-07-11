# λ-1 translator — Python prelude
import sys
sys.setrecursionlimit(1_000_000)  # チャーチ数のデコードは値の深さだけ再帰する

_failures = 0


def encodeInt(n):
    """host int -> チャーチ数（クロージャ）"""
    def church(f):
        def inner(x):
            r = x
            for _ in range(n):
                r = f(r)
            return r
        return inner
    return church


def decodeInt(t):
    """チャーチ数 -> 文字列表現"""
    return str(t(lambda k: k + 1)(0))


def decodeBool(t):
    """チャーチ真偽値 -> 'true' / 'false'"""
    return t('true')('false')


def _check(a, b, label):
    global _failures
    if a == b:
        print('ok   ' + label)
    else:
        _failures += 1
        print('FAIL ' + label + ': ' + repr(a) + ' != ' + repr(b))


def _finish():
    if _failures:
        print(str(_failures) + ' failure(s)')
        sys.exit(1)
    print('all green')

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
