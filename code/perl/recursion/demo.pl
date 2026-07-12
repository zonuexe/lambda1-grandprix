use strict;
use warnings;
use FindBin;
require "$FindBin::Bin/lam1.pl";  # ヘルパーは lam1.pl

# --- definitions ---
my $Z = sub { my ($f) = @_; sub { my ($x) = @_; $f->(sub { my ($v) = @_; $x->($x)->($v) }) }->(sub { my ($x) = @_; $f->(sub { my ($v) = @_; $x->($x)->($v) }) }) };
my $one = sub { my ($f) = @_; sub { my ($x) = @_; $f->($x) } };
my $mult = sub { my ($m) = @_; sub { my ($n) = @_; sub { my ($f) = @_; $m->($n->($f)) } } };
my $pred = sub { my ($n) = @_; sub { my ($f) = @_; sub { my ($x) = @_; $n->(sub { my ($g) = @_; sub { my ($h) = @_; $h->($g->($f)) } })->(sub { my ($u) = @_; $x })->(sub { my ($u) = @_; $u }) } } };
my $true = sub { my ($t) = @_; sub { my ($f) = @_; $t } };
my $false = sub { my ($t) = @_; sub { my ($f) = @_; $f } };
my $isZero = sub { my ($n) = @_; $n->(sub { my ($x) = @_; $false })->($true) };
my $fstep = sub { my ($rec) = @_; sub { my ($n) = @_; $isZero->($n)->(sub { my ($u) = @_; $one })->(sub { my ($u) = @_; $mult->($n)->($rec->($pred->($n))) })->($n) } };
my $fact = $Z->($fstep);

# --- assertions ---
_check('1', decodeInt($fact->(encodeInt(0))), 'assert 1');
_check('1', decodeInt($fact->(encodeInt(1))), 'assert 2');
_check('2', decodeInt($fact->(encodeInt(2))), 'assert 3');
_check('6', decodeInt($fact->(encodeInt(3))), 'assert 4');
_check('120', decodeInt($fact->(encodeInt(5))), 'assert 5');

_finish();
