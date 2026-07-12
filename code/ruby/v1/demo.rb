require_relative 'lam1'  # ヘルパーは lam1.rb

# --- definitions ---
_I = (->(_x) { _x })
_K = (->(_x) { (->(_y) { _x }) })
_S = (->(_x) { (->(_y) { (->(_z) { _x.(_z).(_y.(_z)) }) }) })
_zero = (->(_f) { (->(_x) { _x }) })
_succ = (->(_n) { (->(_f) { (->(_x) { _f.(_n.(_f).(_x)) }) }) })
_add = (->(_m) { (->(_n) { (->(_f) { (->(_x) { _m.(_f).(_n.(_f).(_x)) }) }) }) })
_true = (->(_t) { (->(_f) { _t }) })
_false = (->(_t) { (->(_f) { _f }) })
_if = (->(_b) { (->(_t) { (->(_e) { _b.(_t).(_e) }) }) })
_and = (->(_p) { (->(_q) { _p.(_q).(_p) }) })
_not = (->(_b) { _b.(_false).(_true) })

# --- assertions ---
_check('1', decodeInt(_S.(_K).(_K).(encodeInt(1))), 'assert 1')
_check('0', decodeInt(_zero), 'assert 2')
_check('3', decodeInt(_succ.(encodeInt(2))), 'assert 3')
_check('2', decodeInt(_add.(encodeInt(1)).(encodeInt(1))), 'assert 4')
_check('true', decodeBool(_and.(_true).(_true)), 'assert 5')
_check('false', decodeBool(_and.(_true).(_false)), 'assert 6')
_check('false', decodeBool(_if.(_false).(_true).(_false)), 'assert 7')
_check('true', decodeBool(_not.(_false)), 'assert 8')

_finish
