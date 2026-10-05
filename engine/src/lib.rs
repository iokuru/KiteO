use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn test_parse(code: &str, lang: &str) -> String {
    let mut parser = tree_sitter::Parser::new();
    let language = match lang {
        "cpp" => tree_sitter_cpp::language(),
        "java" => tree_sitter_java::language(),
        _ => return "unsupported language".to_string(),
    };

    if parser.set_language(&language).is_err() {
        return "failed to set language".to_string();
    }

    match parser.parse(code, None) {
        Some(tree) => tree.root_node().to_sexp(),
        None => "failed to parse".to_string(),
    }
}

#[wasm_bindgen]
pub fn parse_serialized_ast(ast_json: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(ast_json) {
        Ok(v) => format!(
            "valid ast with type {}",
            v.get("type").and_then(|t| t.as_str()).unwrap_or("unknown")
        ),
        Err(e) => format!("invalid ast: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_cpp() {
        let sexp = test_parse("int main() { return 0; }", "cpp");
        assert!(sexp.contains("translation_unit"));
    }

    #[test]
    fn parses_simple_java() {
        let sexp = test_parse(
            "class Solution { public int solve() { return 0; } }",
            "java",
        );
        assert!(sexp.contains("program"));
    }

    #[test]
    fn parses_serialized_ast_json() {
        let res = parse_serialized_ast(r#"{"type":"translation_unit","children":[]}"#);
        assert_eq!(res, "valid ast with type translation_unit");
    }
}
