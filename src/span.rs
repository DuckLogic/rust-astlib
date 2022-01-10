mod sealed {
    /// Implementation detail: A `Span` type.
    ///
    /// Right now this is an implementation detail.
    ///
    /// All methods on this trait should be available in all implementations.
    pub trait ISpan: Copy + Debug + Eq + Hash + 'static {
        /// Create a marker span, indicating that location information is missing.
        pub fn missing() -> Self;
        /// Create a span from the specified byte indices
        pub fn from_byte_indices(start: u64, end: u64) -> Self;
        /// Resolve the start index of this span.
        pub fn start_index(&self) -> usize;
        /// Resolve the end index of this span.
        pub fn end_index(&self) -> usize;
    }
}

pub use self::backend::Span;

#[cfg(feature = "codespan")]
#[doc(hidden)]
pub mod backend {
    compile_error!("codespan support is not yet implemented");
}

#[cfg(not(feature = "codespan"))]
#[doc(hidden)]
pub mod backend {
    use super::sealed::ISpan;
    #[cfg(feature = "serde")]
    use serde::{Serialize, Deserialize};
    /// The default implementation of a span.
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    #[cfg_attr(feature = "serde-1", derive(Serialize, Deserialize))]
    pub struct Span {
        start: usize,
        end: usize
    }
    #[inherent]
    impl ISpan for Span {
        #[inline]
        pub fn missing() -> Self {
            Span { start: 0, end: 0 }
        }
        #[inline]
        pub fn from_byte_indices(start: u64, end: u64) -> Self {
            assert!(start <= end);
            Span { start, end }
        }
        #[inline]
        pub fn start_index(&self) -> usize {
            self.start
        }
        #[inline]
        pub fn end_index(&self) -> usize {
            self.end
        }
    }
}