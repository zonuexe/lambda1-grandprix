(load-file "lam1.clj")  ; ヘルパー＋λマクロは lam1.clj

;; --- definitions ---
(def Z (λ f ((λ x (f (λ v ((x x) v)))) (λ x (f (λ v ((x x) v)))))))
(def one (λ f (λ x (f x))))
(def mult (λ m (λ n (λ f (m (n f))))))
(def pred (λ n (λ f (λ x (((n (λ g (λ h (h (g f))))) (λ u x)) (λ u u))))))
(def _true (λ t (λ f t)))
(def _false (λ t (λ f f)))
(def isZero (λ n ((n (λ x _false)) _true)))
(def fstep (λ rec (λ n ((((isZero n) (λ u one)) (λ u ((mult n) (rec (pred n))))) n))))
(def fact (Z fstep))

;; --- assertions ---
(_check "1" (decodeInt (fact (encodeInt 0))) "assert 1")
(_check "1" (decodeInt (fact (encodeInt 1))) "assert 2")
(_check "2" (decodeInt (fact (encodeInt 2))) "assert 3")
(_check "6" (decodeInt (fact (encodeInt 3))) "assert 4")
(_check "120" (decodeInt (fact (encodeInt 5))) "assert 5")

(_finish)
