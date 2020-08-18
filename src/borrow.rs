//! Try traits for [`core::borrow`].

use core::borrow::{Borrow, BorrowMut};

/// The try trait for [`Borrow`].
pub trait TryBorrow<Borrowed: ?Sized> {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`Borrow::borrow`].
	fn try_borrow(&self) -> Result<&Borrowed, Self::Error>;
}

/// The try trait for [`BorrowMut`].
pub trait TryBorrowMut<Borrowed: ?Sized> : TryBorrow<Borrowed> {
	/// The fallible equivalent of [`BorrowMut::try_borrowMut`].
	fn try_borrow_mut(&mut self) -> Result<&mut Borrowed, <Self as TryBorrow<Borrowed>>::Error>;
}

impl<T: Borrow<Borrowed>, Borrowed: ?Sized> TryBorrow<Borrowed> for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_borrow(&self) -> Result<&Borrowed, Self::Error> {
		Ok(self.borrow())
	}
}

impl<T: BorrowMut<Borrowed>, Borrowed: ?Sized> TryBorrowMut<Borrowed> for T {
	#[inline]
	fn try_borrow_mut(&mut self) -> Result<&mut Borrowed, Self::Error> {
		Ok(self.borrow_mut())
	}
}
