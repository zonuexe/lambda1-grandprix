# λ-1 translator — Python prelude
import sys
sys.setrecursionlimit(1_000_000)  # チャーチ数のデコードは値の深さだけ再帰する

# `from lam1 import *` は _ 始まりを取り込まないので明示する
__all__ = ["encodeInt", "decodeInt", "decodeBool", "_check", "_finish"]

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
