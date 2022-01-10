//! Builtin implementations of `Constant` and `Ident` types
//!
//! These are builtin to the ASDL langauge, and some default implementations are provided here.
//!
//! They may or may not be appropriate for your use case.
use std::fmt::{self, Formatter, Display, Debug};

pub struct Constant {
    pub kind: ConstantKind,
    pub span: Span,
}
impl Spanned {
    
}
pub enum ConstantKind {
    String(String),
    Int(i64),
    Float(f64)
}

/// An identifier.
///
/// This is a string and a span.
///
/// This performs no validation.
/// That is up to the language implementation.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Ident {
    value: kstring::KString,
    pub span: Span,
}
impl Ident {
    
    /// Create a copy of this identifier, modified to have the specified Span
    pub fn with_span(self, span: Span) -> Ident {
        Ident { span, ..self }
    }
    /// Slice this identifier, viewing it as a string.
    #[inline]
    pub fn as_str(&self) -> &str {
        &*self.0
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
    fn from(s: &'static str) {
        Ident(s.into())
    }
} 
impl From<String> for Ident {
    #[inline]
    fn from(s: String) {
        Ident(s.into())
    }
}
impl Display for Ident {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}
impl Debug for Ident {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Presumably, identifiers can be `Display`ed directly
        Display::fmt(&self.0, f)
    }
}