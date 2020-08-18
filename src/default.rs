//! Try traits for [`core::default`].

/// The try trait for [`Default`].
pub trait TryDefault : Sized {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`Default::default`].
	fn try_default() -> Result<Self, Self::Error>;
}


impl<T: Default> TryDefault for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_default() -> Result<Self, Self::Error> {
		Ok(Self::default())
	}
}
