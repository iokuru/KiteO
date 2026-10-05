use kiteo_engine::amortized::{AmortizedAnalyzer, AmortizedPattern};
use kiteo_engine::ir::ast::IrModule;

#[test]
fn test_detects_two_pointers_pattern() {
    let code = r#"
    int l = 0, r = n - 1;
    while (l < r) {
        if (a[l] + a[r] == target) return true;
        else if (a[l] + a[r] < target) l++;
        else r--;
    }
    "#;

    let module = IrModule::new();
    let analyzer = AmortizedAnalyzer::new(&module);
    let patterns = analyzer.detect_patterns(code);
    assert!(patterns.contains(&AmortizedPattern::TwoPointers));
    let bound = analyzer.compute_amortized_bound(AmortizedPattern::TwoPointers);
    assert_eq!(bound.to_string(), "O(n)");
}

#[test]
fn test_detects_sum_of_degrees_graph_rule() {
    let code = r#"
    void dfs(int u, vector<bool>& vis, const vector<vector<int>>& g) {
        vis[u] = true;
        for (int v : g[u]) {
            if (!vis[v]) dfs(v, vis, g);
        }
    }
    "#;

    let module = IrModule::new();
    let analyzer = AmortizedAnalyzer::new(&module);
    let patterns = analyzer.detect_patterns(code);
    assert!(patterns.contains(&AmortizedPattern::SumOfDegrees));
    let bound = analyzer.compute_amortized_bound(AmortizedPattern::SumOfDegrees);
    assert_eq!(bound.to_string(), "O(V + E)");
}
