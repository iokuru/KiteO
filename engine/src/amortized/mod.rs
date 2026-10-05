use crate::complexity::ast::{ComplexityExpr, DimensionVar};
use crate::ir::ast::IrModule;

pub struct AmortizedAnalyzer<'a> {
    _module: &'a IrModule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmortizedPattern {
    TwoPointers,
    SlidingWindow,
    MonotonicStack,
    MonotonicQueue,
    SumOfDegrees,
}

impl<'a> AmortizedAnalyzer<'a> {
    pub fn new(module: &'a IrModule) -> Self {
        Self { _module: module }
    }

    pub fn detect_patterns(&self, source: &str) -> Vec<AmortizedPattern> {
        let mut patterns = Vec::new();

        // 1. Two pointers: left and right pointers moving inward
        if (source.contains("l < r") || source.contains("left < right"))
            && (source.contains("l++") || source.contains("left++"))
            && (source.contains("r--") || source.contains("right--"))
        {
            patterns.push(AmortizedPattern::TwoPointers);
        }

        // 2. Sliding window: outer loop over right bound, inner while advancing left bound
        if (source.contains("for (int r") || source.contains("for (int right") || source.contains("for (int i"))
            && (source.contains("while (") && (source.contains("l++") || source.contains("left++")))
        {
            patterns.push(AmortizedPattern::SlidingWindow);
        }

        // 3. Monotonic stack: each element pushed once and popped at most once
        if (source.contains("stack") || source.contains("ArrayDeque"))
            && source.contains(".pop()")
            && source.contains(".push(")
        {
            patterns.push(AmortizedPattern::MonotonicStack);
        }

        // 4. Monotonic queue / deque: sliding window maximum with pop_front/pop_back
        if (source.contains("deque") || source.contains("Deque"))
            && (source.contains("pop_front") || source.contains("pollFirst"))
        {
            patterns.push(AmortizedPattern::MonotonicQueue);
        }

        // 5. Sum of degrees rule: loop over adjacency list inside graph traversal
        if (source.contains("g[u]") || source.contains("g.get(u)") || source.contains("adj[u]") || source.contains("adj.get(u)"))
            && (source.contains("dfs(") || source.contains("bfs(") || source.contains("dijkstra(") || source.contains("topoSort("))
        {
            patterns.push(AmortizedPattern::SumOfDegrees);
        }

        patterns
    }

    pub fn compute_amortized_bound(&self, pattern: AmortizedPattern) -> ComplexityExpr {
        match pattern {
            AmortizedPattern::TwoPointers => ComplexityExpr::var(DimensionVar::N),
            AmortizedPattern::SlidingWindow => ComplexityExpr::var(DimensionVar::N),
            AmortizedPattern::MonotonicStack => ComplexityExpr::var(DimensionVar::N),
            AmortizedPattern::MonotonicQueue => ComplexityExpr::var(DimensionVar::N),
            AmortizedPattern::SumOfDegrees => {
                ComplexityExpr::add(ComplexityExpr::var(DimensionVar::V), ComplexityExpr::var(DimensionVar::E))
            }
        }
    }
}
