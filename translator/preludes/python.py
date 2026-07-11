# λ-1 translator — Python prelude
import sys
sys.setrecursionlimit(1_000_000)  # チャーチ数のデコードは値の深さだけ再帰する

# `from lam1 import *` は _ 始まりを取り込まないので明示する
__all__ = ["encodeInt", "decodeInt", "decodeBool", "_check", "_finish",
           "jInt", "jBool", "jStr", "jArr", "jObj", "jNull", "decodeJson"]

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


# ============ JSON 層（型付き値。ADR-0005） ============
import json as _json

_K = lambda a: lambda b: a
_KI = lambda a: lambda b: b

def _pair(a, b):        # チャーチペア
    return lambda s: s(a)(b)

def _fst(p):
    return p(_K)

def _snd(p):
    return p(_KI)

def _churchToInt(c):    # チャーチ数 -> host int
    return c(lambda k: k + 1)(0)

_cTrue = lambda t: lambda f: t
_cFalse = lambda t: lambda f: f

def _boolToHost(c):
    return c(True)(False)

# Scott リスト（host 側。jStr のバイト列構築用）
_nilH = lambda n: lambda c: n
def _consH(h, t):
    return lambda n: lambda c: c(h)(t)

def _isNil(lst):
    return _boolToHost(lst(_cTrue)(lambda h: lambda t: _cFalse))

def _head(lst):
    return lst(None)(lambda h: lambda t: h)

def _tail(lst):
    return lst(None)(lambda h: lambda t: t)

def _walk(lst):
    out = []
    while not _isNil(lst):
        out.append(_head(lst))
        lst = _tail(lst)
    return out

# --- タグ構築子（tag は encodeInt で作るチャーチ数）---
def jInt(n):   return _pair(encodeInt(1), encodeInt(n))
def jBool(b):  return _pair(encodeInt(2), b)
def jStr(s):
    lst = _nilH
    for byte in reversed(s.encode("utf-8")):
        lst = _consH(encodeInt(byte), lst)
    return _pair(encodeInt(3), lst)
def jArr(lst): return _pair(encodeInt(4), lst)
def jObj(lst): return _pair(encodeInt(5), lst)
def jNull():   return _pair(encodeInt(6), lambda x: x)

# --- 汎用 decode -> JSON 文字列 ---
def decodeJson(v):
    tag = _churchToInt(_fst(v))
    payload = _snd(v)
    if tag == 1:
        return str(_churchToInt(payload))
    if tag == 2:
        return "true" if _boolToHost(payload) else "false"
    if tag == 3:
        bs = bytes(_churchToInt(b) for b in _walk(payload))
        return _json.dumps(bs.decode("utf-8", "surrogateescape"), ensure_ascii=False)
    if tag == 4:
        return "[" + ",".join(decodeJson(x) for x in _walk(payload)) + "]"
    if tag == 5:
        parts = [decodeJson(_fst(pr)) + ":" + decodeJson(_snd(pr)) for pr in _walk(payload)]
        return "{" + ",".join(parts) + "}"
    if tag == 6:
        return "null"
    return "?"
