;;; -*- lexical-binding: t; -*-
(load-file "lam1.el")  ; ヘルパー＋λマクロは lam1.el

;; --- definitions ---
(setq Z (λ f ($ (λ x ($ f (λ v ($ x x v)))) (λ x ($ f (λ v ($ x x v)))))))
(setq one (λ f (λ x ($ f x))))
(setq mult (λ m (λ n (λ f ($ m ($ n f))))))
(setq pred (λ n (λ f (λ x ($ n (λ g (λ h ($ h ($ g f)))) (λ u x) (λ u u))))))
(setq true (λ _t (λ f _t)))
(setq false (λ _t (λ f f)))
(setq isZero (λ n ($ n (λ x false) true)))
(setq fstep (λ rec (λ n ($ isZero n (λ u one) (λ u ($ mult n ($ rec ($ pred n)))) n))))
(setq fact ($ Z fstep))

;; --- assertions ---
(_check "1" (decodeInt ($ fact (encodeInt 0))) "assert 1")
(_check "1" (decodeInt ($ fact (encodeInt 1))) "assert 2")
(_check "2" (decodeInt ($ fact (encodeInt 2))) "assert 3")
(_check "6" (decodeInt ($ fact (encodeInt 3))) "assert 4")
(_check "120" (decodeInt ($ fact (encodeInt 5))) "assert 5")

(_finish)
