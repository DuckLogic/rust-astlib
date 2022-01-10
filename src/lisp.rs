//! Support for converting the AST into lisp sexpr
//!
//! This is on by default.

#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct LispPrintOptions {

}

pub trait PrintLisp {
    /// Convert this ast node into an equivalent lisp value
    fn to_lexpr(&self, opts: &LispPrintOptions) -> lexpr::Value;
}
