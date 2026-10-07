use kiteo_engine::callgraph::CallGraph;
use kiteo_engine::cfg::ControlFlowGraph;
use kiteo_engine::ir::CppNormalizer;
use kiteo_engine::recursion::MemoizedDpAnalyzer;

#[test]
fn test_cfg_detects_loop_cycle() {
    let code = r#"
    void loopFunc(int n) {
        for (int i = 0; i < n; i++) {
            sum += i;
        }
    }
    "#;

    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_cpp::language()).unwrap();
    let tree = parser.parse(code, None).unwrap();
    let ast = kiteo_engine::parser::tree_sitter_to_ast(tree.root_node());
    let ir = CppNormalizer::new(code).normalize(&ast);

    let cfg = ControlFlowGraph::build(&ir);
    assert!(
        cfg.has_cycles(),
        "Loop in function body must produce cycle in CFG"
    );
}

#[test]
fn test_callgraph_detects_recursion() {
    let code = r#"
    int fact(int n) {
        if (n <= 1) return 1;
        return n * fact(n - 1);
    }
    "#;

    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&tree_sitter_cpp::language()).unwrap();
    let tree = parser.parse(code, None).unwrap();
    let ast = kiteo_engine::parser::tree_sitter_to_ast(tree.root_node());
    let ir = CppNormalizer::new(code).normalize(&ast);

    let cg = CallGraph::build(&ir);
    assert!(
        cg.is_recursive("fact"),
        "Call graph must detect recursive self-invocation in factorial"
    );
}

#[test]
fn test_memoized_dp_complexity() {
    let dp_code = r#"
    int solve(int i, int j) {
        if (i == 0 || j == 0) return 0;
        if (dp[i][j] != -1) return dp[i][j];
        return dp[i][j] = solve(i - 1, j) + solve(i, j - 1);
    }
    "#;

    let result = MemoizedDpAnalyzer::analyze(dp_code);
    assert!(result.is_some());
    let expr = result.unwrap();
    assert_eq!(expr.to_string(), "O(n m)");
}
