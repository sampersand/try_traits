#[allow(unused)]
use core::ops::{
	Not,
	BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign,
	Shl, ShlAssign, Shr, ShrAssign
};

/// The try trait for [`Not`].
pub trait TryNot {
	/// The type returned in the event of an error.
	type Error;

	/// The type returned after performing the operation.
	type Output;

	/// The fallible equivalent of [`Not::not`].
	fn try_not(self) -> Result<Self::Output, Self::Error>;
}

/// The try trait for [`BitAnd`].
pub trait TryBitAnd<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The type returned after performing the operation.
	type Output;

	/// The fallible equivalent of [`BitAnd::bitand`].
	fn try_bitand(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

/// The try trait for [`BitOr`].
pub trait TryBitOr<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The type returned after performing the operation.
	type Output;

	/// The fallible equivalent of [`BitOr::bitor`].
	fn try_bitor(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

/// The try trait for [`BitXor`].
pub trait TryBitXor<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The type returned after performing the operation.
	type Output;

	/// The fallible equivalent of [`BitXor::bitxor`].
	fn try_bitxor(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

/// The try trait for [`Shl`].
pub trait TryShl<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The type returned after performing the operation.
	type Output;

	/// The fallible equivalent of [`Shl::shl`].
	fn try_shl(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

/// The try trait for [`Shr`].
pub trait TryShr<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The type returned after performing the operation.
	type Output;

	/// The fallible equivalent of [`Shr::shr`].
	fn try_shr(self, rhs: Rhs) -> Result<Self::Output, Self::Error>;
}

/// The try trait for [`BitAndAssign`].
pub trait TryBitAndAssign<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`BitAndAssign::bitand_assign`].
	fn try_bitand_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error>;
}

/// The try trait for [`BitOrAssign`].
pub trait TryBitOrAssign<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`BitOrAssign::bitor`].
	fn try_bitor_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error>;
}

/// The try trait for [`BitXorAssign`].
pub trait TryBitXorAssign<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`BitXorAssign::bitxor_assign`].
	fn try_bitxor_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error>;
}

/// The try trait for [`ShlAssign`].
pub trait TryShlAssign<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`ShlAssign::shl_assign`].
	fn try_shl_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error>;
}

/// The try trait for [`ShrAssign`].
pub trait TryShrAssign<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`ShrAssign::shr_assign`].
	fn try_shr_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error>;
}

impl<T: Not> TryNot for T {
	type Error = crate::Infallible;
	type Output = <Self as Not>::Output;

	#[inline]
	fn try_not(self) -> Result<Self::Output, Self::Error> {
		Ok(!self)
	}
}

impl<T: BitAnd<Rhs>, Rhs> TryBitAnd<Rhs> for T {
	type Error = crate::Infallible;
	type Output = <Self as BitAnd<Rhs>>::Output;

	#[inline]
	fn try_bitand(self, rhs: Rhs) -> Result<Self::Output, Self::Error> {
		Ok(self & rhs)
	}
}

impl<T: BitOr<Rhs>, Rhs> TryBitOr<Rhs> for T {
	type Error = crate::Infallible;
	type Output = <Self as BitOr<Rhs>>::Output;

	#[inline]
	fn try_bitor(self, rhs: Rhs) -> Result<Self::Output, Self::Error> {
		Ok(self | rhs)
	}
}

impl<T: BitXor<Rhs>, Rhs> TryBitXor<Rhs> for T {
	type Error = crate::Infallible;
	type Output = <Self as BitXor<Rhs>>::Output;

	#[inline]
	fn try_bitxor(self, rhs: Rhs) -> Result<Self::Output, Self::Error> {
		Ok(self ^ rhs)
	}
}

impl<T: Shl<Rhs>, Rhs> TryShl<Rhs> for T {
	type Error = crate::Infallible;
	type Output = <Self as Shl<Rhs>>::Output;

	#[inline]
	fn try_shl(self, rhs: Rhs) -> Result<Self::Output, Self::Error> {
		Ok(self << rhs)
	}
}

impl<T: Shr<Rhs>, Rhs> TryShr<Rhs> for T {
	type Error = crate::Infallible;
	type Output = <Self as Shr<Rhs>>::Output;

	#[inline]
	fn try_shr(self, rhs: Rhs) -> Result<Self::Output, Self::Error> {
		Ok(self >> rhs)
	}
}

impl<T: BitAndAssign<Rhs>, Rhs> TryBitAndAssign<Rhs> for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_bitand_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error> {
		Ok(self.bitand_assign(rhs))
	}
}

impl<T: BitOrAssign<Rhs>, Rhs> TryBitOrAssign<Rhs> for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_bitor_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error> {
		Ok(self.bitor_assign(rhs))
	}
}

impl<T: BitXorAssign<Rhs>, Rhs> TryBitXorAssign<Rhs> for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_bitxor_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error> {
		Ok(self.bitxor_assign(rhs))
	}
}

impl<T: ShlAssign<Rhs>, Rhs> TryShlAssign<Rhs> for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_shl_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error> {
		Ok(self.shl_assign(rhs))
	}
}

impl<T: ShrAssign<Rhs>, Rhs> TryShrAssign<Rhs> for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_shr_assign(&mut self, rhs: Rhs) -> Result<(), Self::Error> {
		Ok(self.shr_assign(rhs))
	}
}
