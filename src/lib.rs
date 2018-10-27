//! Many functions in Rust's standard library, such as `char::encode_utf8`,
//! take a mutable reference that they only ever write to.
//!
//! This crate provides a way to express this guarantee:
//!
//! - The `WriteRef` trait provides a single method, `write`. It is implemented
//! only for `&mut T`. By taking a generic parameter with the `WriteRef` trait
//! bound, a function allows callers to pass in mutable references, but
//! guarantees that it can only write to them.
//! - The `WriteSlice` trait works similarly, being implemented only for
//! `&mut &

#![deny(missing_docs)]

mod sealed {
    pub trait Sealed {}
    impl<'a, T: 'a> Sealed for &'a mut T {}

    pub trait SealedSlice {}
    impl<'a, T: 'a> SealedSlice for &'a mut [T] {}
}

/// Represents a write-only reference.
///
/// This trait is implemented for all `&mut T`. To provide a guarantee that your
/// function will only write to a reference, use this trait as a generic bound.
///
/// This trait is sealed, so you cannot add your own implementations.
///
/// Example:
///
/// ```
/// # extern crate write_ref;
/// # use write_ref::WriteRef;
/// use std::default::Default;
///
/// fn clear<T: Default>(mut r: impl WriteRef<To=T>) {
///     r.write(Default::default());
/// }
/// 
/// fn main() {
///     let mut counter = 4;
///     clear(&mut counter);
///     assert_eq!(counter, 0);
/// }
/// ```
pub trait WriteRef: sealed::Sealed {
    /// The inner type.
    type To;
    /// Write a value of type `To` to this reference.
    fn write(&mut self, Self::To);
}

/// The sole implementation of `WriteRef`.
impl<'a, T: 'a> WriteRef for &'a mut T {
    type To = T;
    fn write(&mut self, t: T) {
        **self = t;
    }
}

/// Represents a write-only slice.
///
/// This trait is implemented for all `&mut [T]`. To provide a guarantee that
/// your function will only write to a slice, use this trait as a generic bound.
///
/// This trait is sealed, so you cannot add your own implementations.
///
/// Example:
///
/// ```
/// # extern crate write_ref;
/// # use write_ref::WriteSlice;
///
/// fn copy_buffer<T: Copy>(input: &[T], mut output: impl WriteSlice<Of=T>) {
///     for (i, val) in input.iter().enumerate() {
///         output.write_elem(i, *val);
///     }
/// }
/// 
/// fn main() {
///     let input = [1, 2, 3];
///     let mut output = [7, 1, 9];
///
///     copy_buffer(&input, &mut output);
///
///     assert_eq!(input, output);
/// }
/// ```
pub trait WriteSlice: sealed::SealedSlice {
    /// The elements of the write-only slice.
    type Of;
    /// Write to an element of the slice. Panic if the index is out-of-bounds.
    fn write_elem(&mut self, usize, Self::Of);
}

/// The sole implementation of `WriteSlice`.
impl<'a, T: 'a> WriteSlice for &'a mut [T] {
    type Of = T;
    fn write_elem(&mut self, idx: usize, t: T) {
        self[idx] = t;
    }
}
