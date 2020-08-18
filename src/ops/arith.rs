#[allow(unused)]
use core::ops::{
	Neg,
	Add, AddAssign, Sub, SubAssign,
	Mul, MulAssign, Div, DivAssign, Rem, RemAssign
};

/// The try trait for [`Neg`].
pub trait TryNeg {
	/// The type returned in the event of an error.
	type Error;

	/// The type returned after performing the operation.
	type Output;

	/// The fallible equivalent of [`Neg::neg`].
	fn try_neg(self) -> Result<Self::Output, Self::Error>;
}

/// The try trait for [`Add`].
pub trait TryAdd<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The type returned after performing the operation.
	type Output;

	/// The fallible equivalent of [`Add::add`].
	fn try_add(self, other: Rhs) -> Result<Self::Output, Self::Error>;
}

/// The try trait for [`Sub`].
pub trait TrySub<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The type returned after performing the operation.
	type Output;

	/// The fallible equivalent of [`Sub::sub`].
	fn try_sub(self, other: Rhs) -> Result<Self::Output, Self::Error>;
}

/// The try trait for [`Mul`].
pub trait TryMul<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The type returned after performing the operation.
	type Output;

	/// The fallible equivalent of [`Mul::mul`].
	fn try_mul(self, other: Rhs) -> Result<Self::Output, Self::Error>;
}

/// The try trait for [`Div`].
pub trait TryDiv<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The type returned after performing the operation.
	type Output;

	/// The fallible equivalent of [`Div::div`].
	fn try_div(self, other: Rhs) -> Result<Self::Output, Self::Error>;
}

/// The try trait for [`Rem`].
pub trait TryRem<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The type returned after performing the operation.
	type Output;

	/// The fallible equivalent of [`Rem::rem`].
	fn try_rem(self, other: Rhs) -> Result<Self::Output, Self::Error>;
}

/// The try trait for [`AddAssign`].
pub trait TryAddAssign<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`AddAssign::add_assign`].
	fn try_add_assign(&mut self, other: Rhs) -> Result<(), Self::Error>;
}

/// The try trait for [`SubAssign`].
pub trait TrySubAssign<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`SubAssign::sub_assign`].
	fn try_sub_assign(&mut self, other: Rhs) -> Result<(), Self::Error>;
}

/// The try trait for [`MulAssign`].
pub trait TryMulAssign<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`MulAssign::mul_assign`].
	fn try_mul_assign(&mut self, other: Rhs) -> Result<(), Self::Error>;
}

/// The try trait for [`DivAssign`].
pub trait TryDivAssign<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`DivAssign::div_assign`].
	fn try_div_assign(&mut self, other: Rhs) -> Result<(), Self::Error>;
}

/// The try trait for [`RemAssign`].
pub trait TryRemAssign<Rhs = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`RemAssign::rem_assign`].
	fn try_rem_assign(&mut self, other: Rhs) -> Result<(), Self::Error>;
}

impl<T: Add<Rhs>, Rhs> TryAdd<Rhs> for T {
	type Error = crate::Infallible;
	type Output = <Self as Add<Rhs>>::Output;

	#[inline]
	fn try_add(self, other: Rhs) -> Result<Self::Output, Self::Error> {
		Ok(self + other)
	}
}

impl<T: Sub<Rhs>, Rhs> TrySub<Rhs> for T {
	type Error = crate::Infallible;
	type Output = <Self as Sub<Rhs>>::Output;

	#[inline]
	fn try_sub(self, other: Rhs) -> Result<Self::Output, Self::Error> {
		Ok(self - other)
	}
}

impl<T: Mul<Rhs>, Rhs> TryMul<Rhs> for T {
	type Error = crate::Infallible;
	type Output = <Self as Mul<Rhs>>::Output;

	#[inline]
	fn try_mul(self, other: Rhs) -> Result<Self::Output, Self::Error> {
		Ok(self * other)
	}
}

impl<T: Div<Rhs>, Rhs> TryDiv<Rhs> for T {
	type Error = crate::Infallible;
	type Output = <Self as Div<Rhs>>::Output;

	#[inline]
	fn try_div(self, other: Rhs) -> Result<Self::Output, Self::Error> {
		Ok(self / other)
	}
}

impl<T: Rem<Rhs>, Rhs> TryRem<Rhs> for T {
	type Error = crate::Infallible;
	type Output = <Self as Rem<Rhs>>::Output;

	#[inline]
	fn try_rem(self, other: Rhs) -> Result<Self::Output, Self::Error> {
		Ok(self % other)
	}
}

impl<T: AddAssign<Rhs>, Rhs> TryAddAssign<Rhs> for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_add_assign(&mut self, other: Rhs) -> Result<(), Self::Error> {
		Ok(self.add_assign(other))
	}
}

impl<T: SubAssign<Rhs>, Rhs> TrySubAssign<Rhs> for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_sub_assign(&mut self, other: Rhs) -> Result<(), Self::Error> {
		Ok(self.sub_assign(other))
	}
}

impl<T: MulAssign<Rhs>, Rhs> TryMulAssign<Rhs> for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_mul_assign(&mut self, other: Rhs) -> Result<(), Self::Error> {
		Ok(self.mul_assign(other))
	}
}

impl<T: DivAssign<Rhs>, Rhs> TryDivAssign<Rhs> for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_div_assign(&mut self, other: Rhs) -> Result<(), Self::Error> {
		Ok(self.div_assign(other))
	}
}

impl<T: RemAssign<Rhs>, Rhs> TryRemAssign<Rhs> for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_rem_assign(&mut self, other: Rhs) -> Result<(), Self::Error> {
		Ok(self.rem_assign(other))
	}
}
