use strict;
use warnings;
use FindBin;
require "$FindBin::Bin/lam1.pl";  # ヘルパーは lam1.pl

# --- definitions ---
my $I = sub { my ($x) = @_; $x };
my $K = sub { my ($x) = @_; sub { my ($y) = @_; $x } };
my $S = sub { my ($x) = @_; sub { my ($y) = @_; sub { my ($z) = @_; $x->($z)->($y->($z)) } } };
my $zero = sub { my ($f) = @_; sub { my ($x) = @_; $x } };
my $succ = sub { my ($n) = @_; sub { my ($f) = @_; sub { my ($x) = @_; $f->($n->($f)->($x)) } } };
my $add = sub { my ($m) = @_; sub { my ($n) = @_; sub { my ($f) = @_; sub { my ($x) = @_; $m->($f)->($n->($f)->($x)) } } } };
my $true = sub { my ($t) = @_; sub { my ($f) = @_; $t } };
my $false = sub { my ($t) = @_; sub { my ($f) = @_; $f } };
my $if = sub { my ($b) = @_; sub { my ($t) = @_; sub { my ($e) = @_; $b->($t)->($e) } } };
my $and = sub { my ($p) = @_; sub { my ($q) = @_; $p->($q)->($p) } };
my $not = sub { my ($b) = @_; $b->($false)->($true) };

# --- assertions ---
_check('1', decodeInt($S->($K)->($K)->(encodeInt(1))), 'assert 1');
_check('0', decodeInt($zero), 'assert 2');
_check('3', decodeInt($succ->(encodeInt(2))), 'assert 3');
_check('2', decodeInt($add->(encodeInt(1))->(encodeInt(1))), 'assert 4');
_check('true', decodeBool($and->($true)->($true)), 'assert 5');
_check('false', decodeBool($and->($true)->($false)), 'assert 6');
_check('false', decodeBool($if->($false)->($true)->($false)), 'assert 7');
_check('true', decodeBool($not->($false)), 'assert 8');

_finish();
