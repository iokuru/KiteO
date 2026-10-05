use crate::complexity::ast::{ComplexityExpr, DimensionVar};
use crate::complexity::simplify::simplify;
use crate::ir::arena::{BlockId, ExprId, StmtId};
use crate::ir::ast::{BinaryOp, IrExpr, IrModule, IrStmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopBound {
    Linear(DimensionVar),
    Logarithmic(DimensionVar),
    LogLog(DimensionVar),
    Constant(u64),
    Unknown,
}

#[derive(Debug, Clone)]
pub struct LoopInfo {
    pub bound: LoopBound,
    pub body: BlockId,
    pub is_dependent: bool,
    pub is_harmonic: bool,
}

pub struct LoopAnalyzer<'a> {
    module: &'a IrModule,
}

impl<'a> LoopAnalyzer<'a> {
    pub fn new(module: &'a IrModule) -> Self {
        Self { module }
    }

    pub fn analyze_module(&self) -> ComplexityExpr {
        if self.module.functions.is_empty() {
            return ComplexityExpr::one();
        }

        let mut total_complexity = ComplexityExpr::one();
        for func in &self.module.functions {
            let func_comp = self.analyze_block(func.body);
            total_complexity = simplify(&ComplexityExpr::add(total_complexity, func_comp));
        }
        total_complexity
    }

    pub fn analyze_block(&self, block_id: BlockId) -> ComplexityExpr {
        let block = match self.module.blocks.get(block_id.0) {
            Some(b) => b,
            None => return ComplexityExpr::one(),
        };

        let mut block_comp = ComplexityExpr::one();
        for &stmt_id in &block.stmts {
            let stmt_comp = self.analyze_statement(stmt_id);
            block_comp = simplify(&ComplexityExpr::add(block_comp, stmt_comp));
        }
        block_comp
    }

    pub fn analyze_statement(&self, stmt_id: StmtId) -> ComplexityExpr {
        let stmt = match self.module.stmts.get(stmt_id.0) {
            Some(s) => s,
            None => return ComplexityExpr::one(),
        };

        match stmt {
            IrStmt::For {
                init,
                cond,
                step,
                body,
            } => self.analyze_for_loop(*init, *cond, *step, *body),
            IrStmt::While { cond, body } => self.analyze_while_loop(*cond, *body),
            IrStmt::If {
                then_block,
                else_block,
                ..
            } => {
                let then_comp = self.analyze_block(*then_block);
                let else_comp = else_block
                    .map(|b| self.analyze_block(b))
                    .unwrap_or_else(ComplexityExpr::one);
                simplify(&ComplexityExpr::max(then_comp, else_comp))
            }
            _ => ComplexityExpr::one(),
        }
    }

    fn analyze_for_loop(
        &self,
        init: Option<StmtId>,
        cond: Option<ExprId>,
        step: Option<StmtId>,
        body: BlockId,
    ) -> ComplexityExpr {
        let loop_info = self.classify_for_loop(init, cond, step, body);
        let body_comp = self.analyze_block(body);

        let iter_comp = match loop_info.bound {
            LoopBound::Linear(v) => ComplexityExpr::var(v),
            LoopBound::Logarithmic(v) => ComplexityExpr::log(ComplexityExpr::var(v)),
            LoopBound::LogLog(v) => {
                ComplexityExpr::log(ComplexityExpr::log(ComplexityExpr::var(v)))
            }
            LoopBound::Constant(_) => ComplexityExpr::one(),
            LoopBound::Unknown => ComplexityExpr::var(DimensionVar::N),
        };

        if loop_info.is_harmonic {
            // Harmonic sum \sum_{i=1}^n n/i = n log n
            ComplexityExpr::mul(
                ComplexityExpr::var(DimensionVar::N),
                ComplexityExpr::log(ComplexityExpr::var(DimensionVar::N)),
            )
        } else {
            simplify(&ComplexityExpr::mul(iter_comp, body_comp))
        }
    }

