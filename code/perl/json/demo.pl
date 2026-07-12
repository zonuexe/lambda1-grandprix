use strict;
use warnings;
use FindBin;
require "$FindBin::Bin/lam1.pl";  # ヘルパーは lam1.pl

# --- definitions ---
my $pair = sub { my ($a) = @_; sub { my ($b) = @_; sub { my ($s) = @_; $s->($a)->($b) } } };
my $nil = sub { my ($n) = @_; sub { my ($c) = @_; $n } };
my $cons = sub { my ($h) = @_; sub { my ($t) = @_; sub { my ($n) = @_; sub { my ($c) = @_; $c->($h)->($t) } } } };
my $true = sub { my ($t) = @_; sub { my ($f) = @_; $t } };
my $false = sub { my ($t) = @_; sub { my ($f) = @_; $f } };
my $snd = sub { my ($p) = @_; $p->($false) };
my $one = sub { my ($f) = @_; sub { my ($x) = @_; $f->($x) } };
my $pred = sub { my ($n) = @_; sub { my ($f) = @_; sub { my ($x) = @_; $n->(sub { my ($g) = @_; sub { my ($h) = @_; $h->($g->($f)) } })->(sub { my ($u) = @_; $x })->(sub { my ($u) = @_; $u }) } } };
my $tint = sub { my ($k) = @_; $pair->($one)->($k) };
my $step = sub { my ($p) = @_; $p->(sub { my ($k) = @_; sub { my ($l) = @_; $pair->($pred->($k))->($cons->($tint->($k))->($l)) } }) };
my $range = sub { my ($n) = @_; $snd->($n->($step)->($pair->($n)->($nil))) };

# --- assertions ---
_check('1', decodeJson(jInt(1)), 'assert 1');
_check('true', decodeJson(jBool($true)), 'assert 2');
_check('false', decodeJson(jBool($false)), 'assert 3');
_check('"hi"', decodeJson(jStr('hi')), 'assert 4');
_check('null', decodeJson(jNull()), 'assert 5');
_check('[1,true]', decodeJson(jArr($cons->(jInt(1))->($cons->(jBool($true))->($nil)))), 'assert 6');
_check('{"k":1}', decodeJson(jObj($cons->($pair->(jStr('k'))->(jInt(1)))->($nil))), 'assert 7');
_check('[1,[2,3]]', decodeJson(jArr($cons->(jInt(1))->($cons->(jArr($cons->(jInt(2))->($cons->(jInt(3))->($nil))))->($nil)))), 'assert 8');
_check('[]', decodeJson(jArr($range->(encodeInt(0)))), 'assert 9');
_check('[1]', decodeJson(jArr($range->(encodeInt(1)))), 'assert 10');
_check('[1,2,3]', decodeJson(jArr($range->(encodeInt(3)))), 'assert 11');

_finish();
