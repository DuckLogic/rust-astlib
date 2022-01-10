//! Builtin implementations of `Constant` and `Ident` types
//!
//! These are builtin to the ASDL langauge, and some default implementations are provided here.
//!
//! They may or may not be appropriate for your use case.
use std::fmt::{self, Formatter, Display, Debug};
use derivative::Derivative;

use crate::{Span, Spanned, AstNode};
use crate::lisp::{PrintLisp, LispPrintOptions};

#[derive(Clone, Derivative)]
#[derivative(Eq, PartialEq, Hash, Debug="transparent")]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde1", serde(transparent))]
pub struct Constant {
    pub kind: ConstantKind,
    #[derivative(PartialEq="ignore", Hash="ignore", Debug="ignore")]
    #[cfg_attr(feature = "serde1", serde(skip, default = "Span::missing"))]
    pub span: Span,
}
impl PrintLisp for Constant {
    #[inline]
    fn to_lexpr(&self, opts: &LispPrintOptions) -> lexpr::Value {
        self.kind.to_lexpr(opts)
    }
}
impl Spanned for Constant {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}
impl AstNode for Constant {}
#[derive(Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde1", serde(untagged))] // TODO: How to distinguish between identifiers and everything else
pub enum ConstantKind {
    String(Box<str>),
    Symbol(Ident),
    Int(i64),
    Float(ordered_float::OrderedFloat<f64>),
    Bool(bool),
    Null
}
impl PrintLisp for ConstantKind {
    #[inline]
    fn to_lexpr(&self, opts: &LispPrintOptions) -> lexpr::Value {
        match *self {
            ConstantKind::String(ref s) => lexpr::Value::String(s.clone()),
            ConstantKind::Symbol(ref i) => i.to_lexpr(opts),
            ConstantKind::Int(i) => lexpr::Value::Number(i.into()),
            ConstantKind::Float(f) => lexpr::Value::Number(f.0.into()),
            ConstantKind::Bool(b) => lexpr::Value::Bool(b.into()),
            ConstantKind::Null => lexpr::Value::Nil
        }
    }
}
impl Debug for ConstantKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ConstantKind::String(ref s) => write!(f, "{:?}", s),
            ConstantKind::Symbol(ref s) => Display::fmt(s, f),
            ConstantKind::Int(val) => write!(f, "{}", val),
            ConstantKind::Float(val) => write!(f, "{:?}", val.0),
            ConstantKind::Bool(true) => f.write_str("true"),
            ConstantKind::Bool(false) => f.write_str("false"),
            ConstantKind::Null => f.write_str("null"),
        }
    }
}

/// An identifier.
///
/// This is a string and a span.
///
/// This performs no validation.
/// That is up to the language implementation.
#[derive(Clone, Derivative)]
#[derivative(Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde1", serde(transparent))]
pub struct Ident {
    value: kstring::KString,
    #[cfg_attr(feature = "serde1", serde(skip, default = "Span::missing"))]
    #[derivative(Hash="ignore", PartialEq="ignore")]
    pub span: Span,
}
impl Ident {
    /// Create a new identifier, associated with a specified Span
    #[inline]
    pub fn new(value: impl Into<Self>, span: Span) -> Self {
        value.into().with_span(span)
    }
    /// Create a copy of this identifier, modified to have the specified Span
    pub fn with_span(self, span: Span) -> Ident {
        Ident { span, ..self }
    }
    /// Slice this identifier, viewing it as a string.
    #[inline]
    pub fn as_str(&self) -> &str {
        &*self.value
    }
}
impl AstNode for Ident {}
impl PrintLisp for Ident {
    fn to_lexpr(&self, _opts: &LispPrintOptions) -> lexpr::Value {
        lexpr::Value::String(self.as_str().into())
    }
}
impl Spanned for Ident {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}
impl AsRef<str> for Ident {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl From<&'static str> for Ident {
    #[inline]
    fn from(s: &'static str) -> Self {
        Ident { value: s.into(), span: Span::missing() }
    }
} 
impl From<String> for Ident {
    #[inline]
    fn from(s: String) -> Self {
        Ident { value: s.into(), span: Span::missing() }
    }
}
impl Display for Ident {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.value, f)
    }
}
impl Debug for Ident {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Presumably, identifiers can be `Display`ed directly
        Display::fmt(&self.value, f)
    }
}