;;; -*- lexical-binding: t; -*-
(load-file "lam1.el")  ; ヘルパーは lam1.el

;; --- definitions ---
(setq Z (lambda (f) (funcall (lambda (x) (funcall f (lambda (v) (funcall (funcall x x) v)))) (lambda (x) (funcall f (lambda (v) (funcall (funcall x x) v)))))))
(setq one (lambda (f) (lambda (x) (funcall f x))))
(setq mult (lambda (m) (lambda (n) (lambda (f) (funcall m (funcall n f))))))
(setq pred (lambda (n) (lambda (f) (lambda (x) (funcall (funcall (funcall n (lambda (g) (lambda (h) (funcall h (funcall g f))))) (lambda (u) x)) (lambda (u) u))))))
(setq true (lambda (_t) (lambda (f) _t)))
(setq false (lambda (_t) (lambda (f) f)))
(setq isZero (lambda (n) (funcall (funcall n (lambda (x) false)) true)))
(setq fstep (lambda (rec) (lambda (n) (funcall (funcall (funcall (funcall isZero n) (lambda (u) one)) (lambda (u) (funcall (funcall mult n) (funcall rec (funcall pred n))))) n))))
(setq fact (funcall Z fstep))

;; --- assertions ---
(_check "1" (decodeInt (funcall fact (encodeInt 0))) "assert 1")
(_check "1" (decodeInt (funcall fact (encodeInt 1))) "assert 2")
(_check "2" (decodeInt (funcall fact (encodeInt 2))) "assert 3")
(_check "6" (decodeInt (funcall fact (encodeInt 3))) "assert 4")
(_check "120" (decodeInt (funcall fact (encodeInt 5))) "assert 5")

(_finish)
