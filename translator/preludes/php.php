<?php
// λ-1 translator — PHP prelude
$GLOBALS['failures'] = 0;

function encodeInt($n) {            // host int -> チャーチ数
    return function ($f) use ($n) {
        return function ($x) use ($f, $n) {
            $acc = $x;
            for ($i = 0; $i < $n; $i++) { $acc = $f($acc); }
            return $acc;
        };
    };
}
function decodeInt($t) {            // チャーチ数 -> 文字列
    return (string)(($t(fn($k) => $k + 1))(0));
}
function decodeBool($t) {           // チャーチ真偽値 -> "true"/"false"
    return ($t('true'))('false');
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
