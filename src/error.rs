//! Bridge `rust_xlsxwriter::XlsxError` into `napi::Error`.
//!
//! The orphan rule prevents `impl From<XlsxError> for napi::Error` (both are
//! foreign types), so we expose a small helper plus an extension trait that adds
//! a `.js()` combinator for ergonomic `?` propagation in the binding methods.

use napi::bindgen_prelude::{Error, Result, Status};
use rust_xlsxwriter::XlsxError;

/// Convert an `XlsxError` into a `napi::Error` that surfaces as a JS exception.
pub fn to_napi(err: XlsxError) -> Error {
    Error::new(Status::GenericFailure, err.to_string())
}

/// Extension trait to turn `Result<T, XlsxError>` into a napi `Result<T>`.
pub trait XlsxResultExt<T> {
    fn js(self) -> Result<T>;
}

impl<T> XlsxResultExt<T> for std::result::Result<T, XlsxError> {
    fn js(self) -> Result<T> {
        self.map_err(to_napi)
    }
}
