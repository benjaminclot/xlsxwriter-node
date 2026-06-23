//! The `Image` class: wraps `rust_xlsxwriter::Image` for insertion into a sheet.
//!
//! Created from a file path or an in-memory `Buffer` (e.g. PNG/JPEG/GIF bytes).
//! Builder setters follow the same in-place `mem::replace` pattern as `Format`
//! and return `&Self` for chaining.

use napi::bindgen_prelude::{Buffer, Result};
use rust_xlsxwriter as x;

use crate::error::XlsxResultExt;

#[napi]
pub struct Image {
    pub(crate) inner: x::Image,
}

/// Apply a consuming builder method to `self.inner` in place. Unlike `Format`,
/// `Image` has no cheap placeholder, so we rebuild from the existing scale-less
/// state via `Clone` (Image is `Clone`).
macro_rules! mutate {
    ($self:ident, $method:ident $(, $arg:expr )* ) => {{
        let current = $self.inner.clone();
        $self.inner = current.$method($($arg),*);
        $self
    }};
}

#[napi]
impl Image {
    /// Load an image from a file path.
    #[napi(factory)]
    pub fn from_path(path: String) -> Result<Self> {
        Ok(Self {
            inner: x::Image::new(path).js()?,
        })
    }

    /// Load an image from an in-memory buffer (PNG, JPEG, GIF, BMP).
    #[napi(factory)]
    pub fn from_buffer(buffer: Buffer) -> Result<Self> {
        Ok(Self {
            inner: x::Image::new_from_buffer(&buffer).js()?,
        })
    }

    /// Scale the image width by a factor (1.0 = original size).
    #[napi]
    pub fn set_scale_width(&mut self, scale: f64) -> &Self {
        mutate!(self, set_scale_width, scale)
    }

    /// Scale the image height by a factor (1.0 = original size).
    #[napi]
    pub fn set_scale_height(&mut self, scale: f64) -> &Self {
        mutate!(self, set_scale_height, scale)
    }

    /// Set alternative text for accessibility.
    #[napi]
    pub fn set_alt_text(&mut self, alt_text: String) -> &Self {
        mutate!(self, set_alt_text, alt_text)
    }
}
