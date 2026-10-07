pub mod ast_node;

pub use ast_node::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Language {
    Cpp,
    Java,
}
