use super::arena::{BlockId, ExprId, StmtId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
    BitNot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrExpr {
    LiteralInt(i64),
    LiteralBool(bool),
    Var(String),
    Binary(BinaryOp, ExprId, ExprId),
    Unary(UnaryOp, ExprId),
    Call(String, Vec<ExprId>),
    Index(ExprId, ExprId),
    MemberAccess(ExprId, String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrStmt {
    Assign(String, ExprId),
    Expr(ExprId),
    If {
        cond: ExprId,
        then_block: BlockId,
        else_block: Option<BlockId>,
    },
    While {
        cond: ExprId,
        body: BlockId,
    },
    For {
        init: Option<StmtId>,
        cond: Option<ExprId>,
        step: Option<StmtId>,
        body: BlockId,
    },
    Return(Option<ExprId>),
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IrBlock {
    pub stmts: Vec<StmtId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<String>,
    pub body: BlockId,
}

#[derive(Debug, Clone, Default)]
pub struct IrModule {
    pub exprs: Vec<IrExpr>,
    pub stmts: Vec<IrStmt>,
    pub blocks: Vec<IrBlock>,
    pub functions: Vec<IrFunction>,
}

impl IrModule {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alloc_expr(&mut self, expr: IrExpr) -> ExprId {
        let id = ExprId(self.exprs.len());
        self.exprs.push(expr);
        id
    }

    pub fn alloc_stmt(&mut self, stmt: IrStmt) -> StmtId {
        let id = StmtId(self.stmts.len());
        self.stmts.push(stmt);
        id
    }

    pub fn alloc_block(&mut self, block: IrBlock) -> BlockId {
        let id = BlockId(self.blocks.len());
        self.blocks.push(block);
        id
    }
}
