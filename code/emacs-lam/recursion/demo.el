;;; -*- lexical-binding: t; -*-
(load-file "lam1.el")  ; ヘルパー＋λマクロは lam1.el

;; --- definitions ---
(setq Z (λ f (funcall (λ x (funcall f (λ v (funcall (funcall x x) v)))) (λ x (funcall f (λ v (funcall (funcall x x) v)))))))
(setq one (λ f (λ x (funcall f x))))
(setq mult (λ m (λ n (λ f (funcall m (funcall n f))))))
(setq pred (λ n (λ f (λ x (funcall (funcall (funcall n (λ g (λ h (funcall h (funcall g f))))) (λ u x)) (λ u u))))))
(setq true (λ _t (λ f _t)))
(setq false (λ _t (λ f f)))
(setq isZero (λ n (funcall (funcall n (λ x false)) true)))
(setq fstep (λ rec (λ n (funcall (funcall (funcall (funcall isZero n) (λ u one)) (λ u (funcall (funcall mult n) (funcall rec (funcall pred n))))) n))))
(setq fact (funcall Z fstep))

;; --- assertions ---
(_check "1" (decodeInt (funcall fact (encodeInt 0))) "assert 1")
(_check "1" (decodeInt (funcall fact (encodeInt 1))) "assert 2")
(_check "2" (decodeInt (funcall fact (encodeInt 2))) "assert 3")
(_check "6" (decodeInt (funcall fact (encodeInt 3))) "assert 4")
(_check "120" (decodeInt (funcall fact (encodeInt 5))) "assert 5")

(_finish)
