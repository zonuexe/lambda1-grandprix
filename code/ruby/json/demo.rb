require_relative 'lam1'  # ヘルパーは lam1.rb

# --- definitions ---
_pair = (->(_a) { (->(_b) { (->(_s) { _s.(_a).(_b) }) }) })
_nil = (->(_n) { (->(_c) { _n }) })
_cons = (->(_h) { (->(_t) { (->(_n) { (->(_c) { _c.(_h).(_t) }) }) }) })
_true = (->(_t) { (->(_f) { _t }) })
_false = (->(_t) { (->(_f) { _f }) })
_snd = (->(_p) { _p.(_false) })
_one = (->(_f) { (->(_x) { _f.(_x) }) })
_pred = (->(_n) { (->(_f) { (->(_x) { _n.((->(_g) { (->(_h) { _h.(_g.(_f)) }) })).((->(_u) { _x })).((->(_u) { _u })) }) }) })
_tint = (->(_k) { _pair.(_one).(_k) })
_step = (->(_p) { _p.((->(_k) { (->(_l) { _pair.(_pred.(_k)).(_cons.(_tint.(_k)).(_l)) }) })) })
_range = (->(_n) { _snd.(_n.(_step).(_pair.(_n).(_nil))) })

# --- assertions ---
_check('1', decodeJson(jInt(1)), 'assert 1')
_check('true', decodeJson(jBool(_true)), 'assert 2')
_check('false', decodeJson(jBool(_false)), 'assert 3')
_check('"hi"', decodeJson(jStr('hi')), 'assert 4')
_check('null', decodeJson(jNull()), 'assert 5')
_check('[1,true]', decodeJson(jArr(_cons.(jInt(1)).(_cons.(jBool(_true)).(_nil)))), 'assert 6')
_check('{"k":1}', decodeJson(jObj(_cons.(_pair.(jStr('k')).(jInt(1))).(_nil))), 'assert 7')
_check('[1,[2,3]]', decodeJson(jArr(_cons.(jInt(1)).(_cons.(jArr(_cons.(jInt(2)).(_cons.(jInt(3)).(_nil)))).(_nil)))), 'assert 8')
_check('[]', decodeJson(jArr(_range.(encodeInt(0)))), 'assert 9')
_check('[1]', decodeJson(jArr(_range.(encodeInt(1)))), 'assert 10')
_check('[1,2,3]', decodeJson(jArr(_range.(encodeInt(3)))), 'assert 11')

_finish
