#![deny(missing_docs)]

//! Write-only references.
//!
//! Many functions in Rust's standard library, such as `char::encode_utf8`, take
//! a mutable reference that they only ever write to.
//!
//! This crate provides a way to express this guarantee:
//!
//! - `WriteRef<T>` provides a single method, `write`. By taking this as a
//!   parameter, a function guarantees that it will only ever write to it.
//! - `WriteSlice<T>` works similarly, but it allows writing only to individual
//!   elements. This is useful for functions that write to a provided buffer,
//!   such as `char::encode_utf8`.
//!
//! Most functions should not take a `WriteRef` or `WriteSlice` directly;
//! instead, they should take an `impl Into<WriteRef<'a, T>>` so that callers
//! can pass in a `&mut T`.

use std::convert::From;

/// Represents a write-only reference.
///
/// It is generally advised to take an `impl Into<WriteRef>` instead of a
/// `WriteRef` itself in APIs so that callers can pass in a mutable reference.
///
/// # Examples
///
/// ```
/// # extern crate write_ref;
/// # use write_ref::WriteRef;
/// # fn main() {
/// let mut a = 3;
/// {
///     let mut a_ref = WriteRef::from(&mut a);
///     a_ref.write(0);
/// }
/// assert_eq!(a, 0);
/// # }
/// ```
pub struct WriteRef<'a, T: 'a>(&'a mut T);

impl<'a, T: 'a> WriteRef<'a, T> {
    /// Write a value to this reference.
    pub fn write(&mut self, val: T) {
        *self.0 = val;
    }
}

impl<'a, T: 'a> From<&'a mut T> for WriteRef<'a, T> {
    fn from(inner: &'a mut T) -> Self {
        WriteRef(inner)
    }
}

/// Represents a write-only buffer.
///
/// You only write to individual elements of this slice; you can't modify the
/// slice itself or read from its elements.
///
/// It is generally advised to take an `impl Into<WriteSlice>` instead of a
/// `WriteSlice` itself in APIs so that callers can pass in a a mutable
/// reference.
///
/// # Examples
///
/// ```
/// # extern crate write_ref;
/// # use write_ref::WriteSlice;
/// fn copy<T>(input: &[T], output: impl Into<WriteSlice<T>>) {
///     for (i, val) in input.iter().enumerate() {
///         output.write(i, val);
///     }
/// }
/// fn main() {
///     let input = [1, 2, 3];
///     let output = [7, 1, 9];
///     copy(&input, &mut output as &mut [_]);
///     assert_eq!(input, output);
/// }
/// ```
pub struct WriteSlice<'a, T: 'a>(&'a mut [T]);

impl<'a, T: 'a> WriteSlice<'a, T> {
    /// Write a value to an element of this slice.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is out of bounds.
    pub fn write(&mut self, idx: usize, val: T) {
        self.0[idx] = val;
    }
}

impl<'a, T: 'a> From<&'a mut [T]> for WriteSlice<'a, T> {
    fn from(inner: &'a mut [T]) -> Self {
        WriteSlice(inner)
    }
}
