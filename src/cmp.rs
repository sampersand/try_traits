//! Try traits for [`core::cmp`].

use core::cmp::{Ord, PartialOrd, Ordering};

/// The try trait for [`PartialEq`].
pub trait TryPartialEq<Rhs: ?Sized = Self> {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`PartialEq::eq`].
	fn try_eq(&self, other: &Rhs) -> Result<bool, Self::Error>;

	/// The fallible equivalent of [`PartialEq::ne`].
	#[inline]
	fn try_ne(&self, other: &Rhs) -> Result<bool, Self::Error> {
		Ok(!self.try_eq(other)?)
	}
}

/// The try trait for [`Eq`].
///
/// That is, if the `try_eq` method returns `Ok`, then the requirements of `Eq` hold.
pub trait TryEq : TryPartialEq<Self> {}

/// The try trait for [`PartialOrd`].
pub trait TryPartialOrd<Rhs: ?Sized = Self> : TryPartialEq<Rhs> {
	/// The fallible equivalent of [`PartialOrd::partial_cmp`].
	fn try_partial_cmp(&self, other: &Rhs) -> Result<Option<Ordering>, Self::Error>;

	/// The fallible equivalent of [`PartialOrd::lt`].
	fn try_lt(&self, other: &Rhs) -> Result<bool, Self::Error> {
		Ok(matches!(self.try_partial_cmp(other)?, Some(Ordering::Less)))
	}

	/// The fallible equivalent of [`PartialOrd::le`].
	fn try_le(&self, other: &Rhs) -> Result<bool, Self::Error> {
		Ok(!matches!(self.try_partial_cmp(other)?, Some(Ordering::Greater)))
	}

	/// The fallible equivalent of [`PartialOrd::gt`].
	fn try_gt(&self, other: &Rhs) -> Result<bool, Self::Error> {
		Ok(matches!(self.try_partial_cmp(other)?, Some(Ordering::Greater)))
	}

	/// The fallible equivalent of [`PartialOrd::gt`].
	fn try_ge(&self, other: &Rhs) -> Result<bool, Self::Error> {
		Ok(!matches!(self.try_partial_cmp(other)?, Some(Ordering::Less)))
	}
}

/// The try trait for [`Ord`].
pub trait TryOrd : TryEq + TryPartialOrd<Self> {
	/// The fallible equivalent of [`Ord::cmp`].
	fn try_cmp(&self, other: &Self) -> Result<Ordering, Self::Error>;

	/// The fallible equivalent of [`Ord::max`].
	fn try_max(self, other: Self) -> Result<Self, Self::Error>
	where
		Self: Sized
	{
		match self.try_cmp(&other)? {
			Ordering::Less | Ordering::Equal => Ok(other),
			Ordering::Greater => Ok(self)
		}
	}

	/// The fallible equivalent of [`Ord::min`].
	fn try_min(self, other: Self) -> Result<Self, Self::Error>
	where
		Self: Sized
	{
		match self.try_cmp(&other)? {
			Ordering::Less | Ordering::Equal => Ok(self),
			Ordering::Greater => Ok(other)
		}
	}

	/// The fallible equivalent of [`Ord::clamp`].
	///
	/// # Panics
	///
	/// Panics if `min > max`
	fn try_clamp(self, min: Self, max: Self) -> Result<Self, Self::Error>
	where
		Self: Sized
	{
		if self.try_lt(&min)? {
			Ok(min)
		} else if self.try_gt(&max)? {
			Ok(max)
		} else {
			Ok(self)
		}
	}
}

impl<T: PartialEq<Rhs>, Rhs: ?Sized> TryPartialEq<Rhs> for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_eq(&self, other: &Rhs) -> Result<bool, Self::Error> {
		Ok(self == other)
	}

	#[inline]
	fn try_ne(&self, other: &Rhs) -> Result<bool, Self::Error> {
		Ok(self != other)
	}
}

impl<T: Eq> TryEq for T {}

impl<T: PartialOrd<Rhs>, Rhs: ?Sized> TryPartialOrd<Rhs> for T {
	#[inline]
	fn try_partial_cmp(&self, other: &Rhs) -> Result<Option<Ordering>, Self::Error> {
		Ok(self.partial_cmp(other))
	}

	#[inline]
	fn try_lt(&self, other: &Rhs) -> Result<bool, Self::Error> {
		Ok(self < other)
	}

	#[inline]
	fn try_le(&self, other: &Rhs) -> Result<bool, Self::Error> {
		Ok(self <= other)
	}

	#[inline]
	fn try_gt(&self, other: &Rhs) -> Result<bool, Self::Error> {
		Ok(self > other)
	}

	#[inline]
	fn try_ge(&self, other: &Rhs) -> Result<bool, Self::Error> {
		Ok(self >= other)
	}
}

impl<T: Ord> TryOrd for T {
	#[inline]
	fn try_cmp(&self, other: &Self) -> Result<Ordering, Self::Error> {
		Ok(self.cmp(other))
	}

	#[inline]
	fn try_max(self, other: Self) -> Result<Self, Self::Error>
	where
		Self: Sized
	{
		Ok(self.max(other))
	}

	#[inline]
	fn try_min(self, other: Self) -> Result<Self, Self::Error>
	where
		Self: Sized
	{
		Ok(self.min(other))
	}

	fn try_clamp(self, min: Self, max: Self) -> Result<Self, Self::Error>
	where
		Self: Sized
	{
		#[cfg(feature = "nightly_clamp")]
		{
			Ok(self.clamp(min, max))
		}
		#[cfg(not(feature = "nightly_clamp"))]
		{
			Ok(if self < min {
				min
			} else if self > max {
				max
			} else {
				self
			})
		}
	}
}
