#![doc = include_str!("../README.md")]
use std::hash::Hash;
use std::fmt::Debug;

#[cfg(feature = "builtins")]
pub mod builtins;

#[cfg(not(feature = "builtins"))]
mod builtins {
    compile_error!("Unfortunately, the `builtins` feature is currently required (by the generated AST)");
}


mod span;
pub mod lisp;

pub use self::span::{Span, Spanned};

pub trait AstNode: Clone + Eq + Hash + Debug + MaybeSerialize + lisp::PrintLisp {

}

/// Indicates a type conditionally supports serde Serialization
#[cfg(feature = "serde1")]
pub trait MaybeSerialize: Serialize {}
#[cfg(feature = "serde1")]
impl<T: serde::Serialize + ?Sized> MaybeSerialize for T {}
/// Indicates a type conditionally supports serde Serialization
#[cfg(not(feature = "serde1"))]
pub trait MaybeSerialize {

}
#[cfg(not(feature = "serde1"))]
impl<T: ?Sized> MaybeSerialize for T {}