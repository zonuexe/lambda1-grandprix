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


;; ============ JSON 層（型付き値。ADR-0005） ============
;; ラムダは自作 λ マクロ（単引数）を使う。多引数の内部 fn は fn のまま。
(defn jK [a] (λ b a))
(defn jKI [a] (λ b b))
(defn mkpair [a b] (λ s ((s a) b)))
(defn fstp [p] (p jK))
(defn sndp [p] (p jKI))
(defn churchToInt [c] ((c inc) 0))
(defn jcTrue [t] (λ f t))
(defn jcFalse [t] (λ f f))
(defn boolToHost [c] ((c true) false))
(def nilH (λ n (λ c n)))
(defn consH [h t] (λ n (λ c ((c h) t))))
(defn isNil [lst] (boolToHost ((lst jcTrue) (λ h (λ t jcFalse)))))
(defn headL [lst] ((lst false) (λ h (λ t h))))
(defn tailL [lst] ((lst false) (λ h (λ t t))))
(defn walk [lst]
  (loop [lst lst acc []]
    (if (isNil lst) acc (recur (tailL lst) (conj acc (headL lst))))))

(defn jInt [n] (mkpair (encodeInt 1) (encodeInt n)))
(defn jBool [b] (mkpair (encodeInt 2) b))
(defn jStr [s]
  (mkpair (encodeInt 3)
          (reduce (fn [lst c] (consH (encodeInt c) lst))
                  nilH (reverse (map int s)))))
(defn jArr [lst] (mkpair (encodeInt 4) lst))
(defn jObj [lst] (mkpair (encodeInt 5) lst))
(defn jNull [] (mkpair (encodeInt 6) (λ x x)))

(defn jsonEscape [s]
  (str "\""
       (apply str (map (fn [c]
                         (cond (= c \") "\\\""
                               (= c \\) "\\\\"
                               :else (str c))) s))
       "\""))

(defn decodeJson [v]
  (let [tag (churchToInt (fstp v))
        payload (sndp v)]
    (cond
      (= tag 1) (str (churchToInt payload))
      (= tag 2) (if (boolToHost payload) "true" "false")
      (= tag 3) (jsonEscape (apply str (map char (map churchToInt (walk payload)))))
      (= tag 4) (str "[" (apply str (interpose "," (map decodeJson (walk payload)))) "]")
      (= tag 5) (str "{"
                     (apply str (interpose ","
                       (map (fn [pr] (str (decodeJson (fstp pr)) ":" (decodeJson (sndp pr))))
                            (walk payload))))
                     "}")
      (= tag 6) "null"
      :else "?")))
