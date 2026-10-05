pub mod arena;
pub mod ast;

#[cfg(not(target_arch = "wasm32"))]
pub mod normalize_cpp;
#[cfg(not(target_arch = "wasm32"))]
pub mod normalize_java;

#[cfg(not(target_arch = "wasm32"))]
pub use normalize_cpp::CppNormalizer;
#[cfg(not(target_arch = "wasm32"))]
pub use normalize_java::JavaNormalizer;
