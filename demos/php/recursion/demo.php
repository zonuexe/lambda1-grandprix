<?php
require __DIR__ . '/lam1.php';  // ヘルパーは lam1.php

// --- definitions ---
$Z = (fn($f) => ((fn($x) => ($f)((fn($v) => (($x)($x))($v)))))((fn($x) => ($f)((fn($v) => (($x)($x))($v))))));
$one = (fn($f) => (fn($x) => ($f)($x)));
$mult = (fn($m) => (fn($n) => (fn($f) => ($m)(($n)($f)))));
$pred = (fn($n) => (fn($f) => (fn($x) => ((($n)((fn($g) => (fn($h) => ($h)(($g)($f))))))((fn($u) => $x)))((fn($u) => $u)))));
$true = (fn($t) => (fn($f) => $t));
$false = (fn($t) => (fn($f) => $f));
$isZero = (fn($n) => (($n)((fn($x) => $false)))($true));
$fstep = (fn($rec) => (fn($n) => (((($isZero)($n))((fn($u) => $one)))((fn($u) => (($mult)($n))(($rec)(($pred)($n))))))($n)));
$fact = ($Z)($fstep);

// --- assertions ---
_check('1', decodeInt(($fact)(encodeInt(0))), 'assert 1');
_check('1', decodeInt(($fact)(encodeInt(1))), 'assert 2');
_check('2', decodeInt(($fact)(encodeInt(2))), 'assert 3');
_check('6', decodeInt(($fact)(encodeInt(3))), 'assert 4');
_check('120', decodeInt(($fact)(encodeInt(5))), 'assert 5');

_finish();
