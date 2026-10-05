#[cfg(not(target_arch = "wasm32"))]
use super::arena::{BlockId, ExprId, StmtId};
#[cfg(not(target_arch = "wasm32"))]
use super::ast::*;
#[cfg(not(target_arch = "wasm32"))]
use tree_sitter::Node;

#[cfg(not(target_arch = "wasm32"))]
pub struct JavaNormalizer<'a> {
    source: &'a [u8],
    module: IrModule,
}

#[cfg(not(target_arch = "wasm32"))]
impl<'a> JavaNormalizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            module: IrModule::new(),
        }
    }

    pub fn normalize(mut self, root: Node<'a>) -> IrModule {
        self.visit_node(root);
        self.module
    }

    fn text(&self, node: Node<'a>) -> String {
        node.utf8_text(self.source).unwrap_or("").to_string()
    }

    fn visit_node(&mut self, node: Node<'a>) {
        match node.kind() {
            "method_declaration" => {
                self.visit_method(node);
            }
            "class_declaration" | "program" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    self.visit_node(child);
                }
            }
            _ => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    self.visit_node(child);
                }
            }
        }
    }

    fn visit_method(&mut self, node: Node<'a>) {
        let name = node
            .child_by_field_name("name")
            .map(|d| self.text(d))
            .unwrap_or_else(|| "anon".to_string());

        let body_node = node.child_by_field_name("body");
        let body_block = if let Some(body) = body_node {
            self.lower_block(body)
        } else {
            self.module.alloc_block(IrBlock::default())
        };

        self.module.functions.push(IrFunction {
            name,
            params: Vec::new(),
            body: body_block,
        });
    }

    pub fn lower_block(&mut self, node: Node<'a>) -> BlockId {
        let mut stmts = Vec::new();
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "{" || child.kind() == "}" || child.kind() == ";" {
                continue;
            }
            if let Some(stmt_id) = self.lower_statement(child) {
                stmts.push(stmt_id);
            }
        }
        self.module.alloc_block(IrBlock { stmts })
    }

    pub fn lower_statement(&mut self, node: Node<'a>) -> Option<StmtId> {
        match node.kind() {
            "block" => {
                let block_id = self.lower_block(node);
                let dummy_expr = self.module.alloc_expr(IrExpr::LiteralInt(0));
                let dummy_stmt = self.module.alloc_stmt(IrStmt::Expr(dummy_expr));
                if let Some(b) = self.module.blocks.get(block_id.0) {
                    if let Some(first) = b.stmts.first() {
                        return Some(*first);
                    }
                }
                Some(dummy_stmt)
            }
            "expression_statement" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() != ";" {
                        return self.lower_statement(child);
                    }
                }
                None
            }
            "for_statement" => {
                let init = node
                    .child_by_field_name("init")
                    .and_then(|n| self.lower_statement(n));
                let cond = node
                    .child_by_field_name("condition")
                    .map(|n| self.lower_expr(n));
                let step = node
                    .child_by_field_name("update")
                    .and_then(|n| self.lower_statement(n));
                let body_node = node.child_by_field_name("body");
                let body = if let Some(b) = body_node {
                    if b.kind() == "block" {
                        self.lower_block(b)
                    } else if let Some(stmt) = self.lower_statement(b) {
                        self.module.alloc_block(IrBlock { stmts: vec![stmt] })
                    } else {
                        self.module.alloc_block(IrBlock::default())
                    }
                } else {
                    self.module.alloc_block(IrBlock::default())
                };

                let stmt = IrStmt::For {
                    init,
                    cond,
                    step,
                    body,
                };
                Some(self.module.alloc_stmt(stmt))
            }
            "while_statement" => {
                let cond = node
                    .child_by_field_name("condition")
                    .map(|n| self.lower_expr(n))
                    .unwrap_or_else(|| self.module.alloc_expr(IrExpr::LiteralBool(true)));
                let body_node = node.child_by_field_name("body");
                let body = if let Some(b) = body_node {
                    if b.kind() == "block" {
                        self.lower_block(b)
                    } else if let Some(stmt) = self.lower_statement(b) {
                        self.module.alloc_block(IrBlock { stmts: vec![stmt] })
                    } else {
                        self.module.alloc_block(IrBlock::default())
                    }
                } else {
                    self.module.alloc_block(IrBlock::default())
                };

                Some(self.module.alloc_stmt(IrStmt::While { cond, body }))
            }
            "if_statement" => {
                let cond = node
                    .child_by_field_name("condition")
                    .map(|n| self.lower_expr(n))
                    .unwrap_or_else(|| self.module.alloc_expr(IrExpr::LiteralBool(true)));

                let cons_node = node.child_by_field_name("consequence");
                let then_block = if let Some(c) = cons_node {
                    if c.kind() == "block" {
                        self.lower_block(c)
                    } else if let Some(stmt) = self.lower_statement(c) {
                        self.module.alloc_block(IrBlock { stmts: vec![stmt] })
                    } else {
                        self.module.alloc_block(IrBlock::default())
                    }
                } else {
                    self.module.alloc_block(IrBlock::default())
                };

                let else_block = node.child_by_field_name("alternative").map(|alt| {
                    if alt.kind() == "block" {
                        self.lower_block(alt)
                    } else if let Some(stmt) = self.lower_statement(alt) {
                        self.module.alloc_block(IrBlock { stmts: vec![stmt] })
                    } else {
                        self.module.alloc_block(IrBlock::default())
                    }
                });

                Some(self.module.alloc_stmt(IrStmt::If {
                    cond,
                    then_block,
                    else_block,
                }))
            }
            "return_statement" => {
                let mut cursor = node.walk();
                let mut ret_expr = None;
                for child in node.children(&mut cursor) {
                    if child.kind() != "return" && child.kind() != ";" {
                        ret_expr = Some(self.lower_expr(child));
                        break;
                    }
                }
                Some(self.module.alloc_stmt(IrStmt::Return(ret_expr)))
            }
            "break_statement" => Some(self.module.alloc_stmt(IrStmt::Break)),
            "continue_statement" => Some(self.module.alloc_stmt(IrStmt::Continue)),
            "local_variable_declaration" => {
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    if child.kind() == "variable_declarator" {
                        let var_name = child
                            .child_by_field_name("name")
                            .map(|d| self.text(d))
                            .unwrap_or_else(|| "var".to_string());
                        let value_node = child.child_by_field_name("value");
                        let val_expr = if let Some(v) = value_node {
                            self.lower_expr(v)
                        } else {
                            self.module.alloc_expr(IrExpr::LiteralInt(0))
                        };
                        return Some(self.module.alloc_stmt(IrStmt::Assign(var_name, val_expr)));
                    }
                }
                None
            }
            "assignment_expression" => {
                let left = node
                    .child_by_field_name("left")
                    .map(|n| self.text(n))
                    .unwrap_or_else(|| "var".to_string());
                let right_node = node.child_by_field_name("right");
                let op = node
                    .child_by_field_name("operator")
                    .map(|n| self.text(n))
                    .unwrap_or_else(|| "=".to_string());
                let right = if let Some(r) = right_node {
                    let r_expr = self.lower_expr(r);
                    match op.as_str() {
                        "+=" => {
                            let l_expr = self.module.alloc_expr(IrExpr::Var(left.clone()));
                            self.module
                                .alloc_expr(IrExpr::Binary(BinaryOp::Add, l_expr, r_expr))
                        }
                        "-=" => {
                            let l_expr = self.module.alloc_expr(IrExpr::Var(left.clone()));
                            self.module
                                .alloc_expr(IrExpr::Binary(BinaryOp::Sub, l_expr, r_expr))
                        }
                        "*=" => {
                            let l_expr = self.module.alloc_expr(IrExpr::Var(left.clone()));
                            self.module
                                .alloc_expr(IrExpr::Binary(BinaryOp::Mul, l_expr, r_expr))
                        }
                        "/=" => {
                            let l_expr = self.module.alloc_expr(IrExpr::Var(left.clone()));
                            self.module
                                .alloc_expr(IrExpr::Binary(BinaryOp::Div, l_expr, r_expr))
                        }
                        _ => r_expr,
                    }
                } else {
                    self.module.alloc_expr(IrExpr::LiteralInt(0))
                };
                Some(self.module.alloc_stmt(IrStmt::Assign(left, right)))
            }
            "update_expression" => {
                let mut cursor = node.walk();
                let mut var_name = "var".to_string();
                let mut op = "++".to_string();
                for child in node.children(&mut cursor) {
                    if child.kind() == "identifier" {
                        var_name = self.text(child);
                    } else if child.kind() == "++" || child.kind() == "--" {
                        op = self.text(child);
                    }
                }
                let one = self.module.alloc_expr(IrExpr::LiteralInt(1));
                let var_expr = self.module.alloc_expr(IrExpr::Var(var_name.clone()));
                let b_op = if op == "++" {
                    BinaryOp::Add
                } else {
                    BinaryOp::Sub
                };
                let new_expr = self.module.alloc_expr(IrExpr::Binary(b_op, var_expr, one));
                Some(self.module.alloc_stmt(IrStmt::Assign(var_name, new_expr)))
            }
            _ => {
                let expr = self.lower_expr(node);
                Some(self.module.alloc_stmt(IrStmt::Expr(expr)))
            }
        }
    }

    pub fn lower_expr(&mut self, node: Node<'a>) -> ExprId {
        match node.kind() {
            "decimal_integer_literal" | "integral_type" => {
                let val = self.text(node).parse::<i64>().unwrap_or(0);
                self.module.alloc_expr(IrExpr::LiteralInt(val))
            }
            "true" => self.module.alloc_expr(IrExpr::LiteralBool(true)),
            "false" => self.module.alloc_expr(IrExpr::LiteralBool(false)),
            "identifier" => {
                let name = self.text(node);
                self.module.alloc_expr(IrExpr::Var(name))
            }
            "binary_expression" => {
                let left_node = node.child_by_field_name("left");
                let right_node = node.child_by_field_name("right");
                let op_str = node
                    .child_by_field_name("operator")
                    .map(|n| self.text(n))
                    .unwrap_or_else(|| "+".to_string());

                let left = left_node
                    .map(|n| self.lower_expr(n))
                    .unwrap_or_else(|| self.module.alloc_expr(IrExpr::LiteralInt(0)));
                let right = right_node
                    .map(|n| self.lower_expr(n))
                    .unwrap_or_else(|| self.module.alloc_expr(IrExpr::LiteralInt(0)));

                let op = match op_str.as_str() {
                    "+" => BinaryOp::Add,
                    "-" => BinaryOp::Sub,
                    "*" => BinaryOp::Mul,
                    "/" => BinaryOp::Div,
                    "%" => BinaryOp::Mod,
                    "&" => BinaryOp::BitAnd,
                    "|" => BinaryOp::BitOr,
                    "^" => BinaryOp::BitXor,
                    "<<" => BinaryOp::Shl,
                    ">>" => BinaryOp::Shr,
                    "==" => BinaryOp::Eq,
                    "!=" => BinaryOp::Ne,
                    "<" => BinaryOp::Lt,
                    "<=" => BinaryOp::Le,
                    ">" => BinaryOp::Gt,
                    ">=" => BinaryOp::Ge,
                    "&&" => BinaryOp::And,
                    "||" => BinaryOp::Or,
                    _ => BinaryOp::Add,
                };

                self.module.alloc_expr(IrExpr::Binary(op, left, right))
            }
            "method_invocation" => {
                let fn_name = node
                    .child_by_field_name("name")
                    .map(|n| self.text(n))
                    .unwrap_or_else(|| "method".to_string());
                let args_node = node.child_by_field_name("arguments");
                let mut args = Vec::new();
                if let Some(args_list) = args_node {
                    let mut cursor = args_list.walk();
                    for child in args_list.children(&mut cursor) {
                        if child.kind() != "(" && child.kind() != ")" && child.kind() != "," {
                            args.push(self.lower_expr(child));
                        }
                    }
                }
                self.module.alloc_expr(IrExpr::Call(fn_name, args))
            }
            "array_access" => {
                let base = node
                    .child_by_field_name("array")
                    .map(|n| self.lower_expr(n))
                    .unwrap_or_else(|| self.module.alloc_expr(IrExpr::LiteralInt(0)));
                let index = node
                    .child_by_field_name("index")
                    .map(|n| self.lower_expr(n))
                    .unwrap_or_else(|| self.module.alloc_expr(IrExpr::LiteralInt(0)));
                self.module.alloc_expr(IrExpr::Index(base, index))
            }
            _ => {
                let text = self.text(node);
                self.module.alloc_expr(IrExpr::Var(text))
            }
        }
    }
}