    fn classify_for_loop(
        &self,
        _init: Option<StmtId>,
        cond: Option<ExprId>,
        step: Option<StmtId>,
        body: BlockId,
    ) -> LoopInfo {
        let mut bound_var = DimensionVar::N;
        let mut is_geometric = false;
        let mut is_halving = false;
        let mut is_harmonic = false;
        let mut is_testcase_driver = false;

        // Inspect condition
        if let Some(cond_id) = cond {
            if let Some(IrExpr::Binary(op, left, right)) = self.module.exprs.get(cond_id.0) {
                match op {
                    BinaryOp::Lt | BinaryOp::Le => {
                        if let Some(IrExpr::Var(name)) = self.module.exprs.get(right.0) {
                            if name == "t" || name == "tc" || name == "tests" || name == "test_cases" {
                                is_testcase_driver = true;
                            }
                        }
                        bound_var = self.extract_var_from_expr(*right);
                    }
                    BinaryOp::Gt | BinaryOp::Ge => {
                        if let Some(IrExpr::Var(name)) = self.module.exprs.get(left.0) {
                            if name == "t" || name == "tc" || name == "tests" || name == "test_cases" {
                                is_testcase_driver = true;
                            }
                        }
                        bound_var = self.extract_var_from_expr(*left);
                    }
                    _ => {}
                }
            }
        }

        // Inspect step
        if let Some(step_id) = step {
            if let Some(IrStmt::Assign(_v, rhs)) = self.module.stmts.get(step_id.0) {
                if let Some(IrExpr::Binary(op, _, _)) = self.module.exprs.get(rhs.0) {
                    match op {
                        BinaryOp::Mul => is_geometric = true,
                        BinaryOp::Div => is_halving = true,
                        _ => {}
                    }
                }
            }
        }

        // Check if body contains a nested loop that steps by outer variable (harmonic)
        if let Some(body_block) = self.module.blocks.get(body.0) {
            for &s_id in &body_block.stmts {
                if let Some(IrStmt::For {
                    step: Some(inner_step),
                    ..
                }) = self.module.stmts.get(s_id.0)
                {
                    if let Some(IrStmt::Assign(_iv, irhs)) = self.module.stmts.get(inner_step.0) {
                        if let Some(IrExpr::Binary(BinaryOp::Add, _, rhs_val)) =
                            self.module.exprs.get(irhs.0)
                        {
                            if let Some(IrExpr::Var(name)) = self.module.exprs.get(rhs_val.0) {
                                if name == "i" {
                                    is_harmonic = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        let bound = if is_testcase_driver {
            LoopBound::Constant(1)
        } else if is_geometric || is_halving {
            LoopBound::Logarithmic(bound_var)
        } else {
            LoopBound::Linear(bound_var)
        };

        LoopInfo {
            bound,
            body,
            is_dependent: false,
            is_harmonic,
        }
    }

    fn analyze_while_loop(&self, cond: ExprId, body: BlockId) -> ComplexityExpr {
        let body_comp = self.analyze_block(body);

        // Check for multi-testcase while loop driver: while(t--) or while(t > 0)
        if let Some(IrExpr::Var(name)) = self.module.exprs.get(cond.0) {
            if name == "t" || name == "tc" || name == "tests" || name == "test_cases" {
                return body_comp;
            }
        }

        // Check for lowbit pattern: while (x > 0) { ... x -= x & -x; }
        if self.is_lowbit_loop(body) {
            return simplify(&ComplexityExpr::mul(
                ComplexityExpr::log(ComplexityExpr::var(DimensionVar::N)),
                body_comp,
            ));
        }

        // Check for two pointers / sliding window pattern in while loop
        if self.is_two_pointer_loop(cond, body) {
            return ComplexityExpr::var(DimensionVar::N);
        }

        // Default while loop bound to logarithmic or linear
        simplify(&ComplexityExpr::mul(
            ComplexityExpr::var(DimensionVar::N),
            body_comp,
        ))
    }

    fn is_lowbit_loop(&self, body: BlockId) -> bool {
        let block = match self.module.blocks.get(body.0) {
            Some(b) => b,
            None => return false,
        };

        for &stmt_id in &block.stmts {
            if let Some(IrStmt::Assign(_, rhs)) = self.module.stmts.get(stmt_id.0) {
                if let Some(IrExpr::Binary(BinaryOp::Sub, _, right_sub)) =
                    self.module.exprs.get(rhs.0)
                {
                    if let Some(IrExpr::Binary(BinaryOp::BitAnd, _, _)) =
                        self.module.exprs.get(right_sub.0)
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn is_two_pointer_loop(&self, _cond: ExprId, _body: BlockId) -> bool {
        false
    }

    fn extract_var_from_expr(&self, expr_id: ExprId) -> DimensionVar {
        if let Some(expr) = self.module.exprs.get(expr_id.0) {
            match expr {
                IrExpr::Var(name) => match name.as_str() {
                    "m" => DimensionVar::M,
                    "q" => DimensionVar::Q,
                    "k" => DimensionVar::K,
                    "V" => DimensionVar::V,
                    "E" => DimensionVar::E,
                    "A" => DimensionVar::A,
                    _ => DimensionVar::N,
                },
                IrExpr::MemberAccess(_, field) if field == "size" || field == "length" => {
                    DimensionVar::N
                }
                IrExpr::Call(name, _) if name.contains("size") || name.contains("length") => {
                    DimensionVar::N
                }
                IrExpr::Binary(_, left, right) => {
                    let lv = self.extract_var_from_expr(*left);
                    let rv = self.extract_var_from_expr(*right);
                    if lv != DimensionVar::N {
                        lv
                    } else {
                        rv
                    }
                }
                _ => DimensionVar::N,
            }
        } else {
            DimensionVar::N
        }
    }
}
