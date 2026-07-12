// λ-1 translator — Java prelude (single-file source launch)
import java.util.function.Function;

class Main {
    static int _failures = 0;

    static final D mult = new Fun(m -> new Fun(n -> new Fun(f -> m.apply(n.apply(f)))));

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
        check("assert 1", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 2", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 3", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 4", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 5", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 6", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 7", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 8", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 9", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 10", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 11", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 12", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 13", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 14", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 15", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 16", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 17", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 18", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 19", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 20", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 21", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 22", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 23", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 24", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 25", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 26", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 27", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 28", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 29", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 30", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 31", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 32", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 33", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 34", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 35", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 36", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 37", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 38", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 39", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 40", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 41", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 42", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 43", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 44", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 45", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 46", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 47", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 48", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 49", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 50", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 51", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 52", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 53", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 54", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 55", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 56", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 57", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 58", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 59", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 60", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 61", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 62", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 63", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 64", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 65", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 66", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 67", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 68", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 69", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 70", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 71", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 72", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 73", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 74", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 75", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 76", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 77", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 78", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 79", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 80", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 81", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 82", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 83", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 84", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 85", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 86", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 87", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 88", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 89", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 90", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 91", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 92", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 93", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 94", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 95", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 96", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 97", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 98", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 99", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 100", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 101", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 102", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 103", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 104", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 105", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 106", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 107", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 108", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 109", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 110", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 111", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 112", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 113", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 114", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 115", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 116", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 117", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 118", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 119", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 120", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 121", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 122", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 123", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 124", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 125", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 126", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 127", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 128", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 129", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 130", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 131", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 132", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 133", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 134", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 135", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 136", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 137", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 138", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 139", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 140", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 141", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 142", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 143", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 144", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 145", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 146", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 147", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 148", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 149", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 150", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 151", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 152", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 153", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 154", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 155", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 156", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 157", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 158", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 159", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 160", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 161", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 162", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 163", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 164", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 165", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 166", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 167", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 168", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 169", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 170", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 171", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 172", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 173", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 174", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 175", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 176", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 177", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 178", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 179", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 180", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 181", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 182", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 183", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 184", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 185", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 186", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 187", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 188", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 189", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 190", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 191", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 192", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 193", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 194", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 195", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 196", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 197", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 198", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 199", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 200", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 201", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 202", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 203", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 204", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 205", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 206", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 207", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 208", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 209", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 210", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 211", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 212", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 213", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 214", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 215", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 216", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 217", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 218", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 219", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 220", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 221", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 222", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 223", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 224", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 225", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 226", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 227", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 228", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 229", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 230", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 231", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 232", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 233", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 234", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 235", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 236", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 237", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 238", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 239", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 240", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 241", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 242", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 243", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 244", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 245", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 246", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 247", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 248", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 249", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 250", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 251", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 252", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 253", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 254", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 255", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 256", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 257", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 258", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 259", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 260", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 261", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 262", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 263", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 264", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 265", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 266", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 267", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 268", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 269", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 270", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 271", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 272", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 273", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 274", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 275", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 276", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 277", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 278", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 279", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 280", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 281", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 282", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 283", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 284", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 285", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 286", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 287", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 288", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 289", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 290", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 291", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 292", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 293", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 294", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 295", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 296", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 297", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 298", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 299", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 300", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 301", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 302", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 303", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 304", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 305", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 306", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 307", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 308", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 309", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 310", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 311", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 312", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 313", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 314", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 315", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 316", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 317", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 318", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 319", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 320", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 321", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 322", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 323", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 324", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 325", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 326", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 327", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 328", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 329", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 330", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 331", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 332", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 333", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 334", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 335", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 336", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 337", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 338", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 339", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 340", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 341", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 342", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 343", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 344", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 345", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 346", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 347", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 348", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 349", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 350", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 351", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 352", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 353", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 354", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 355", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 356", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 357", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 358", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 359", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 360", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 361", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 362", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 363", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 364", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 365", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 366", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 367", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 368", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 369", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 370", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 371", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 372", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 373", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 374", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 375", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 376", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 377", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 378", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 379", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 380", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 381", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 382", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 383", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 384", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 385", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 386", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 387", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 388", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 389", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 390", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 391", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 392", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 393", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 394", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 395", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 396", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 397", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 398", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 399", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 400", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 401", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 402", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 403", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 404", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 405", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 406", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 407", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 408", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 409", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 410", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 411", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 412", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 413", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 414", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 415", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 416", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 417", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 418", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 419", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 420", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 421", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 422", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 423", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 424", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 425", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 426", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 427", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 428", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 429", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 430", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 431", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 432", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 433", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 434", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 435", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 436", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 437", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 438", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 439", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 440", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 441", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 442", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 443", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 444", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 445", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 446", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 447", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 448", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 449", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 450", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 451", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 452", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 453", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 454", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 455", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 456", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 457", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 458", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 459", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 460", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 461", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 462", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 463", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 464", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 465", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 466", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 467", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 468", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 469", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 470", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 471", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 472", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 473", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 474", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 475", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 476", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 477", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 478", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 479", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 480", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 481", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 482", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 483", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 484", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 485", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 486", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 487", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 488", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 489", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 490", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 491", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 492", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 493", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 494", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 495", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 496", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 497", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 498", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 499", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
        check("assert 500", "900", decodeInt(mult.apply(encodeInt(30)).apply(encodeInt(30))));
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
