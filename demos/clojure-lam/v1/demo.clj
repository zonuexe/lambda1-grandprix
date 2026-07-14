(load-file "lam1.clj")  ; ヘルパー＋λマクロは lam1.clj

;; --- definitions ---
(def I (λ x x))
(def K (λ x (λ y x)))
(def S (λ x (λ y (λ z ((x z) (y z))))))
(def zero (λ f (λ x x)))
(def succ (λ n (λ f (λ x (f ((n f) x))))))
(def add (λ m (λ n (λ f (λ x ((m f) ((n f) x)))))))
(def _true (λ t (λ f t)))
(def _false (λ t (λ f f)))
(def _if (λ b (λ t (λ e ((b t) e)))))
(def _and (λ p (λ q ((p q) p))))
(def _not (λ b ((b _false) _true)))

;; --- assertions ---
(_check "1" (decodeInt (((S K) K) (encodeInt 1))) "assert 1")
(_check "0" (decodeInt zero) "assert 2")
(_check "3" (decodeInt (succ (encodeInt 2))) "assert 3")
(_check "2" (decodeInt ((add (encodeInt 1)) (encodeInt 1))) "assert 4")
(_check "true" (decodeBool ((_and _true) _true)) "assert 5")
(_check "false" (decodeBool ((_and _true) _false)) "assert 6")
(_check "false" (decodeBool (((_if _false) _true) _false)) "assert 7")
(_check "true" (decodeBool (_not _false)) "assert 8")

(_finish)
