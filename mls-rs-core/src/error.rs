// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// Copyright by contributors to this project.
// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use core::fmt::{self, Display};

#[cfg(feature = "std")]
#[derive(Debug)]
/// Generic error used to wrap errors produced by providers.
pub struct AnyError(Box<dyn std::error::Error + Send + Sync>);

#[cfg(not(feature = "std"))]
#[derive(Debug)]
pub struct AnyError;

#[cfg(feature = "std")]
impl Display for AnyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AnyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

#[cfg(feature = "std")]
impl AnyError {
    pub fn inner_dyn_error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
        &*self.0
    }
}

#[cfg(not(feature = "std"))]
impl Display for AnyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

/// Trait to convert a provider specific error into [`AnyError`]
pub trait IntoAnyError: core::fmt::Debug + Sized {
    #[cfg(feature = "std")]
    fn into_any_error(self) -> AnyError {
        self.into_dyn_error()
            .map_or_else(|this| AnyError(format!("{this:?}").into()), AnyError)
    }

    #[cfg(not(feature = "std"))]
    fn into_any_error(self) -> AnyError {
        AnyError
    }

    #[cfg(feature = "std")]
    fn into_dyn_error(self) -> Result<Box<dyn std::error::Error + Send + Sync>, Self> {
        Err(self)
    }
}

impl IntoAnyError for mls_rs_codec::Error {
    #[cfg(feature = "std")]
    fn into_dyn_error(self) -> Result<Box<dyn std::error::Error + Send + Sync>, Self> {
        Ok(self.into())
    }
}

impl IntoAnyError for core::convert::Infallible {}

#[cfg(test)]
mod tests {
    use super::{AnyError, IntoAnyError};
    use mls_rs_codec::Error as CodecError;

    #[test]
    fn inner_dyn_error_returns_wrapped_error_with_data() {
        let error_with_data = CodecError::Custom(42);
        let any_error: AnyError = error_with_data.into_any_error();

        let downcast_err = any_error
            .inner_dyn_error()
            .downcast_ref::<CodecError>()
            .expect("Expected the dyn error to be of type CodecError");

        assert!(matches!(downcast_err, CodecError::Custom(42)));
    }
}
