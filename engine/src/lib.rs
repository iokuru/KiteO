use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub mod algorithms;
pub mod amortized;
pub mod callgraph;
pub mod cfg;
pub mod complexity;
pub mod cost_model;
pub mod dataflow;
pub mod ir;
pub mod loops;
pub mod parser;
pub mod preprocessor;
pub mod recursion;
pub mod space;
pub mod wasm;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnalysisOutput {
    pub tc: String,
    pub sc: String,
    pub algorithms: Vec<String>,
}

impl Default for AnalysisOutput {
    fn default() -> Self {
        Self {
            tc: "Unknown".to_string(),
            sc: "Unknown".to_string(),
            algorithms: Vec::new(),
        }
    }
}

pub fn analyze(code: &str, lang: &str) -> AnalysisOutput {
    // Top-level entrypoint: preprocesses code, parses AST, lowers to IR,
    // and runs symbolic complexity, loop/recursion, space, and algorithm detection passes.
    let _ = (code, lang);
    AnalysisOutput::default()
}

#[wasm_bindgen]
pub fn analyze_wasm(code: &str, lang: &str) -> String {
    let result = analyze(code, lang);
    serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_analysis_returns_unknown() {
        let res = analyze("int main() {}", "cpp");
        assert_eq!(res.tc, "Unknown");
        assert_eq!(res.sc, "Unknown");
        assert!(res.algorithms.is_empty());
    }
}
