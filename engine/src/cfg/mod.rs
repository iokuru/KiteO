use petgraph::graph::DiGraph;

pub struct ControlFlowGraph {
    pub graph: DiGraph<usize, ()>,
}

impl Default for ControlFlowGraph {
    fn default() -> Self {
        Self {
            graph: DiGraph::new(),
        }
    }
}
