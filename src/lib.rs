#![doc = include_str!("../README.md")]

#[cfg(feature = "builtins")]
pub mod builtins;

#[cfg(not(feature = "builtins"))]
mod builtins {
    compile_error!("Unfortunately, the `builtins` feature is currently required (by the generated AST)");
}


mod span;

pub use self::span::Span;

/// Indicates that a type is associated with a [Span]
///
/// This is required of all [AstNode]s
pub trait Spanned {
    /// 
    fn span(&self) -> Span;
}

pub trait AstNode: Clone + Eq {

}