//! Try traits for [`core::any`]
use core::any::{Any, TypeId};

/// The try trait for [`Any`](core::any::Any).
pub trait TryAny : 'static {
	/// The type returned in the event of an error.
	type Error;

	/// The fallible equivalent of [`Any::type_id`](core::any::Any::type_id)
	fn try_type_id(&self) -> Result<TypeId, Self::Error>;
}

impl<T: Any> TryAny for T {
	type Error = crate::Infallible;

	#[inline]
	fn try_type_id(&self) -> Result<TypeId, Self::Error> {
		Ok(self.type_id())
	}
}
