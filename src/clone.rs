//! Try traits for [`core::clone`]

/// The try trait for [`Clone`](core::clone::Clone).
pub trait TryClone : Sized {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`Clone::clone`](core::clone::Clone::clone)
	fn try_clone(&self) -> Result<Self, Self::Error>;

	/// The fallible equivalent of [`Clone::clone_from`](core::clone::Clone::clone_from)
	#[inline]
	fn try_clone_from(&mut self, source: &Self) -> Result<(), Self::Error> {
		Ok(*self = source.try_clone()?)
	}
}

impl<T: Clone> TryClone for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_clone(&self) -> Result<Self, Self::Error> {
		Ok(self.clone())
	}

	#[inline]
	fn try_clone_from(&mut self, source: &Self) -> Result<(), Self::Error> {
		Ok(self.clone_from(source))
	}
}
