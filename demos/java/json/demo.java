// λ-1 translator — Java prelude (single-file source launch)
import java.util.function.Function;

class Main {
    static int _failures = 0;

    static final D pair = new Fun(a -> new Fun(b -> new Fun(s -> s.apply(a).apply(b))));
    static final D nil = new Fun(n -> new Fun(c -> n));
    static final D cons = new Fun(h -> new Fun(t -> new Fun(n -> new Fun(c -> c.apply(h).apply(t)))));
    static final D _true = new Fun(t -> new Fun(f -> t));
    static final D _false = new Fun(t -> new Fun(f -> f));
    static final D snd = new Fun(p -> p.apply(_false));
    static final D one = new Fun(f -> new Fun(x -> f.apply(x)));
    static final D pred = new Fun(n -> new Fun(f -> new Fun(x -> n.apply(new Fun(g -> new Fun(h -> h.apply(g.apply(f))))).apply(new Fun(u -> x)).apply(new Fun(u -> u)))));
    static final D tint = new Fun(k -> pair.apply(one).apply(k));
    static final D step = new Fun(p -> p.apply(new Fun(k -> new Fun(l -> pair.apply(pred.apply(k)).apply(cons.apply(tint.apply(k)).apply(l))))));
    static final D range = new Fun(n -> snd.apply(n.apply(step).apply(pair.apply(n).apply(nil))));

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

    // ============ JSON 層（型付き値。ADR-0005） ============
    static final D jK = new Fun(a -> new Fun(b -> a));
    static final D jKI = new Fun(a -> new Fun(b -> b));
    static D mkpair(D a, D b) { return new Fun(s -> s.apply(a).apply(b)); }
    static D fstp(D p) { return p.apply(jK); }
    static D sndp(D p) { return p.apply(jKI); }
    static int churchToInt(D c) {
        D r = c.apply(new Fun(v -> new Num(((Num) v).n + 1))).apply(new Num(0));
        return ((Num) r).n;
    }
    static final D cTrue = new Fun(t -> new Fun(f -> t));
    static final D cFalse = new Fun(t -> new Fun(f -> f));
    static boolean boolToHost(D c) {
        return ((Str) c.apply(new Str("T")).apply(new Str("F"))).s.equals("T");
    }
    static final D nilH = new Fun(n -> new Fun(c -> n));
    static D consH(D h, D t) { return new Fun(n -> new Fun(c -> c.apply(h).apply(t))); }
    static boolean isNil(D lst) {
        return boolToHost(lst.apply(cTrue).apply(new Fun(h -> new Fun(t -> cFalse))));
    }
    static D headL(D lst) { return lst.apply(new Str("")).apply(new Fun(h -> new Fun(t -> h))); }
    static D tailL(D lst) { return lst.apply(new Str("")).apply(new Fun(h -> new Fun(t -> t))); }
    static java.util.List<D> walkL(D lst) {
        java.util.List<D> out = new java.util.ArrayList<>();
        while (!isNil(lst)) { out.add(headL(lst)); lst = tailL(lst); }
        return out;
    }
    static D jInt(int n) { return mkpair(encodeInt(1), encodeInt(n)); }
    static D jBool(D b) { return mkpair(encodeInt(2), b); }
    static D jStr(String s) {
        D lst = nilH;
        byte[] bytes = s.getBytes(java.nio.charset.StandardCharsets.UTF_8);
        for (int i = bytes.length - 1; i >= 0; i--) lst = consH(encodeInt(bytes[i] & 0xFF), lst);
        return mkpair(encodeInt(3), lst);
    }
    static D jArr(D lst) { return mkpair(encodeInt(4), lst); }
    static D jObj(D lst) { return mkpair(encodeInt(5), lst); }
    static D jNull() { return mkpair(encodeInt(6), new Fun(x -> x)); }
    static String jsonEscape(String s) {
        StringBuilder sb = new StringBuilder("\"");
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == '"') sb.append("\\\"");
            else if (c == '\\') sb.append("\\\\");
            else sb.append(c);
        }
        return sb.append("\"").toString();
    }
    static String decodeJson(D v) {
        int tag = churchToInt(fstp(v));
        D payload = sndp(v);
        switch (tag) {
            case 1: return String.valueOf(churchToInt(payload));
            case 2: return boolToHost(payload) ? "true" : "false";
            case 3: {
                java.util.List<D> bs = walkL(payload);
                byte[] arr = new byte[bs.size()];
                for (int i = 0; i < bs.size(); i++) arr[i] = (byte) churchToInt(bs.get(i));
                return jsonEscape(new String(arr, java.nio.charset.StandardCharsets.UTF_8));
            }
            case 4: {
                StringBuilder sb = new StringBuilder("[");
                java.util.List<D> xs = walkL(payload);
                for (int i = 0; i < xs.size(); i++) { if (i > 0) sb.append(","); sb.append(decodeJson(xs.get(i))); }
                return sb.append("]").toString();
            }
            case 5: {
                StringBuilder sb = new StringBuilder("{");
                java.util.List<D> xs = walkL(payload);
                for (int i = 0; i < xs.size(); i++) {
                    if (i > 0) sb.append(",");
                    D pr = xs.get(i);
                    sb.append(decodeJson(fstp(pr))).append(":").append(decodeJson(sndp(pr)));
                }
                return sb.append("}").toString();
            }
            case 6: return "null";
            default: return "?";
        }
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
        check("assert 1", "1", decodeJson(jInt(1)));
        check("assert 2", "true", decodeJson(jBool(_true)));
        check("assert 3", "false", decodeJson(jBool(_false)));
        check("assert 4", "\"hi\"", decodeJson(jStr("hi")));
        check("assert 5", "null", decodeJson(jNull()));
        check("assert 6", "[1,true]", decodeJson(jArr(cons.apply(jInt(1)).apply(cons.apply(jBool(_true)).apply(nil)))));
        check("assert 7", "{\"k\":1}", decodeJson(jObj(cons.apply(pair.apply(jStr("k")).apply(jInt(1))).apply(nil))));
        check("assert 8", "[1,[2,3]]", decodeJson(jArr(cons.apply(jInt(1)).apply(cons.apply(jArr(cons.apply(jInt(2)).apply(cons.apply(jInt(3)).apply(nil)))).apply(nil)))));
        check("assert 9", "[]", decodeJson(jArr(range.apply(encodeInt(0)))));
        check("assert 10", "[1]", decodeJson(jArr(range.apply(encodeInt(1)))));
        check("assert 11", "[1,2,3]", decodeJson(jArr(range.apply(encodeInt(3)))));
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
