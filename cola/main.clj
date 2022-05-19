(def names ["Sheldon" "Leonard" "Penny" "Rajesh" "Howard"])

(defn double-group [[size name]]
  [(* 2 size) name])

(defn groups []
  (lazy-seq
    (concat (map #(vector 1 %) names)
                 (map double-group (groups)))))

(defn get-nth [n gs]
  (loop [n n
         [[size name] & rest] gs]
    (if (<= n size)
      name
      (recur (- n size) rest))))
