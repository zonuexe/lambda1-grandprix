;; λ-1 translator — Clojure prelude（λ マクロ変種）

;; Clojure の #(...) は入れ子不可でカリー化に使えないため、マクロで短縮 λ を自作:
;;   (λ x body) => (fn [x] body)
(defmacro λ [x body] (list 'fn [x] body))

(def _failures (atom 0))

(defn encodeInt [n]              ; host int -> チャーチ数
  (λ f (λ x (reduce (fn [acc _] (f acc)) x (range n)))))

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
