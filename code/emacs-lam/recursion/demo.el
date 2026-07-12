;;; -*- lexical-binding: t; -*-
(load-file "lam1.el")  ; ヘルパー＋λマクロは lam1.el

;; --- definitions ---
(setq _Z (λ _f (funcall (λ _x (funcall _f (λ _v (funcall (funcall _x _x) _v)))) (λ _x (funcall _f (λ _v (funcall (funcall _x _x) _v)))))))
(setq _one (λ _f (λ _x (funcall _f _x))))
(setq _mult (λ _m (λ _n (λ _f (funcall _m (funcall _n _f))))))
(setq _pred (λ _n (λ _f (λ _x (funcall (funcall (funcall _n (λ _g (λ _h (funcall _h (funcall _g _f))))) (λ _u _x)) (λ _u _u))))))
(setq _true (λ _t (λ _f _t)))
(setq _false (λ _t (λ _f _f)))
(setq _isZero (λ _n (funcall (funcall _n (λ _x _false)) _true)))
(setq _fstep (λ _rec (λ _n (funcall (funcall (funcall (funcall _isZero _n) (λ _u _one)) (λ _u (funcall (funcall _mult _n) (funcall _rec (funcall _pred _n))))) _n))))
(setq _fact (funcall _Z _fstep))

;; --- assertions ---
(_check "1" (decodeInt (funcall _fact (encodeInt 0))) "assert 1")
(_check "1" (decodeInt (funcall _fact (encodeInt 1))) "assert 2")
(_check "2" (decodeInt (funcall _fact (encodeInt 2))) "assert 3")
(_check "6" (decodeInt (funcall _fact (encodeInt 3))) "assert 4")
(_check "120" (decodeInt (funcall _fact (encodeInt 5))) "assert 5")

(_finish)
