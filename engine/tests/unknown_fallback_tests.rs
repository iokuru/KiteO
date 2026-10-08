use kiteo_engine::analyze;

#[test]
fn test_unsupported_language_returns_unknown() {
    let code = "print('hello')";
    let output = analyze(code, "python");
    assert_eq!(output.tc, "Unknown");
    assert_eq!(output.sc, "Unknown");
}

#[test]
fn test_completely_malformed_code_returns_unknown() {
    // When code is severely malformed and cannot produce a valid AST
    let code = "@#$%^&*()_+{}|:<>?";
    let output = analyze(code, "cpp");
    assert_eq!(output.tc, "Unknown");
    assert_eq!(output.sc, "Unknown");
}

#[test]
fn test_unclosed_infinite_loop_without_bound() {
    // A loop with no condition and no step
    let code = r#"
    int main() {
        int x = 0;
        for (;;) {
            x++;
            if (x > 10) break;
        }
        return 0;
    }
    "#;
    let output = analyze(code, "cpp");
    assert_eq!(output.tc, "Unknown");
}

#[test]
fn test_loop_with_unknown_condition_returns_unknown() {
    let code = r#"
    bool custom_check(int x);
    int main() {
        int cnt = 0;
        for (int i = 0; custom_check(i); i++) {
            cnt++;
        }
        return 0;
    }
    "#;
    let output = analyze(code, "cpp");
    assert_eq!(output.tc, "Unknown");
}
