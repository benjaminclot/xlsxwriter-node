//! The `Workbook` class: the entry point that owns the underlying
//! `rust_xlsxwriter::Workbook` and produces `Worksheet` handles.

use std::cell::RefCell;
use std::rc::Rc;

use napi::bindgen_prelude::{Buffer, Result};
use rust_xlsxwriter as x;

use crate::error::XlsxResultExt;
use crate::properties::DocProperties;
use crate::worksheet::Worksheet;

#[napi]
pub struct Workbook {
    pub(crate) inner: Rc<RefCell<x::Workbook>>,
    /// Index that the next added worksheet will occupy.
    next_index: usize,
}

#[napi]
impl Workbook {
    #[napi(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(x::Workbook::new())),
            next_index: 0,
        }
    }

    /// Add a new worksheet, optionally with a name, and return a handle to it.
    #[napi]
    pub fn add_worksheet(&mut self, name: Option<String>) -> Result<Worksheet> {
        let index = self.next_index;
        {
            let mut wb = self.inner.borrow_mut();
            let sheet = wb.add_worksheet();
            if let Some(name) = name {
                sheet.set_name(name).js()?;
            }
        }
        self.next_index += 1;
        Ok(Worksheet {
            wb: self.inner.clone(),
            index,
        })
    }

    /// Set the workbook's document properties (title, author, etc.).
    #[napi]
    pub fn set_properties(&mut self, properties: &DocProperties) {
        self.inner.borrow_mut().set_properties(&properties.inner);
    }

    /// Save the workbook to a file path.
    #[napi]
    pub fn save(&mut self, path: String) -> Result<()> {
        self.inner.borrow_mut().save(path).js()
    }

    /// Save the workbook to an in-memory buffer (for streaming / HTTP responses).
    #[napi]
    pub fn save_to_buffer(&mut self) -> Result<Buffer> {
        let bytes = self.inner.borrow_mut().save_to_buffer().js()?;
        Ok(Buffer::from(bytes))
    }
}
