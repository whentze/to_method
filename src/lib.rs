//! A utility micro-crate for using [`Into`](::core::convert::Into) and
//! [`TryInto`](::core::convert::TryInto) more ergonomically.
//!
//! It exposes a [`To`](crate::To) extension trait with a [`.to()`](To::to)
//! method and [`.try_to()`](To::try_to) method which you can use to invoke
//! [`Into::into`](::core::convert::Into::into) and
//! [`TryInto`](::core::convert::TryInto::try_into) while specifying the target
//! type and without having to abandon method-call syntax.
//!
//! Being a micro-crate, it tries to be as nice of a dependency as possible and has:
//!
//! - No dependencies of its own
//! - No feature flags
//! - No `build.rs`
//! - `#![no_std]`
//! - `#![forbid(unsafe_code)]`
//!
//! # Regular `Into` usage
//!
//! ```
//! let x : u8 = 5;
//!
//! // The type parameter is on `Into`, not on `Into::into`,
//! // so we need to do it like this:
//! let y = Into::<u16>::into(x);
//!
//! // Depending on context, inference can make this work though:
//! let z : u32 = y.into();
//! ```
//!
//! # With `To`
//!
//! ```
//! use to_method::To as _;
//!
//! let x : u8 = 5;
//!
//! // The type parameter is on the `to` method, so this works:
//! let y = x.to::<u16>();
//!
//! // And you can still rely on inference as well:
//! let z : u32 = y.to();
//! ```
//!
//! # `TryInto`
//!
//! The same mechanism works for the `TryInto` trait via the `try_to` method:
//!
//! ```
//! use core::convert::TryInto;
//!
//! # fn main() -> Result<(), core::num::TryFromIntError> {
//!
//! let x : u16 = 5;
//!
//! // The type parameter is on `Into`, not on `Into::into`,
//! // so we need to do it like this:
//! let y = TryInto::<u8>::try_into(x)?;
//!
//! // Depending on context, inference can make this work though:
//! let z : u8 = y.try_into()?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! compared to:
//!
//! ```
//! use core::convert::TryInto;
//!
//! # fn main() -> Result<(), core::num::TryFromIntError> {
//!
//! use to_method::To as _;
//!
//! let x : u16 = 5;
//!
//! // The type parameter is on the `to` method, so this works:
//! let y = x.try_to::<u8>()?;
//!
//! // And you can still rely on inference as well:
//! let z : u8 = y.try_to()?;
//!
//! # Ok(())
//! # }
//! ```

#![no_std]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

use core::convert::TryInto;

/// Extension trait providing the [`to`](To::to) and [`try_to`](To::try_to) methods.
pub trait To {
    /// Converts to `T` by calling `Into<T>::into`.
    #[inline(always)]
    fn to<T>(self) -> T
    where
        Self: Into<T>,
    {
        <Self as Into<T>>::into(self)
    }
    
    /// Tries to convert to `T` by calling `TryInto<T>::try_into`.
    fn try_to<T>(self) -> Result<T, <Self as TryInto<T>>::Error>
    where
        Self: TryInto<T>,
    {
        <Self as TryInto<T>>::try_into(self)
    }
}

/// Blanket impl for all types.
/// This makes sure that everything implements `To` and
/// that no downstream impls can exist.
impl<T: ?Sized> To for T {}
