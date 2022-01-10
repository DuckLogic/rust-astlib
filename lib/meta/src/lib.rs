//! A library for generating ASTs from ASDL files.
//!
//! Unfortunatley, this currently requires delegating
//! to python scripts right now.
//!
//! This is only a build dependency, not a runtime depdency.

#[path = "bootstrap_ast.rs"]
mod ast;

pub struct AstConfig {
    
}