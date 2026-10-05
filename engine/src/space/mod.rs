use crate::complexity::ast::{ComplexityExpr, DimensionVar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayUsage {
    InputIndexed,
    ValueIndexed,
    ConstantSmall,
    Unclear,
}

pub struct SpaceAnalyzer;

impl SpaceAnalyzer {
    pub fn analyze_source(source: &str) -> ComplexityExpr {
        // 1. Check fixed-size array usage rules
        if source.contains("freq[") || source.contains("count[") || source.contains("freq.") {
            return ComplexityExpr::var(DimensionVar::A);
        }

        if source.contains("tree[4 * n")
            || source.contains("tree[4*n")
            || source.contains("new int[4 * n")
        {
            return ComplexityExpr::var(DimensionVar::N);
        }

        if source.contains("st[")
            && (source.contains("][20]") || source.contains("][log") || source.contains("[20][n]"))
        {
            return ComplexityExpr::mul(
                ComplexityExpr::var(DimensionVar::N),
                ComplexityExpr::log(ComplexityExpr::var(DimensionVar::N)),
            );
        }

        if source.contains("1 << n") || source.contains("1<<n") {
            return ComplexityExpr::mul(
                ComplexityExpr::pow(ComplexityExpr::Const(2), 0),
                ComplexityExpr::var(DimensionVar::N),
            );
        }

        // 2. Graph structures: V + E space
        if (source.contains("vector<vector<") || source.contains("List<List<"))
            && (source.contains("dist[") || source.contains("g[") || source.contains("g.get("))
        {
            return ComplexityExpr::add(
                ComplexityExpr::var(DimensionVar::V),
                ComplexityExpr::var(DimensionVar::E),
            );
        }

        // 3. Dynamic auxiliary arrays / recursion
        if source.contains("vector<")
            || source.contains("new int[")
            || source.contains("new long[")
            || source.contains("new boolean[")
        {
            return ComplexityExpr::var(DimensionVar::N);
        }

        if source.contains("factorial(") || (source.contains("dfs(") && source.contains("int u")) {
            return ComplexityExpr::var(DimensionVar::N);
        }

        // 4. Default in-place auxiliary space
        ComplexityExpr::one()
    }
}
