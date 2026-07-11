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


;; ============ JSON 層（型付き値。ADR-0005） ============
(defn jK [a] (fn [b] a))
(defn jKI [a] (fn [b] b))
(defn mkpair [a b] (fn [s] ((s a) b)))
(defn fstp [p] (p jK))
(defn sndp [p] (p jKI))
(defn churchToInt [c] ((c inc) 0))
(defn jcTrue [t] (fn [f] t))
(defn jcFalse [t] (fn [f] f))
(defn boolToHost [c] ((c true) false))
(def nilH (fn [n] (fn [c] n)))
(defn consH [h t] (fn [n] (fn [c] ((c h) t))))
(defn isNil [lst] (boolToHost ((lst jcTrue) (fn [h] (fn [t] jcFalse)))))
(defn headL [lst] ((lst false) (fn [h] (fn [t] h))))
(defn tailL [lst] ((lst false) (fn [h] (fn [t] t))))
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
(defn jNull [] (mkpair (encodeInt 6) (fn [x] x)))

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
