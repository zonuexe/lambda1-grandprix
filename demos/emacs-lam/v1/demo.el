;;; -*- lexical-binding: t; -*-
(load-file "lam1.el")  ; ヘルパー＋λマクロは lam1.el

;; --- definitions ---
(setq I (λ x x))
(setq K (λ x (λ y x)))
(setq S (λ x (λ y (λ z ($ x z ($ y z))))))
(setq zero (λ f (λ x x)))
(setq succ (λ n (λ f (λ x ($ f ($ n f x))))))
(setq add (λ m (λ n (λ f (λ x ($ m f ($ n f x)))))))
(setq true (λ _t (λ f _t)))
(setq false (λ _t (λ f f)))
(setq if (λ b (λ _t (λ e ($ b _t e)))))
(setq and (λ p (λ q ($ p q p))))
(setq not (λ b ($ b false true)))

;; --- assertions ---
(_check "1" (decodeInt ($ S K K (encodeInt 1))) "assert 1")
(_check "0" (decodeInt zero) "assert 2")
(_check "3" (decodeInt ($ succ (encodeInt 2))) "assert 3")
(_check "2" (decodeInt ($ add (encodeInt 1) (encodeInt 1))) "assert 4")
(_check "true" (decodeBool ($ and true true)) "assert 5")
(_check "false" (decodeBool ($ and true false)) "assert 6")
(_check "false" (decodeBool ($ if false true false)) "assert 7")
(_check "true" (decodeBool ($ not false)) "assert 8")

(_finish)
