;;; -*- lexical-binding: t; -*-
(load-file "lam1.el")  ; ヘルパーは lam1.el

;; --- definitions ---
(setq _pair (lambda (_a) (lambda (_b) (lambda (_s) (funcall (funcall _s _a) _b)))))
(setq _nil (lambda (_n) (lambda (_c) _n)))
(setq _cons (lambda (_h) (lambda (_t) (lambda (_n) (lambda (_c) (funcall (funcall _c _h) _t))))))
(setq _true (lambda (_t) (lambda (_f) _t)))
(setq _false (lambda (_t) (lambda (_f) _f)))
(setq _snd (lambda (_p) (funcall _p _false)))
(setq _one (lambda (_f) (lambda (_x) (funcall _f _x))))
(setq _pred (lambda (_n) (lambda (_f) (lambda (_x) (funcall (funcall (funcall _n (lambda (_g) (lambda (_h) (funcall _h (funcall _g _f))))) (lambda (_u) _x)) (lambda (_u) _u))))))
(setq _tint (lambda (_k) (funcall (funcall _pair _one) _k)))
(setq _step (lambda (_p) (funcall _p (lambda (_k) (lambda (_l) (funcall (funcall _pair (funcall _pred _k)) (funcall (funcall _cons (funcall _tint _k)) _l)))))))
(setq _range (lambda (_n) (funcall _snd (funcall (funcall _n _step) (funcall (funcall _pair _n) _nil)))))

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
