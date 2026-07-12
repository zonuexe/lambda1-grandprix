;;; -*- lexical-binding: t; -*-
(load-file "lam1.el")  ; ヘルパーは lam1.el

;; --- definitions ---
(setq I (lambda (x) x))
(setq K (lambda (x) (lambda (y) x)))
(setq S (lambda (x) (lambda (y) (lambda (z) (funcall (funcall x z) (funcall y z))))))
(setq zero (lambda (f) (lambda (x) x)))
(setq succ (lambda (n) (lambda (f) (lambda (x) (funcall f (funcall (funcall n f) x))))))
(setq add (lambda (m) (lambda (n) (lambda (f) (lambda (x) (funcall (funcall m f) (funcall (funcall n f) x)))))))
(setq true (lambda (_t) (lambda (f) _t)))
(setq false (lambda (_t) (lambda (f) f)))
(setq if (lambda (b) (lambda (_t) (lambda (e) (funcall (funcall b _t) e)))))
(setq and (lambda (p) (lambda (q) (funcall (funcall p q) p))))
(setq not (lambda (b) (funcall (funcall b false) true)))

;; --- assertions ---
(_check "1" (decodeInt (funcall (funcall (funcall S K) K) (encodeInt 1))) "assert 1")
(_check "0" (decodeInt zero) "assert 2")
(_check "3" (decodeInt (funcall succ (encodeInt 2))) "assert 3")
(_check "2" (decodeInt (funcall (funcall add (encodeInt 1)) (encodeInt 1))) "assert 4")
(_check "true" (decodeBool (funcall (funcall and true) true)) "assert 5")
(_check "false" (decodeBool (funcall (funcall and true) false)) "assert 6")
(_check "false" (decodeBool (funcall (funcall (funcall if false) true) false)) "assert 7")
(_check "true" (decodeBool (funcall not false)) "assert 8")

(_finish)
