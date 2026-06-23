//! The `Note` class: a worksheet cell note (the classic "comment" post-it).
//!
//! Wraps an owned `rust_xlsxwriter::Note`. Builder setters consume `self`
//! upstream, so we use the same clone-and-replace pattern as `Image` and return
//! `&Self` for JS chaining. Insert a note with `Worksheet.insertNote`.
//!
//! Omitted: `setObjectMovement` (needs the `ObjectMovement` enum) and the
//! mutating `resetText`; these can be added later if needed.

use rust_xlsxwriter as x;

use crate::format::Format;

#[napi]
pub struct Note {
    pub(crate) inner: x::Note,
}

macro_rules! mutate {
    ($self:ident, $method:ident $(, $arg:expr )* ) => {{
        let current = $self.inner.clone();
        $self.inner = current.$method($($arg),*);
        $self
    }};
}

#[napi]
impl Note {
    /// Create a note with the given text.
    #[napi(constructor)]
    pub fn new(text: String) -> Self {
        Self {
            inner: x::Note::new(text),
        }
    }

    /// Set the note author (shown as a prefix on the note text).
    #[napi]
    pub fn set_author(&mut self, name: String) -> &Self {
        mutate!(self, set_author, name)
    }

    /// Enable/disable the "Author:" prefix on the note text.
    #[napi]
    pub fn add_author_prefix(&mut self, enable: bool) -> &Self {
        mutate!(self, add_author_prefix, enable)
    }

    /// Set the note box width in pixels.
    #[napi]
    pub fn set_width(&mut self, width: u32) -> &Self {
        mutate!(self, set_width, width)
    }

    /// Set the note box height in pixels.
    #[napi]
    pub fn set_height(&mut self, height: u32) -> &Self {
        mutate!(self, set_height, height)
    }

    /// Show the note by default (instead of only on hover).
    #[napi]
    pub fn set_visible(&mut self, enable: bool) -> &Self {
        mutate!(self, set_visible, enable)
    }

    /// Set the note background color from a 24-bit RGB integer (e.g. `0xFFFFCC`).
    #[napi]
    pub fn set_background_color(&mut self, rgb: u32) -> &Self {
        mutate!(self, set_background_color, rgb)
    }

    /// Set the note font name.
    #[napi]
    pub fn set_font_name(&mut self, font_name: String) -> &Self {
        mutate!(self, set_font_name, font_name)
    }

    /// Set the note font size in points.
    #[napi]
    pub fn set_font_size(&mut self, font_size: f64) -> &Self {
        mutate!(self, set_font_size, font_size)
    }

    /// Apply a `Format` to the note.
    #[napi]
    pub fn set_format(&mut self, format: &Format) -> &Self {
        let fmt = format.inner.clone();
        mutate!(self, set_format, fmt)
    }

    /// Set alternative text for accessibility.
    #[napi]
    pub fn set_alt_text(&mut self, alt_text: String) -> &Self {
        mutate!(self, set_alt_text, alt_text)
    }
}
