use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AstNode {
    pub kind: String,
    #[serde(default)]
    pub field_name: Option<String>,
    pub start_byte: usize,
    pub end_byte: usize,
    #[serde(default)]
    pub children: Vec<AstNode>,
}

impl AstNode {
    pub fn new(kind: impl Into<String>, start_byte: usize, end_byte: usize) -> Self {
        Self {
            kind: kind.into(),
            field_name: None,
            start_byte,
            end_byte,
            children: Vec::new(),
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn child_by_field_name(&self, name: &str) -> Option<&AstNode> {
        self.children
            .iter()
            .find(|c| c.field_name.as_deref() == Some(name))
    }

    pub fn text<'a>(&self, source: &'a [u8]) -> &'a str {
        if self.start_byte <= self.end_byte && self.end_byte <= source.len() {
            std::str::from_utf8(&source[self.start_byte..self.end_byte]).unwrap_or("")
        } else {
            ""
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn tree_sitter_to_ast(node: tree_sitter::Node) -> AstNode {
    tree_sitter_to_ast_with_field(node, None)
}

#[cfg(not(target_arch = "wasm32"))]
fn tree_sitter_to_ast_with_field(node: tree_sitter::Node, field_name: Option<String>) -> AstNode {
    let mut cursor = node.walk();
    let mut children = Vec::new();
    for (i, child) in node.children(&mut cursor).enumerate() {
        let child_field = node.field_name_for_child(i as u32).map(|s| s.to_string());
        children.push(tree_sitter_to_ast_with_field(child, child_field));
    }
    AstNode {
        kind: node.kind().to_string(),
        field_name,
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        children,
    }
}
