(load-file "lam1.clj")  ; ヘルパー＋λマクロは lam1.clj

;; --- definitions ---
(def _I (λ _x _x))
(def _K (λ _x (λ _y _x)))
(def _S (λ _x (λ _y (λ _z ((_x _z) (_y _z))))))
(def _zero (λ _f (λ _x _x)))
(def _succ (λ _n (λ _f (λ _x (_f ((_n _f) _x))))))
(def _add (λ _m (λ _n (λ _f (λ _x ((_m _f) ((_n _f) _x)))))))
(def _true (λ _t (λ _f _t)))
(def _false (λ _t (λ _f _f)))
(def _if (λ _b (λ _t (λ _e ((_b _t) _e)))))
(def _and (λ _p (λ _q ((_p _q) _p))))
(def _not (λ _b ((_b _false) _true)))

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
