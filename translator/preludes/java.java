// λ-1 translator — Java prelude (single-file source launch)
import java.util.function.Function;

class Main {
    static int _failures = 0;

//__DEFS__

    static D encodeInt(int n) {              // host int -> チャーチ数（Fun）
        return new Fun(f -> new Fun(x -> {
            D acc = x;
            for (int i = 0; i < n; i++) acc = f.apply(acc);
            return acc;
        }));
    }

    static String decodeInt(D t) {           // Num を注入して数える
        D incr = new Fun(v -> new Num(((Num) v).n + 1));
        D r = t.apply(incr).apply(new Num(0));
        return String.valueOf(((Num) r).n);
    }

    static String decodeBool(D t) {          // Str を注入
        D r = t.apply(new Str("true")).apply(new Str("false"));
        return ((Str) r).s;
    }

    static void check(String label, String a, String b) {
        if (a.equals(b)) {
            System.out.println("ok   " + label);
        } else {
            _failures++;
            System.out.println("FAIL " + label + ": " + a + " != " + b);
        }
    }

    public static void main(String[] args) {
//__ASSERTS__
        if (_failures > 0) {
            System.out.println(_failures + " failure(s)");
            System.exit(1);
        }
        System.out.println("all green");
    }
}

// 万能型（タグ付き union）: ラムダ世界は Fun のみ、Num/Str は境界のみ。
abstract class D {
    D apply(D x) { throw new RuntimeException("applied a non-function"); }
}
class Fun extends D {
    final Function<D, D> f;
    Fun(Function<D, D> f) { this.f = f; }
    D apply(D x) { return f.apply(x); }
}
class Num extends D {
    final int n;
    Num(int n) { this.n = n; }
}
class Str extends D {
    final String s;
    Str(String s) { this.s = s; }
}
