;;; -*- lexical-binding: t; -*-
(load-file "lam1.el")  ; ヘルパー＋λマクロは lam1.el

;; --- definitions ---
(setq _pair (λ _a (λ _b (λ _s (funcall (funcall _s _a) _b)))))
(setq _nil (λ _n (λ _c _n)))
(setq _cons (λ _h (λ _t (λ _n (λ _c (funcall (funcall _c _h) _t))))))
(setq _true (λ _t (λ _f _t)))
(setq _false (λ _t (λ _f _f)))
(setq _snd (λ _p (funcall _p _false)))
(setq _one (λ _f (λ _x (funcall _f _x))))
(setq _pred (λ _n (λ _f (λ _x (funcall (funcall (funcall _n (λ _g (λ _h (funcall _h (funcall _g _f))))) (λ _u _x)) (λ _u _u))))))
(setq _tint (λ _k (funcall (funcall _pair _one) _k)))
(setq _step (λ _p (funcall _p (λ _k (λ _l (funcall (funcall _pair (funcall _pred _k)) (funcall (funcall _cons (funcall _tint _k)) _l)))))))
(setq _range (λ _n (funcall _snd (funcall (funcall _n _step) (funcall (funcall _pair _n) _nil)))))

;; --- assertions ---
(_check "1" (decodeJson (jInt 1)) "assert 1")
(_check "true" (decodeJson (jBool _true)) "assert 2")
(_check "false" (decodeJson (jBool _false)) "assert 3")
(_check "\"hi\"" (decodeJson (jStr "hi")) "assert 4")
(_check "null" (decodeJson (jNull)) "assert 5")
(_check "[1,true]" (decodeJson (jArr (funcall (funcall _cons (jInt 1)) (funcall (funcall _cons (jBool _true)) _nil)))) "assert 6")
(_check "{\"k\":1}" (decodeJson (jObj (funcall (funcall _cons (funcall (funcall _pair (jStr "k")) (jInt 1))) _nil))) "assert 7")
(_check "[1,[2,3]]" (decodeJson (jArr (funcall (funcall _cons (jInt 1)) (funcall (funcall _cons (jArr (funcall (funcall _cons (jInt 2)) (funcall (funcall _cons (jInt 3)) _nil)))) _nil)))) "assert 8")
(_check "[]" (decodeJson (jArr (funcall _range (encodeInt 0)))) "assert 9")
(_check "[1]" (decodeJson (jArr (funcall _range (encodeInt 1)))) "assert 10")
(_check "[1,2,3]" (decodeJson (jArr (funcall _range (encodeInt 3)))) "assert 11")

(_finish)
