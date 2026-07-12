<?php
require __DIR__ . '/lam1.php';  // ヘルパーは lam1.php

// --- definitions ---
$_Z = (fn($_f) => ((fn($_x) => ($_f)((fn($_v) => (($_x)($_x))($_v)))))((fn($_x) => ($_f)((fn($_v) => (($_x)($_x))($_v))))));
$_one = (fn($_f) => (fn($_x) => ($_f)($_x)));
$_mult = (fn($_m) => (fn($_n) => (fn($_f) => ($_m)(($_n)($_f)))));
$_pred = (fn($_n) => (fn($_f) => (fn($_x) => ((($_n)((fn($_g) => (fn($_h) => ($_h)(($_g)($_f))))))((fn($_u) => $_x)))((fn($_u) => $_u)))));
$_true = (fn($_t) => (fn($_f) => $_t));
$_false = (fn($_t) => (fn($_f) => $_f));
$_isZero = (fn($_n) => (($_n)((fn($_x) => $_false)))($_true));
$_fstep = (fn($_rec) => (fn($_n) => (((($_isZero)($_n))((fn($_u) => $_one)))((fn($_u) => (($_mult)($_n))(($_rec)(($_pred)($_n))))))($_n)));
$_fact = ($_Z)($_fstep);

// --- assertions ---
_check('1', decodeInt(($_fact)(encodeInt(0))), 'assert 1');
_check('1', decodeInt(($_fact)(encodeInt(1))), 'assert 2');
_check('2', decodeInt(($_fact)(encodeInt(2))), 'assert 3');
_check('6', decodeInt(($_fact)(encodeInt(3))), 'assert 4');
_check('120', decodeInt(($_fact)(encodeInt(5))), 'assert 5');

_finish();
