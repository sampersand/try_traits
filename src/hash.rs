//! Try traits for [`core::hash`].
//!
//! Note that [`BuildHasher`](core::hash::BuildHasher) isn't implemented because each call to
//! `build_hasher` should return identical values.
//!
//! [`Hasher`] also was not given a `Try` variant as I can see no real use for it (and as it
//! complicates automatic implementations), , but if there's good reason for one it'll be added.
use core::hash::{Hash, Hasher};

/// The try trait for [`Hash`].
pub trait TryHash {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`Hash::hash`].
	fn try_hash<H: Hasher>(&self, state: &mut H) -> Result<(), Self::Error>;

	/// The fallible equivalent of [`Hash::hash_slice`].
	fn try_hash_slice<H: Hasher>(data: &[Self], state: &mut H) -> Result<(), Self::Error>
	where
		Self: Sized
	{
		for piece in data {
			piece.try_hash(state)?;
		}

		Ok(())
	}
}

impl<T: Hash> TryHash for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_hash<H: Hasher>(&self, state: &mut H) -> Result<(), Self::Error> {
		Ok(self.hash(state))
	}

	#[inline]
	fn try_hash_slice<H: Hasher>(data: &[Self], state: &mut H) -> Result<(), Self::Error> {
		Ok(Self::hash_slice(data, state))
	}
}
