;;; -*- lexical-binding: t; -*-
(load-file "lam1.el")  ; ヘルパーは lam1.el

;; --- definitions ---
(setq _Z (lambda (_f) (funcall (lambda (_x) (funcall _f (lambda (_v) (funcall (funcall _x _x) _v)))) (lambda (_x) (funcall _f (lambda (_v) (funcall (funcall _x _x) _v)))))))
(setq _one (lambda (_f) (lambda (_x) (funcall _f _x))))
(setq _mult (lambda (_m) (lambda (_n) (lambda (_f) (funcall _m (funcall _n _f))))))
(setq _pred (lambda (_n) (lambda (_f) (lambda (_x) (funcall (funcall (funcall _n (lambda (_g) (lambda (_h) (funcall _h (funcall _g _f))))) (lambda (_u) _x)) (lambda (_u) _u))))))
(setq _true (lambda (_t) (lambda (_f) _t)))
(setq _false (lambda (_t) (lambda (_f) _f)))
(setq _isZero (lambda (_n) (funcall (funcall _n (lambda (_x) _false)) _true)))
(setq _fstep (lambda (_rec) (lambda (_n) (funcall (funcall (funcall (funcall _isZero _n) (lambda (_u) _one)) (lambda (_u) (funcall (funcall _mult _n) (funcall _rec (funcall _pred _n))))) _n))))
(setq _fact (funcall _Z _fstep))

;; --- assertions ---
(_check "1" (decodeInt (funcall _fact (encodeInt 0))) "assert 1")
(_check "1" (decodeInt (funcall _fact (encodeInt 1))) "assert 2")
(_check "2" (decodeInt (funcall _fact (encodeInt 2))) "assert 3")
(_check "6" (decodeInt (funcall _fact (encodeInt 3))) "assert 4")
(_check "120" (decodeInt (funcall _fact (encodeInt 5))) "assert 5")

(_finish)
