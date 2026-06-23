//! The `Format` class: a builder for cell formatting.
//!
//! `rust_xlsxwriter`'s `Format` setters consume `self` and return a new `Format`.
//! `Format` is not `Default`, so in the binding we swap the inner value out with
//! `mem::replace` (using `Format::new()` as a placeholder), apply the setter, and
//! store the result back. Each setter returns `&Self` so JS can chain calls:
//! `new Format().setBold().setFontColor(0xFF0000)`.

use rust_xlsxwriter as x;

use crate::enums::{FormatAlign, FormatBorder, FormatPattern, FormatUnderline};

#[napi]
#[derive(Clone)]
pub struct Format {
    pub(crate) inner: x::Format,
}

/// Apply a consuming builder method to `self.inner` in place.
macro_rules! mutate {
    ($self:ident, $method:ident $(, $arg:expr )* ) => {{
        let current = std::mem::replace(&mut $self.inner, x::Format::new());
        $self.inner = current.$method($($arg),*);
        $self
    }};
}

#[napi]
impl Format {
    #[napi(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: x::Format::new(),
        }
    }

    #[napi]
    pub fn set_bold(&mut self) -> &Self {
        mutate!(self, set_bold)
    }

    #[napi]
    pub fn set_italic(&mut self) -> &Self {
        mutate!(self, set_italic)
    }

    #[napi]
    pub fn set_font_strikethrough(&mut self) -> &Self {
        mutate!(self, set_font_strikethrough)
    }

    #[napi]
    pub fn set_text_wrap(&mut self) -> &Self {
        mutate!(self, set_text_wrap)
    }

    #[napi]
    pub fn set_font_name(&mut self, name: String) -> &Self {
        mutate!(self, set_font_name, name)
    }

    #[napi]
    pub fn set_font_size(&mut self, size: f64) -> &Self {
        mutate!(self, set_font_size, size)
    }

    /// Set the font color from a 24-bit RGB integer, e.g. `0xFF0000` for red.
    #[napi]
    pub fn set_font_color(&mut self, rgb: u32) -> &Self {
        mutate!(self, set_font_color, rgb)
    }

    #[napi]
    pub fn set_underline(&mut self, underline: FormatUnderline) -> &Self {
        mutate!(self, set_underline, underline.into())
    }

    /// Set the Excel number format string, e.g. `"0.000"` or `"#,##0.00"`.
    #[napi]
    pub fn set_num_format(&mut self, num_format: String) -> &Self {
        mutate!(self, set_num_format, num_format)
    }

    #[napi]
    pub fn set_align(&mut self, align: FormatAlign) -> &Self {
        mutate!(self, set_align, align.into())
    }

    #[napi]
    pub fn set_indent(&mut self, indent: u8) -> &Self {
        mutate!(self, set_indent, indent)
    }

    /// Text rotation in degrees, -90 to 90 (or 270 for vertical text).
    #[napi]
    pub fn set_rotation(&mut self, rotation: i16) -> &Self {
        mutate!(self, set_rotation, rotation)
    }

    /// Background (fill) color from a 24-bit RGB integer. Needs a pattern; for a
    /// plain fill, set the pattern to `Solid`.
    #[napi]
    pub fn set_background_color(&mut self, rgb: u32) -> &Self {
        mutate!(self, set_background_color, rgb)
    }

    #[napi]
    pub fn set_foreground_color(&mut self, rgb: u32) -> &Self {
        mutate!(self, set_foreground_color, rgb)
    }

    #[napi]
    pub fn set_pattern(&mut self, pattern: FormatPattern) -> &Self {
        mutate!(self, set_pattern, pattern.into())
    }

    /// Set all four cell borders to the same style.
    #[napi]
    pub fn set_border(&mut self, border: FormatBorder) -> &Self {
        mutate!(self, set_border, border.into())
    }
}
