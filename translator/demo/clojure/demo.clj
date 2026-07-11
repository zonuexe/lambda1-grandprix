;; λ-1 translator — Clojure prelude
(def _failures (atom 0))

(defn encodeInt [n]              ; host int -> チャーチ数
  (fn [f] (fn [x] (reduce (fn [acc _] (f acc)) x (range n)))))

(defn decodeInt [t]             ; チャーチ数 -> 文字列
  (str ((t inc) 0)))

(defn decodeBool [t]            ; チャーチ真偽値 -> "true"/"false"
  ((t "true") "false"))

(defn _check [a b label]
  (if (= a b)
    (println (str "ok   " label))
    (do (swap! _failures inc)
        (println (str "FAIL " label ": " (pr-str a) " != " (pr-str b))))))

(defn _finish []
  (when (> @_failures 0)
    (println (str @_failures " failure(s)"))
    (System/exit 1))
  (println "all green"))

;; --- definitions ---
(def _I (fn [_x] _x))
(def _K (fn [_x] (fn [_y] _x)))
(def _S (fn [_x] (fn [_y] (fn [_z] ((_x _z) (_y _z))))))
(def _zero (fn [_f] (fn [_x] _x)))
(def _succ (fn [_n] (fn [_f] (fn [_x] (_f ((_n _f) _x))))))
(def _add (fn [_m] (fn [_n] (fn [_f] (fn [_x] ((_m _f) ((_n _f) _x)))))))
(def _true (fn [_t] (fn [_f] _t)))
(def _false (fn [_t] (fn [_f] _f)))
(def _if (fn [_b] (fn [_t] (fn [_e] ((_b _t) _e)))))
(def _and (fn [_p] (fn [_q] ((_p _q) _p))))
(def _not (fn [_b] ((_b _false) _true)))

;; --- assertions ---
(_check "1" (decodeInt (((_S _K) _K) (encodeInt 1))) "assert 1")
(_check "0" (decodeInt _zero) "assert 2")
(_check "3" (decodeInt (_succ (encodeInt 2))) "assert 3")
(_check "2" (decodeInt ((_add (encodeInt 1)) (encodeInt 1))) "assert 4")
(_check "true" (decodeBool ((_and _true) _true)) "assert 5")
(_check "false" (decodeBool ((_and _true) _false)) "assert 6")
(_check "false" (decodeBool (((_if _false) _true) _false)) "assert 7")
(_check "true" (decodeBool (_not _false)) "assert 8")

(_finish)
