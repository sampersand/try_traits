//! Fallible versions of the std lib's traits.
//! 
//! For most cases, these traits are probably not what you need, and simply with careful use of
//! generics you can use the builtin variants. And, when you can't, it's preferred to write "try"
//! functions directly, such as [`File`](std::fs::File)'s [`try_clone`](
//! std::fs::File::try_clone) method.
//!
//! Instead, these are meant to be used in APIs that require the ability to abstract over operations
//! that could be fallible.
//!
//! # On Traits That Weren't Included
//! There's a few reasons a core trait might not have been included:
//! - They already completely support a form of failibility, and as such it wouldn't make sense to 
//!   add a try version of them. (examples: `FromStr`).
//! - Traits that it doesn't make much sense for them to be failible, such as [`core::marker`]
//!   traits like [`Copy`] and [`Send`].
//! - They simply hadn't been released yet when this was written. If this is the case, file an issue
//!   (or submit a pull request!)
#![no_std]
#![forbid(unsafe_code)]
#![allow(
	clippy::unit_arg, // to allow for simple `Ok(x = y)` return values.
	clippy::module_name_repetitions, // the core lib does this all over the place
)]

/// The value used for infallible conversions.
pub type Infallible = core::convert::Infallible;

pub mod clone;
pub mod borrow;
pub mod cmp;
pub mod ops;
pub mod hash;
pub mod convert;
pub mod default;
pub mod iter;
// Should we even include `slice`, `any`, or `future`?
