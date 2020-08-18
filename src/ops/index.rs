use core::ops::{Index, IndexMut};

///
pub trait TryIndex<Idx> {
	/// The type returned in the event of an error.
	type Error;

	/// The type returned after performing the operation.
	type Output: ?Sized;

	/// The fallible equivalent of [`Index::index`].
	fn try_index(&self, index: Idx) -> Result<&Self::Output, Self::Error>;
}

/// The try trait for [`IndexMut`].
pub trait TryIndexMut<Idx> : TryIndex<Idx> {
	/// The fallible equivalent of [`Index::index`].
	fn try_index_mut(&mut self, index: Idx) -> Result<&mut Self::Output, Self::Error>;
}

impl<T: Index<Idx>, Idx> TryIndex<Idx> for T {
	type Error = crate::Infallible;
	type Output = <Self as Index<Idx>>::Output;

	fn try_index(&self, index: Idx) -> Result<&Self::Output, Self::Error> {
		Ok(self.index(index))
	}
}

impl<T: IndexMut<Idx>, Idx> TryIndexMut<Idx> for T {
	fn try_index_mut(&mut self, index: Idx) -> Result<&mut Self::Output, Self::Error> {
		Ok(self.index_mut(index))
	}
}
