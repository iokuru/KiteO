use crate::ir::arena::BlockId;
use crate::ir::ast::{IrBlock, IrModule, IrStmt};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

pub struct ControlFlowGraph {
    pub graph: DiGraph<usize, ()>,
    pub block_nodes: HashMap<BlockId, NodeIndex>,
}

impl Default for ControlFlowGraph {
    fn default() -> Self {
        Self {
            graph: DiGraph::new(),
            block_nodes: HashMap::new(),
        }
    }
}

impl ControlFlowGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(module: &IrModule) -> Self {
        let mut cfg = Self::new();

        // Create a CFG node for each block
        for (i, _) in module.blocks.iter().enumerate() {
            let idx = cfg.graph.add_node(i);
            cfg.block_nodes.insert(BlockId(i), idx);
        }

        // Add edges between sequential blocks and control branches
        for (i, block) in module.blocks.iter().enumerate() {
            let u = cfg.block_nodes[&BlockId(i)];
            cfg.connect_block(u, block, module);
        }

        cfg
    }

    fn connect_block(&mut self, u: NodeIndex, block: &IrBlock, module: &IrModule) {
        for &stmt_id in &block.stmts {
            if let Some(stmt) = module.stmts.get(stmt_id.0) {
                match stmt {
                    IrStmt::If {
                        then_block,
                        else_block,
                        ..
                    } => {
                        if let Some(&v_then) = self.block_nodes.get(then_block) {
                            self.graph.add_edge(u, v_then, ());
                        }
                        if let Some(eb) = else_block {
                            if let Some(&v_else) = self.block_nodes.get(eb) {
                                self.graph.add_edge(u, v_else, ());
                            }
                        }
                    }
                    IrStmt::While { body, .. } | IrStmt::For { body, .. } => {
                        if let Some(&v_body) = self.block_nodes.get(body) {
                            self.graph.add_edge(u, v_body, ());
                            // Backedge for loop
                            self.graph.add_edge(v_body, u, ());
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn has_cycles(&self) -> bool {
        petgraph::algo::is_cyclic_directed(&self.graph)
    }
}
