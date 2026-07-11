#lang racket
(require "lam1.rkt")  ; ヘルパーは lam1.rkt

;; --- definitions ---
(define _I (lambda (_x) _x))
(define _K (lambda (_x) (lambda (_y) _x)))
(define _S (lambda (_x) (lambda (_y) (lambda (_z) ((_x _z) (_y _z))))))
(define _zero (lambda (_f) (lambda (_x) _x)))
(define _succ (lambda (_n) (lambda (_f) (lambda (_x) (_f ((_n _f) _x))))))
(define _add (lambda (_m) (lambda (_n) (lambda (_f) (lambda (_x) ((_m _f) ((_n _f) _x)))))))
(define _true (lambda (_t) (lambda (_f) _t)))
(define _false (lambda (_t) (lambda (_f) _f)))
(define _if (lambda (_b) (lambda (_t) (lambda (_e) ((_b _t) _e)))))
(define _and (lambda (_p) (lambda (_q) ((_p _q) _p))))
(define _not (lambda (_b) ((_b _false) _true)))

;; --- assertions ---
(_check "1" (decodeInt (((_S _K) _K) (encodeInt 1))) "assert 1")
(_check "0" (decodeInt _zero) "assert 2")
(_check "3" (decodeInt (_succ (encodeInt 2))) "assert 3")
(_check "2" (decodeInt ((_add (encodeInt 1)) (encodeInt 1))) "assert 4")
(_check "true" (decodeBool ((_and _true) _true)) "assert 5")
(_check "false" (decodeBool ((_and _true) _false)) "assert 6")
(_check "false" (decodeBool (((_if _false) _true) _false)) "assert 7")
(_check "true" (decodeBool (_not _false)) "assert 8")

(_finish)
