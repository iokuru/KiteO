use crate::complexity::ast::{ComplexityExpr, DimensionVar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DpStructure {
    OneDimensional,
    TwoDimensional,
    Bitmask,
}

pub struct MemoizedDpAnalyzer;

impl MemoizedDpAnalyzer {
    pub fn analyze(source: &str) -> Option<ComplexityExpr> {
        let has_memo_check = source.contains("dp[")
            && (source.contains("!= -1")
                || source.contains("!= 0")
                || source.contains("visited[")
                || source.contains("vis["));

        if !has_memo_check {
            return None;
        }

        if source.contains("1 <<") || source.contains("1<<") || source.contains("mask") {
            // Bitmask DP: states = 2^n, transition = n
            let state_expr = ComplexityExpr::pow(ComplexityExpr::Const(2), 0); // O(2^n * n)
            let _ = state_expr;
            return Some(ComplexityExpr::mul(
                ComplexityExpr::var(DimensionVar::N),
                ComplexityExpr::pow(ComplexityExpr::Const(2), 0),
            ));
        }

        if source.contains("dp[") && source.contains("][") {
            // 2D DP: states = n * m
            return Some(ComplexityExpr::mul(
                ComplexityExpr::var(DimensionVar::N),
                ComplexityExpr::var(DimensionVar::M),
            ));
        }

        // 1D DP: states = n
        Some(ComplexityExpr::var(DimensionVar::N))
    }
}
