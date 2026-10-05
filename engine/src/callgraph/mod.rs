use crate::complexity::ast::{ComplexityExpr, DimensionVar};
use crate::ir::ast::{IrExpr, IrModule, IrStmt};
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

pub struct CallGraph {
    pub graph: DiGraph<String, ()>,
    pub fn_nodes: HashMap<String, NodeIndex>,
}

impl Default for CallGraph {
    fn default() -> Self {
        Self {
            graph: DiGraph::new(),
            fn_nodes: HashMap::new(),
        }
    }
}

impl CallGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(module: &IrModule) -> Self {
        let mut cg = Self::new();

        for func in &module.functions {
            let idx = cg.graph.add_node(func.name.clone());
            cg.fn_nodes.insert(func.name.clone(), idx);
        }

        // Trace call expressions inside each function
        for func in &module.functions {
            let u = cg.fn_nodes[&func.name];
            let calls = find_function_calls(func.body, module);
            for callee in calls {
                if let Some(&v) = cg.fn_nodes.get(&callee) {
                    cg.graph.add_edge(u, v, ());
                }
            }
        }

        cg
    }

    pub fn is_recursive(&self, fn_name: &str) -> bool {
        if let Some(&u) = self.fn_nodes.get(fn_name) {
            // Check self edge or cycle
            let mut visited = HashMap::new();
            self.dfs_cycle(u, u, &mut visited)
        } else {
            false
        }
    }

    fn dfs_cycle(
        &self,
        start: NodeIndex,
        current: NodeIndex,
        visited: &mut HashMap<NodeIndex, bool>,
    ) -> bool {
        for neighbor in self.graph.neighbors(current) {
            if neighbor == start {
                return true;
            }
            if !visited.get(&neighbor).copied().unwrap_or(false) {
                visited.insert(neighbor, true);
                if self.dfs_cycle(start, neighbor, visited) {
                    return true;
                }
            }
        }
        false
    }
}

fn find_function_calls(block_id: crate::ir::arena::BlockId, module: &IrModule) -> Vec<String> {
    let mut calls = Vec::new();
    let block = match module.blocks.get(block_id.0) {
        Some(b) => b,
        None => return calls,
    };

    for &stmt_id in &block.stmts {
        if let Some(stmt) = module.stmts.get(stmt_id.0) {
            match stmt {
                IrStmt::Expr(expr_id) | IrStmt::Return(Some(expr_id)) => {
                    extract_calls_from_expr(*expr_id, module, &mut calls);
                }
                IrStmt::Assign(_, expr_id) => {
                    extract_calls_from_expr(*expr_id, module, &mut calls);
                }
                IrStmt::If {
                    cond,
                    then_block,
                    else_block,
                } => {
                    extract_calls_from_expr(*cond, module, &mut calls);
                    calls.extend(find_function_calls(*then_block, module));
                    if let Some(eb) = else_block {
                        calls.extend(find_function_calls(*eb, module));
                    }
                }
                IrStmt::While { cond, body } => {
                    extract_calls_from_expr(*cond, module, &mut calls);
                    calls.extend(find_function_calls(*body, module));
                }
                IrStmt::For {
                    init,
                    cond,
                    step,
                    body,
                } => {
                    if let Some(c) = cond {
                        extract_calls_from_expr(*c, module, &mut calls);
                    }
                    if let Some(s) = step {
                        if let Some(IrStmt::Assign(_, e)) = module.stmts.get(s.0) {
                            extract_calls_from_expr(*e, module, &mut calls);
                        }
                    }
                    let _ = init;
                    calls.extend(find_function_calls(*body, module));
                }
                _ => {}
            }
        }
    }
    calls
}

fn extract_calls_from_expr(
    expr_id: crate::ir::arena::ExprId,
    module: &IrModule,
    calls: &mut Vec<String>,
) {
    if let Some(expr) = module.exprs.get(expr_id.0) {
        match expr {
            IrExpr::Call(name, args) => {
                calls.push(name.clone());
                for &arg in args {
                    extract_calls_from_expr(arg, module, calls);
                }
            }
            IrExpr::Binary(_, l, r) => {
                extract_calls_from_expr(*l, module, calls);
                extract_calls_from_expr(*r, module, calls);
            }
            IrExpr::Unary(_, operand) => {
                extract_calls_from_expr(*operand, module, calls);
            }
            IrExpr::Index(base, idx) => {
                extract_calls_from_expr(*base, module, calls);
                extract_calls_from_expr(*idx, module, calls);
            }
            _ => {}
        }
    }
}

pub struct RecursionSolver;

impl RecursionSolver {
    pub fn solve_linear_recurrence(depth_var: DimensionVar) -> ComplexityExpr {
        ComplexityExpr::var(depth_var)
    }

    pub fn solve_divide_and_conquer(var: DimensionVar) -> ComplexityExpr {
        ComplexityExpr::mul(
            ComplexityExpr::var(var),
            ComplexityExpr::log(ComplexityExpr::var(var)),
        )
    }
}
