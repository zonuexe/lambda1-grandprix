(load-file "lam1.clj")  ; ヘルパーは lam1.clj

;; --- definitions ---
(def I (fn [x] x))
(def K (fn [x] (fn [y] x)))
(def S (fn [x] (fn [y] (fn [z] ((x z) (y z))))))
(def zero (fn [f] (fn [x] x)))
(def succ (fn [n] (fn [f] (fn [x] (f ((n f) x))))))
(def add (fn [m] (fn [n] (fn [f] (fn [x] ((m f) ((n f) x)))))))
(def _true (fn [t] (fn [f] t)))
(def _false (fn [t] (fn [f] f)))
(def _if (fn [b] (fn [t] (fn [e] ((b t) e)))))
(def _and (fn [p] (fn [q] ((p q) p))))
(def _not (fn [b] ((b _false) _true)))

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
