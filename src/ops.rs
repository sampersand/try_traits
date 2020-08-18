//! Try traits for [`core::ops`].
//!
//! This is quite useful when you want to perform a logical operation on two values, but there's the
//! possibility of the operation failing.
//!
//! Note that none of these actually change how Rust syntax works: You'll need to do
//! `12.try_add(13)?`.
mod arith;
mod bit;
mod index;

// `deref` is missing because `try_deref` makes no sense---the whole point is it's infallible
// `function` is missing because you simply change the return type
// `range` is missing bc the only triat is `RangeBounds`, but if there's a valid use i'll add it.

pub use arith::*;
pub use bit::*;
pub use index::*;
