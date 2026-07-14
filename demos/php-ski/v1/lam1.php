<?php
// λ-1 translator — PHP (遅延 SKI / call-by-name) prelude
//
// Lazy K 方式: DSL の λ は bracket abstraction で S/K/I へ除去され、名前付き定義は
// 参照箇所へインライン展開される。生成コードは S()/K()/I() コンビネータの適用のみで、
// DSL 定義に対応するホスト変数（$pair 等）は一切現れない。
//
// PHP は正格なので、そのまま適用すると bracket abstraction で生じるサンク（`λu.M`→`K M`）が
// 先行評価され、Z 不動点などが発散する。そこで **call-by-name（遅延）** で評価する:
//   - 適用の実引数は必ず「メモ化サンク」`_t(fn()=>…)` として渡す（生成側も同じ）。
//   - コンビネータ／境界関数は引数をサンクで受け取り、必要な時だけ `$thunk()` で force する。
// これで K は第2引数を決して force せず、Z のサンクが保たれる（＝Lazy K と同じ遅延意味論）。

// メモ化サンク（call-by-need）。
function _t($f) {
    $done = false;
    $val = null;
    return function () use (&$done, &$val, $f) {
        if (!$done) { $val = $f(); $done = true; }
        return $val;
    };
}
// 遅延値 $f を eager 値 $x へ適用（境界ヘルパーが church 値を適用する時に使う）。
function _ap($f, $x) { return $f(_t(fn () => $x)); }

// 遅延 S/K/I（引数はサンク。force は $thunk()）。
function I() { return fn ($x) => $x(); }
function K() { return fn ($x) => fn ($y) => $x(); }
function S() { return fn ($x) => fn ($y) => fn ($z) => (($x())($z))(_t(fn () => (($y())($z)))); }

$GLOBALS['failures'] = 0;

function encodeInt($n) {            // host int -> 遅延チャーチ数
    return fn ($f) => fn ($x) => (function () use ($f, $x, $n) {
        $acc = $x;                              // サンク
        for ($i = 0; $i < $n; $i++) {
            $g = $acc;
            $acc = _t(fn () => ($f())($g));      // force f、サンク acc へ適用
        }
        return $acc();
    })();
}
function decodeInt($t) {            // 遅延チャーチ数 -> 文字列
    $succ = fn ($k) => $k() + 1;
    return (string)(($t(_t(fn () => $succ)))(_t(fn () => 0)));
}
function decodeBool($t) {           // 遅延チャーチ真偽値 -> "true"/"false"
    return ($t(_t(fn () => 'true')))(_t(fn () => 'false'));
}
function _check($a, $b, $label) {
    if ($a === $b) {
        echo "ok   $label\n";
    } else {
        $GLOBALS['failures']++;
        echo "FAIL $label: " . var_export($a, true) . " != " . var_export($b, true) . "\n";
    }
}
function _finish() {
    if ($GLOBALS['failures'] > 0) {
        echo $GLOBALS['failures'] . " failure(s)\n";
        exit(1);
    }
    echo "all green\n";
}

// ============ JSON 層（型付き値。ADR-0005）遅延版 ============
function _mkpair($a, $b) {
    return fn ($s) => _ap(_ap($s(), $a), $b);          // λs. s a b
}
function _fstp($p) {
    $k = fn ($a) => fn ($b) => $a();                    // λa.λb.a
    return _ap($p, $k);
}
function _sndp($p) {
    $ki = fn ($a) => fn ($b) => $b();                   // λa.λb.b
    return _ap($p, $ki);
}
function _churchToInt($c) {
    $succ = fn ($k) => $k() + 1;
    return ($c(_t(fn () => $succ)))(_t(fn () => 0));
}
function _boolToHost($c) {
    return ($c(_t(fn () => true)))(_t(fn () => false));
}
function _consH($h, $t) {
    return fn ($n) => fn ($c) => _ap(_ap($c(), $h), $t); // λn.λc. c h t
}
function _isNil($lst) {
    $ctrue = fn ($t) => fn ($f) => $t();
    $conscase = fn ($h) => fn ($t) => (fn ($tt) => fn ($ff) => $ff());
    return _boolToHost(_ap(_ap($lst, $ctrue), $conscase));
}
function _headL($lst) {
    $hc = fn ($h) => fn ($t) => $h();
    return _ap(_ap($lst, ''), $hc);
}
function _tailL($lst) {
    $tc = fn ($h) => fn ($t) => $t();
    return _ap(_ap($lst, ''), $tc);
}
function _walk($lst) {
    $out = [];
    while (!_isNil($lst)) {
        $out[] = _headL($lst);
        $lst = _tailL($lst);
    }
    return $out;
}

function jInt($n)  { return _mkpair(encodeInt(1), encodeInt($n)); }
function jBool($b) { return _mkpair(encodeInt(2), $b); }
function jStr($s) {
    $lst = fn ($n) => fn ($c) => $n();                  // nil
    $bytes = array_values(unpack('C*', $s));
    for ($i = count($bytes) - 1; $i >= 0; $i--) {
        $lst = _consH(encodeInt($bytes[$i]), $lst);
    }
    return _mkpair(encodeInt(3), $lst);
}
function jArr($lst) { return _mkpair(encodeInt(4), $lst); }
function jObj($lst) { return _mkpair(encodeInt(5), $lst); }
function jNull()    { return _mkpair(encodeInt(6), fn ($x) => $x()); }

function _jsonEscape($s) {
    $out = '"';
    $len = strlen($s);
    for ($i = 0; $i < $len; $i++) {
        $ch = $s[$i];
        if ($ch === '"') {
            $out .= '\\"';
        } elseif ($ch === '\\') {
            $out .= '\\\\';
        } else {
            $out .= $ch;
        }
    }
    return $out . '"';
}

function decodeJson($v) {
    $tag = _churchToInt(_fstp($v));
    $payload = _sndp($v);
    switch ($tag) {
        case 1: return (string) _churchToInt($payload);
        case 2: return _boolToHost($payload) ? 'true' : 'false';
        case 3:
            $str = '';
            foreach (_walk($payload) as $b) { $str .= chr(_churchToInt($b)); }
            return _jsonEscape($str);
        case 4:
            return '[' . implode(',', array_map('decodeJson', _walk($payload))) . ']';
        case 5:
            $parts = [];
            foreach (_walk($payload) as $pr) {
                $parts[] = decodeJson(_fstp($pr)) . ':' . decodeJson(_sndp($pr));
            }
            return '{' . implode(',', $parts) . '}';
        case 6: return 'null';
        default: return '?';
    }
}
