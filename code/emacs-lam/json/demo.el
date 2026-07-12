;;; -*- lexical-binding: t; -*-
(load-file "lam1.el")  ; ヘルパー＋λマクロは lam1.el

;; --- definitions ---
(setq pair (λ a (λ b (λ s (funcall (funcall s a) b)))))
(setq _nil (λ n (λ c n)))
(setq cons (λ h (λ _t (λ n (λ c (funcall (funcall c h) _t))))))
(setq true (λ _t (λ f _t)))
(setq false (λ _t (λ f f)))
(setq snd (λ p (funcall p false)))
(setq one (λ f (λ x (funcall f x))))
(setq pred (λ n (λ f (λ x (funcall (funcall (funcall n (λ g (λ h (funcall h (funcall g f))))) (λ u x)) (λ u u))))))
(setq tint (λ k (funcall (funcall pair one) k)))
(setq step (λ p (funcall p (λ k (λ l (funcall (funcall pair (funcall pred k)) (funcall (funcall cons (funcall tint k)) l)))))))
(setq range (λ n (funcall snd (funcall (funcall n step) (funcall (funcall pair n) _nil)))))

;; --- assertions ---
(_check "1" (decodeJson (jInt 1)) "assert 1")
(_check "true" (decodeJson (jBool true)) "assert 2")
(_check "false" (decodeJson (jBool false)) "assert 3")
(_check "\"hi\"" (decodeJson (jStr "hi")) "assert 4")
(_check "null" (decodeJson (jNull)) "assert 5")
(_check "[1,true]" (decodeJson (jArr (funcall (funcall cons (jInt 1)) (funcall (funcall cons (jBool true)) _nil)))) "assert 6")
(_check "{\"k\":1}" (decodeJson (jObj (funcall (funcall cons (funcall (funcall pair (jStr "k")) (jInt 1))) _nil))) "assert 7")
(_check "[1,[2,3]]" (decodeJson (jArr (funcall (funcall cons (jInt 1)) (funcall (funcall cons (jArr (funcall (funcall cons (jInt 2)) (funcall (funcall cons (jInt 3)) _nil)))) _nil)))) "assert 8")
(_check "[]" (decodeJson (jArr (funcall range (encodeInt 0)))) "assert 9")
(_check "[1]" (decodeJson (jArr (funcall range (encodeInt 1)))) "assert 10")
(_check "[1,2,3]" (decodeJson (jArr (funcall range (encodeInt 3)))) "assert 11")

(_finish)
