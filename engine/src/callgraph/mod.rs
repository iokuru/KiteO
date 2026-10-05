use petgraph::graph::DiGraph;

pub struct CallGraph {
    pub graph: DiGraph<String, ()>,
}

impl Default for CallGraph {
    fn default() -> Self {
        Self {
            graph: DiGraph::new(),
        }
    }
}
