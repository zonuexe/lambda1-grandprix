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
