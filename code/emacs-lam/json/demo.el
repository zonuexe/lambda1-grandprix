;;; -*- lexical-binding: t; -*-
(load-file "lam1.el")  ; ヘルパー＋λマクロは lam1.el

;; --- definitions ---
(setq pair (λ a (λ b (λ s ($ s a b)))))
(setq _nil (λ n (λ c n)))
(setq cons (λ h (λ _t (λ n (λ c ($ c h _t))))))
(setq true (λ _t (λ f _t)))
(setq false (λ _t (λ f f)))
(setq snd (λ p ($ p false)))
(setq one (λ f (λ x ($ f x))))
(setq pred (λ n (λ f (λ x ($ n (λ g (λ h ($ h ($ g f)))) (λ u x) (λ u u))))))
(setq tint (λ k ($ pair one k)))
(setq step (λ p ($ p (λ k (λ l ($ pair ($ pred k) ($ cons ($ tint k) l)))))))
(setq range (λ n ($ snd ($ n step ($ pair n _nil)))))

;; --- assertions ---
(_check "1" (decodeJson (jInt 1)) "assert 1")
(_check "true" (decodeJson (jBool true)) "assert 2")
(_check "false" (decodeJson (jBool false)) "assert 3")
(_check "\"hi\"" (decodeJson (jStr "hi")) "assert 4")
(_check "null" (decodeJson (jNull)) "assert 5")
(_check "[1,true]" (decodeJson (jArr ($ cons (jInt 1) ($ cons (jBool true) _nil)))) "assert 6")
(_check "{\"k\":1}" (decodeJson (jObj ($ cons ($ pair (jStr "k") (jInt 1)) _nil))) "assert 7")
(_check "[1,[2,3]]" (decodeJson (jArr ($ cons (jInt 1) ($ cons (jArr ($ cons (jInt 2) ($ cons (jInt 3) _nil))) _nil)))) "assert 8")
(_check "[]" (decodeJson (jArr ($ range (encodeInt 0)))) "assert 9")
(_check "[1]" (decodeJson (jArr ($ range (encodeInt 1)))) "assert 10")
(_check "[1,2,3]" (decodeJson (jArr ($ range (encodeInt 3)))) "assert 11")

(_finish)
