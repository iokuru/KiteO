use kiteo_engine::analyze;

#[test]
fn test_unclosed_brace_cpp() {
    let code = r#"
    void solve(int n) {
        for (int i = 0; i < n; i++) {
            sum += i;
    "#;
    let res = analyze(code, "cpp");
    // Should parse with error recovery or fallback gracefully without panic
    assert!(!res.tc.is_empty(), "TC should not be empty");
    assert!(!res.sc.is_empty(), "SC should not be empty");
}

#[test]
fn test_missing_semicolon_cpp() {
    let code = r#"
    void solve(int n) {
        int x = 10
        for (int i = 0; i < n; i++) {
            sum += i;
        }
    }
    "#;
    let res = analyze(code, "cpp");
    assert!(!res.tc.is_empty());
}

#[test]
fn test_half_written_loop_cpp() {
    let code = r#"
    void solve(int n) {
        for (int i = 0; i < n;
    "#;
    let res = analyze(code, "cpp");
    assert!(!res.tc.is_empty());
}

#[test]
fn test_unclosed_brace_java() {
    let code = r#"
    class Solution {
        public void solve(int n) {
            for (int i = 0; i < n; i++) {
                sum += i;
    "#;
    let res = analyze(code, "java");
    assert!(!res.tc.is_empty());
}

#[test]
fn test_completely_malformed_syntax() {
    let code = r#"
    ??? !!! @@@ this is not code {{{
    "#;
    let res = analyze(code, "cpp");
    // Unknown or O(1) fallback, never panics
    assert!(!res.tc.is_empty());
}
