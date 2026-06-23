//! The `DocProperties` class: workbook document metadata (title, author, etc.).
//!
//! Wraps an owned `rust_xlsxwriter::DocProperties`. Apply it to a workbook with
//! `Workbook.setProperties`. Builder setters consume `self` upstream, so we use
//! the clone-and-replace pattern and return `&Self` for chaining.
//!
//! Omitted: `setCreationDatetime` (needs a datetime operand); only string-valued
//! custom properties are supported via `setCustomProperty`.

use rust_xlsxwriter as x;

#[napi]
#[derive(Clone)]
pub struct DocProperties {
    pub(crate) inner: x::DocProperties,
}

macro_rules! mutate {
    ($self:ident, $method:ident $(, $arg:expr )* ) => {{
        let current = $self.inner.clone();
        $self.inner = current.$method($($arg),*);
        $self
    }};
}

#[napi]
impl DocProperties {
    #[napi(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: x::DocProperties::new(),
        }
    }

    #[napi]
    pub fn set_title(&mut self, title: String) -> &Self {
        mutate!(self, set_title, title)
    }

    #[napi]
    pub fn set_subject(&mut self, subject: String) -> &Self {
        mutate!(self, set_subject, subject)
    }

    #[napi]
    pub fn set_author(&mut self, author: String) -> &Self {
        mutate!(self, set_author, author)
    }

    #[napi]
    pub fn set_manager(&mut self, manager: String) -> &Self {
        mutate!(self, set_manager, manager)
    }

    #[napi]
    pub fn set_company(&mut self, company: String) -> &Self {
        mutate!(self, set_company, company)
    }

    #[napi]
    pub fn set_category(&mut self, category: String) -> &Self {
        mutate!(self, set_category, category)
    }

    #[napi]
    pub fn set_keywords(&mut self, keywords: String) -> &Self {
        mutate!(self, set_keywords, keywords)
    }

    #[napi]
    pub fn set_comment(&mut self, comment: String) -> &Self {
        mutate!(self, set_comment, comment)
    }

    #[napi]
    pub fn set_status(&mut self, status: String) -> &Self {
        mutate!(self, set_status, status)
    }

    #[napi]
    pub fn set_hyperlink_base(&mut self, hyperlink_base: String) -> &Self {
        mutate!(self, set_hyperlink_base, hyperlink_base)
    }

    /// Set a string-valued custom document property.
    #[napi]
    pub fn set_custom_property(&mut self, name: String, value: String) -> &Self {
        mutate!(self, set_custom_property, name, value.as_str())
    }
}
