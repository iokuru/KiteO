use kiteo_engine::space::SpaceAnalyzer;

#[test]
fn test_fixed_size_value_indexed_array_yields_a() {
    let code = r#"
    void solve(int n, const vector<int>& a) {
        int freq[1000005] = {};
        for (int i = 0; i < n; i++) {
            freq[a[i]]++;
        }
    }
    "#;

    let sc = SpaceAnalyzer::analyze_source(code);
    assert_eq!(sc.to_string(), "O(A)");
}

#[test]
fn test_fixed_size_input_indexed_array_yields_n() {
    let code = r#"
    void solve(int n) {
        vector<int> dp(n + 1, 0);
        for (int i = 1; i <= n; i++) {
            dp[i] = dp[i - 1] + 1;
        }
    }
    "#;

    let sc = SpaceAnalyzer::analyze_source(code);
    assert_eq!(sc.to_string(), "O(n)");
}

#[test]
fn test_in_place_algorithm_yields_o1() {
    let code = r#"
    int sum(int n, const int* a) {
        int s = 0;
        for (int i = 0; i < n; i++) s += a[i];
        return s;
    }
    "#;

    let sc = SpaceAnalyzer::analyze_source(code);
    assert_eq!(sc.to_string(), "O(1)");
}
