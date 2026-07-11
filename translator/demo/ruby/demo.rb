# λ-1 translator — Ruby prelude
$failures = 0

def encodeInt(n)               # host int -> チャーチ数
  ->(f) { ->(x) { acc = x; n.times { acc = f.(acc) }; acc } }
end

def decodeInt(t)               # チャーチ数 -> 文字列
  t.(->(k) { k + 1 }).(0).to_s
end

def decodeBool(t)              # チャーチ真偽値 -> "true"/"false"
  t.('true').('false')
end

def _check(a, b, label)
  if a == b
    puts "ok   #{label}"
  else
    $failures += 1
    puts "FAIL #{label}: #{a.inspect} != #{b.inspect}"
  end
end

def _finish
  if $failures > 0
    puts "#{$failures} failure(s)"
    exit 1
  end
  puts "all green"
end

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
