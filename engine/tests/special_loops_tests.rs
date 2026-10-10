use kiteo_engine::analyze;

#[test]
fn test_harmonic_series_loop() {
    let code = r#"
    void solve(int n) {
        long long sum = 0;
        for (int i = 1; i <= n; i++) {
            for (int j = i; j <= n; j += i) {
                sum++;
            }
        }
    }
    "#;

    let res = analyze(code, "cpp");
    assert_eq!(res.tc, "O(n log n)");
    assert_eq!(res.sc, "O(1)");
}

#[test]
fn test_lowbit_loop() {
    let code = r#"
    int countBits(int x) {
        int cnt = 0;
        while (x > 0) {
            cnt++;
            x -= x & -x;
        }
        return cnt;
    }
    "#;

    let res = analyze(code, "cpp");
    assert_eq!(res.tc, "O(log n)");
    assert_eq!(res.sc, "O(1)");
}

#[test]
fn test_bitmask_subsets_loop() {
    let code = r#"
    int tsp(int n, const vector<vector<int>>& dist) {
        vector<vector<int>> dp(1 << n, vector<int>(n, 1e9));
        for (int mask = 1; mask < (1 << n); mask++) {
            for (int u = 0; u < n; u++) {
                if (mask & (1 << u)) dp[mask][u] = 0;
            }
        }
        return dp[(1 << n) - 1][0];
    }
    "#;

    let res = analyze(code, "cpp");
    assert_eq!(res.tc, "O(2^n * n)");
    assert_eq!(res.sc, "O(2^n * n)");
    assert_eq!(res.algorithms, vec!["Bitmask DP"]);
    assert!(kiteo_engine::algorithms::AlgorithmDetector::detect_raw(code)
        .contains(&kiteo_engine::algorithms::AllowedAlgorithm::BitmaskDp));
}

