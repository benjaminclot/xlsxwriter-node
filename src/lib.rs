#![deny(clippy::all)]

//! Node.js bindings for the `rust_xlsxwriter` crate, generated with napi-rs.
//!
//! The JavaScript API mirrors the Rust API: `Workbook` -> `Worksheet` -> cell
//! writes, with `Format` objects for styling. Because `rust_xlsxwriter` hands out
//! worksheets as `&mut` references borrowed from the workbook (which cannot cross
//! the FFI boundary into a GC-owned JS object), the workbook is held in an
//! `Rc<RefCell<..>>` and each JS `Worksheet` is a lightweight handle holding a
//! clone of that `Rc` plus the sheet index. This is safe because Node executes
//! these calls on a single thread.

#[macro_use]
extern crate napi_derive;

mod chart;
mod conditional_format;
mod data_validation;
mod enums;
mod error;
mod format;
mod image;
mod note;
mod properties;
mod table;
mod workbook;
mod worksheet;
