;;; -*- lexical-binding: t; -*-
(load-file "lam1.el")  ; ヘルパー＋λマクロは lam1.el

;; --- definitions ---
(setq I (λ x x))
(setq K (λ x (λ y x)))
(setq S (λ x (λ y (λ z (funcall (funcall x z) (funcall y z))))))
(setq zero (λ f (λ x x)))
(setq succ (λ n (λ f (λ x (funcall f (funcall (funcall n f) x))))))
(setq add (λ m (λ n (λ f (λ x (funcall (funcall m f) (funcall (funcall n f) x)))))))
(setq true (λ _t (λ f _t)))
(setq false (λ _t (λ f f)))
(setq if (λ b (λ _t (λ e (funcall (funcall b _t) e)))))
(setq and (λ p (λ q (funcall (funcall p q) p))))
(setq not (λ b (funcall (funcall b false) true)))

;; --- assertions ---
(_check "1" (decodeInt (funcall (funcall (funcall S K) K) (encodeInt 1))) "assert 1")
(_check "0" (decodeInt zero) "assert 2")
(_check "3" (decodeInt (funcall succ (encodeInt 2))) "assert 3")
(_check "2" (decodeInt (funcall (funcall add (encodeInt 1)) (encodeInt 1))) "assert 4")
(_check "true" (decodeBool (funcall (funcall and true) true)) "assert 5")
(_check "false" (decodeBool (funcall (funcall and true) false)) "assert 6")
(_check "false" (decodeBool (funcall (funcall (funcall if false) true) false)) "assert 7")
(_check "true" (decodeBool (funcall not false)) "assert 8")

(_finish)
