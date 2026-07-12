(load-file "lam1.clj")  ; ヘルパー＋λマクロは lam1.clj

;; --- definitions ---
(def _Z (λ _f ((λ _x (_f (λ _v ((_x _x) _v)))) (λ _x (_f (λ _v ((_x _x) _v)))))))
(def _one (λ _f (λ _x (_f _x))))
(def _mult (λ _m (λ _n (λ _f (_m (_n _f))))))
(def _pred (λ _n (λ _f (λ _x (((_n (λ _g (λ _h (_h (_g _f))))) (λ _u _x)) (λ _u _u))))))
(def _true (λ _t (λ _f _t)))
(def _false (λ _t (λ _f _f)))
(def _isZero (λ _n ((_n (λ _x _false)) _true)))
(def _fstep (λ _rec (λ _n ((((_isZero _n) (λ _u _one)) (λ _u ((_mult _n) (_rec (_pred _n))))) _n))))
(def _fact (_Z _fstep))

;; --- assertions ---
(_check "1" (decodeInt (_fact (encodeInt 0))) "assert 1")
(_check "1" (decodeInt (_fact (encodeInt 1))) "assert 2")
(_check "2" (decodeInt (_fact (encodeInt 2))) "assert 3")
(_check "6" (decodeInt (_fact (encodeInt 3))) "assert 4")
(_check "120" (decodeInt (_fact (encodeInt 5))) "assert 5")

(_finish)
