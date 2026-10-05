use kiteo_engine::analyze;
use kiteo_engine::preprocessor::preprocess;

#[test]
fn test_30_cp_template_macros() {
    let templates = [
        (
            "#define rep(i, a, b) for(int i = (a); i < (b); ++i)\nrep(i, 0, n) { sum += i; }",
            "for(int i = (0); i < (n); ++i)",
        ),
        (
            "#define forn(i, n) for(int i = 0; i < (int)(n); ++i)\nforn(i, n) { a[i] = i; }",
            "for(int i = 0; i < (int)(n); ++i)",
        ),
        (
            "#define rof(i, a, b) for(int i = (b) - 1; i >= (a); --i)\nrof(i, 0, n) { sum++; }",
            "for(int i = (n) - 1; i >= (0); --i)",
        ),
        (
            "#define FOR(i, a, b) for(int i = (a); i <= (b); ++i)\nFOR(i, 1, n) { count++; }",
            "for(int i = (1); i <= (n); ++i)",
        ),
        (
            "#define FORD(i, a, b) for(int i = (a); i >= (b); --i)\nFORD(i, n, 1) { count++; }",
            "for(int i = (n); i >= (1); --i)",
        ),
        (
            "#define REP(i, n) for(int i = 0; i < (n); ++i)\nREP(i, n) { count++; }",
            "for(int i = 0; i < (n); ++i)",
        ),
        (
            "#define PER(i, n) for(int i = (n) - 1; i >= 0; --i)\nPER(i, n) { count++; }",
            "for(int i = (n) - 1; i >= 0; --i)",
        ),
        (
            "#define trav(a, x) for (auto& a : x)\ntrav(val, arr) { sum += val; }",
            "for (auto& val : arr)",
        ),
        ("#define pb push_back\nv.pb(42);", "push_back"),
        ("#define eb emplace_back\nv.eb(1, 2);", "emplace_back"),
        ("#define mp make_pair\nauto p = mp(1, 2);", "make_pair"),
        (
            "#define fi first\n#define se second\nint x = p.fi + p.se;",
            "p.first + p.second",
        ),
        (
            "using ll = long long;\nll val = 100;",
            "long long val = 100;",
        ),
        (
            "typedef long long ll;\nll val = 100;",
            "long long val = 100;",
        ),
        (
            "using pii = pair<int, int>;\npii item;",
            "pair<int, int> item;",
        ),
        ("using vi = vector<int>;\nvi nums;", "vector<int> nums;"),
        (
            "using vll = vector<long long>;\nvll nums;",
            "vector<long long> nums;",
        ),
        (
            "using vvi = vector<vector<int>>;\nvvi matrix;",
            "vector<vector<int>> matrix;",
        ),
        (
            "#define sz(x) (int)(x).size()\nint len = sz(a);",
            "(int)(a).size()",
        ),
        (
            "#pragma GCC optimize(\"O3\")\nint main() {}",
            "int main() {}",
        ),
        ("#include <bits/stdc++.h>\nint main() {}", "int main() {}"),
        (
            "#define fast_io ios_base::sync_with_stdio(false); cin.tie(NULL);\nfast_io",
            "ios_base::sync_with_stdio(false)",
        ),
        (
            "#define bit(x, i) (((x) >> (i)) & 1)\nif (bit(mask, j))",
            "(((mask) >> (j)) & 1)",
        ),
        (
            "#define lowbit(x) ((x) & -(x))\nint l = lowbit(v);",
            "((v) & -(v))",
        ),
        ("#define rsz resize\nv.rsz(n);", "resize"),
        ("#define ft front()\nint x = q.ft;", "front()"),
        ("#define bk back()\nint x = q.bk;", "back()"),
        ("#define ins insert\ns.ins(val);", "insert"),
        ("#define sqr(x) ((x) * (x))\nint y = sqr(5);", "((5) * (5))"),
        (
            "#define max3(a, b, c) max(a, max(b, c))\nint m = max3(1, 2, 3);",
            "max(1, max(2, 3))",
        ),
    ];

    assert_eq!(templates.len(), 30);

    for (i, (code, expected_substr)) in templates.iter().enumerate() {
        let processed = preprocess(code);
        assert!(
            processed.contains(expected_substr),
            "Template #{i} failed! Code: {code}\nProcessed: {processed}\nExpected to contain: {expected_substr}"
        );
    }
}

#[test]
fn test_real_cp_solution_with_template_headers() {
    let code = r#"
    #include <bits/stdc++.h>
    #pragma GCC optimize("O3")
    using namespace std;
    using ll = long long;
    #define rep(i, a, b) for(int i = (a); i < (b); ++i)
    #define pb push_back

    void solve(int n) {
        vector<int> a;
        rep(i, 0, n) {
            a.pb(i);
        }
    }
    "#;

    let res = analyze(code, "cpp");
    assert_eq!(res.tc, "O(n)");
    assert_eq!(res.sc, "O(n)");
}
