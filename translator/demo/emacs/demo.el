;;; -*- lexical-binding: t; -*-
(load-file "lam1.el")  ; ヘルパーは lam1.el

;; --- definitions ---
(setq _I (lambda (_x) _x))
(setq _K (lambda (_x) (lambda (_y) _x)))
(setq _S (lambda (_x) (lambda (_y) (lambda (_z) (funcall (funcall _x _z) (funcall _y _z))))))
(setq _zero (lambda (_f) (lambda (_x) _x)))
(setq _succ (lambda (_n) (lambda (_f) (lambda (_x) (funcall _f (funcall (funcall _n _f) _x))))))
(setq _add (lambda (_m) (lambda (_n) (lambda (_f) (lambda (_x) (funcall (funcall _m _f) (funcall (funcall _n _f) _x)))))))
(setq _true (lambda (_t) (lambda (_f) _t)))
(setq _false (lambda (_t) (lambda (_f) _f)))
(setq _if (lambda (_b) (lambda (_t) (lambda (_e) (funcall (funcall _b _t) _e)))))
(setq _and (lambda (_p) (lambda (_q) (funcall (funcall _p _q) _p))))
(setq _not (lambda (_b) (funcall (funcall _b _false) _true)))

;; --- assertions ---
(_check "1" (decodeInt (funcall (funcall (funcall _S _K) _K) (encodeInt 1))) "assert 1")
(_check "0" (decodeInt _zero) "assert 2")
(_check "3" (decodeInt (funcall _succ (encodeInt 2))) "assert 3")
(_check "2" (decodeInt (funcall (funcall _add (encodeInt 1)) (encodeInt 1))) "assert 4")
(_check "true" (decodeBool (funcall (funcall _and _true) _true)) "assert 5")
(_check "false" (decodeBool (funcall (funcall _and _true) _false)) "assert 6")
(_check "false" (decodeBool (funcall (funcall (funcall _if _false) _true) _false)) "assert 7")
(_check "true" (decodeBool (funcall _not _false)) "assert 8")

(_finish)
