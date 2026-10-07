use kiteo_engine::ir::{CppNormalizer, JavaNormalizer};

#[test]
fn test_loop_ir_snapshot_cpp_vs_java() {
    let cpp_code = r#"
    void solve(int n) {
        for (int i = 0; i < n; i++) {
            sum += i;
        }
    }
    "#;

    let java_code = r#"
    class Solution {
        public void solve(int n) {
            for (int i = 0; i < n; i++) {
                sum += i;
            }
        }
    }
    "#;

    let mut cpp_parser = tree_sitter::Parser::new();
    cpp_parser
        .set_language(&tree_sitter_cpp::language())
        .unwrap();
    let cpp_tree = cpp_parser.parse(cpp_code, None).unwrap();
    let cpp_ast = kiteo_engine::parser::tree_sitter_to_ast(cpp_tree.root_node());
    let cpp_ir = CppNormalizer::new(cpp_code).normalize(&cpp_ast);

    let mut java_parser = tree_sitter::Parser::new();
    java_parser
        .set_language(&tree_sitter_java::language())
        .unwrap();
    let java_tree = java_parser.parse(java_code, None).unwrap();
    let java_ast = kiteo_engine::parser::tree_sitter_to_ast(java_tree.root_node());
    let java_ir = JavaNormalizer::new(java_code).normalize(&java_ast);

    assert_eq!(cpp_ir.functions.len(), 1);
    assert_eq!(java_ir.functions.len(), 1);

    let has_for_cpp = cpp_ir
        .stmts
        .iter()
        .any(|s| matches!(s, kiteo_engine::ir::ast::IrStmt::For { .. }));
    let has_for_java = java_ir
        .stmts
        .iter()
        .any(|s| matches!(s, kiteo_engine::ir::ast::IrStmt::For { .. }));
    assert!(has_for_cpp, "C++ IR must have For loop");
    assert!(has_for_java, "Java IR must have For loop");

    insta::assert_yaml_snapshot!("cpp_loop_ir_stmts_count", cpp_ir.stmts.len());
    insta::assert_yaml_snapshot!("java_loop_ir_stmts_count", java_ir.stmts.len());
}

#[test]
fn test_condition_and_call_ir_snapshot() {
    let cpp_code = r#"
    int dfs(int u, int p) {
        if (u == p) return 0;
        return dfs(p, u);
    }
    "#;

    let java_code = r#"
    class Solution {
        public int dfs(int u, int p) {
            if (u == p) return 0;
            return dfs(p, u);
        }
    }
    "#;

    let mut cpp_parser = tree_sitter::Parser::new();
    cpp_parser
        .set_language(&tree_sitter_cpp::language())
        .unwrap();
    let cpp_tree = cpp_parser.parse(cpp_code, None).unwrap();
    let cpp_ast = kiteo_engine::parser::tree_sitter_to_ast(cpp_tree.root_node());
    let cpp_ir = CppNormalizer::new(cpp_code).normalize(&cpp_ast);

    let mut java_parser = tree_sitter::Parser::new();
    java_parser
        .set_language(&tree_sitter_java::language())
        .unwrap();
    let java_tree = java_parser.parse(java_code, None).unwrap();
    let java_ast = kiteo_engine::parser::tree_sitter_to_ast(java_tree.root_node());
    let java_ir = JavaNormalizer::new(java_code).normalize(&java_ast);

    let has_if_cpp = cpp_ir
        .stmts
        .iter()
        .any(|s| matches!(s, kiteo_engine::ir::ast::IrStmt::If { .. }));
    let has_if_java = java_ir
        .stmts
        .iter()
        .any(|s| matches!(s, kiteo_engine::ir::ast::IrStmt::If { .. }));
    assert!(has_if_cpp);
    assert!(has_if_java);

    let has_call_cpp = cpp_ir
        .exprs
        .iter()
        .any(|e| matches!(e, kiteo_engine::ir::ast::IrExpr::Call(name, _) if name.contains("dfs")));
    let has_call_java = java_ir
        .exprs
        .iter()
        .any(|e| matches!(e, kiteo_engine::ir::ast::IrExpr::Call(name, _) if name.contains("dfs")));
    assert!(has_call_cpp);
    assert!(has_call_java);
}
