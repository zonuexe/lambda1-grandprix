(load-file "lam1.clj")  ; ヘルパー＋λマクロは lam1.clj

;; --- definitions ---
(def pair (λ a (λ b (λ s ((s a) b)))))
(def _nil (λ n (λ c n)))
(def cons (λ h (λ t (λ n (λ c ((c h) t))))))
(def _true (λ t (λ f t)))
(def _false (λ t (λ f f)))
(def snd (λ p (p _false)))
(def one (λ f (λ x (f x))))
(def pred (λ n (λ f (λ x (((n (λ g (λ h (h (g f))))) (λ u x)) (λ u u))))))
(def tint (λ k ((pair one) k)))
(def step (λ p (p (λ k (λ l ((pair (pred k)) ((cons (tint k)) l)))))))
(def range (λ n (snd ((n step) ((pair n) _nil)))))

;; --- assertions ---
(_check "1" (decodeJson (jInt 1)) "assert 1")
(_check "true" (decodeJson (jBool _true)) "assert 2")
(_check "false" (decodeJson (jBool _false)) "assert 3")
(_check "\"hi\"" (decodeJson (jStr "hi")) "assert 4")
(_check "null" (decodeJson (jNull)) "assert 5")
(_check "[1,true]" (decodeJson (jArr ((cons (jInt 1)) ((cons (jBool _true)) _nil)))) "assert 6")
(_check "{\"k\":1}" (decodeJson (jObj ((cons ((pair (jStr "k")) (jInt 1))) _nil))) "assert 7")
(_check "[1,[2,3]]" (decodeJson (jArr ((cons (jInt 1)) ((cons (jArr ((cons (jInt 2)) ((cons (jInt 3)) _nil)))) _nil)))) "assert 8")
(_check "[]" (decodeJson (jArr (range (encodeInt 0)))) "assert 9")
(_check "[1]" (decodeJson (jArr (range (encodeInt 1)))) "assert 10")
(_check "[1,2,3]" (decodeJson (jArr (range (encodeInt 3)))) "assert 11")

(_finish)
