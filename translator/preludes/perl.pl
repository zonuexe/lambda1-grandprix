use strict;
use warnings;
# λ-1 translator — Perl prelude
my $failures = 0;

sub encodeInt {                    # host int -> チャーチ数
    my ($n) = @_;
    return sub {
        my ($f) = @_;
        return sub {
            my ($x) = @_;
            my $acc = $x;
            for (1 .. $n) { $acc = $f->($acc); }
            return $acc;
        };
    };
}

sub decodeInt {                    # チャーチ数 -> 文字列
    my ($t) = @_;
    return "" . $t->(sub { $_[0] + 1 })->(0);
}

sub decodeBool {                   # チャーチ真偽値 -> "true"/"false"
    my ($t) = @_;
    return $t->('true')->('false');
}

sub _check {
    my ($a, $b, $label) = @_;
    if ($a eq $b) {
        print "ok   $label\n";
    } else {
        $failures++;
        print "FAIL $label: $a != $b\n";
    }
}

sub _finish {
    if ($failures > 0) {
        print "$failures failure(s)\n";
        exit 1;
    }
    print "all green\n";
}
