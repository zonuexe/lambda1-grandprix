require_relative 'lam1'  # ヘルパーは lam1.rb

# --- definitions ---
_Z = (->(_f) { (->(_x) { _f.((->(_v) { _x.(_x).(_v) })) }).((->(_x) { _f.((->(_v) { _x.(_x).(_v) })) })) })
_one = (->(_f) { (->(_x) { _f.(_x) }) })
_mult = (->(_m) { (->(_n) { (->(_f) { _m.(_n.(_f)) }) }) })
_pred = (->(_n) { (->(_f) { (->(_x) { _n.((->(_g) { (->(_h) { _h.(_g.(_f)) }) })).((->(_u) { _x })).((->(_u) { _u })) }) }) })
_true = (->(_t) { (->(_f) { _t }) })
_false = (->(_t) { (->(_f) { _f }) })
_isZero = (->(_n) { _n.((->(_x) { _false })).(_true) })
_fstep = (->(_rec) { (->(_n) { _isZero.(_n).((->(_u) { _one })).((->(_u) { _mult.(_n).(_rec.(_pred.(_n))) })).(_n) }) })
_fact = _Z.(_fstep)

# --- assertions ---
_check('1', decodeInt(_fact.(encodeInt(0))), 'assert 1')
_check('1', decodeInt(_fact.(encodeInt(1))), 'assert 2')
_check('2', decodeInt(_fact.(encodeInt(2))), 'assert 3')
_check('6', decodeInt(_fact.(encodeInt(3))), 'assert 4')
_check('120', decodeInt(_fact.(encodeInt(5))), 'assert 5')

_finish
