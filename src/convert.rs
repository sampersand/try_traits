//! Try traits for [`core::convert`].
//!
//! Note that [`TryFrom`](core::convert::TryFrom) and [`TryInto`](core::convert::TryInto) are
//! missing, as they're already a part of the core library.

/// The try trait for [`AsRef`].
pub trait TryAsRef<T: ?Sized> {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`AsRef::as_ref`].
	fn try_as_ref(&self) -> Result<&T, Self::Error>;
}

/// The try trait for [`AsMut`].
pub trait TryAsMut<T: ?Sized> {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`AsMut::as_mut`].
	fn try_as_mut(&mut self) -> Result<&mut T, Self::Error>;
}
