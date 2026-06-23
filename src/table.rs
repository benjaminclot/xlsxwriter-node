//! The `Table` class: wraps `rust_xlsxwriter::Table` (an Excel worksheet table
//! over a cell range with a header row, banding, and an optional totals row).
//!
//! `Table`'s setters consume `self` and return `Table` (and `Table` is `Clone`),
//! so the binding uses the same in-place clone-and-replace builder pattern as
//! `Image`, returning `&Self` for chaining. `TableColumn` is a nested builder, so
//! it is flattened into a `#[napi(object)]` options struct passed to `setColumns`.

use rust_xlsxwriter as x;

use crate::enums::{TableFunction, TableStyle};

/// Options describing a single table column.
#[napi(object)]
pub struct TableColumnOptions {
    /// Header caption for the column.
    pub header: Option<String>,
    /// Label to place in the totals row (e.g. "Total"). Mutually useful with
    /// `total_function` on other columns.
    pub total_label: Option<String>,
    /// Aggregation function for this column's totals-row cell.
    pub total_function: Option<TableFunction>,
}

#[napi]
#[derive(Clone)]
pub struct Table {
    pub(crate) inner: x::Table,
}

/// Apply a consuming builder method to `self.inner` in place (Table is Clone).
macro_rules! mutate {
    ($self:ident, $method:ident $(, $arg:expr )* ) => {{
        let current = $self.inner.clone();
        $self.inner = current.$method($($arg),*);
        $self
    }};
}

#[napi]
impl Table {
    #[napi(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: x::Table::new(),
        }
    }

    /// Define the table columns (headers and totals-row behaviour), in order.
    #[napi]
    pub fn set_columns(&mut self, columns: Vec<TableColumnOptions>) -> &Self {
        let cols: Vec<x::TableColumn> = columns
            .into_iter()
            .map(|opt| {
                let mut col = x::TableColumn::new();
                if let Some(header) = opt.header {
                    col = col.set_header(header);
                }
                if let Some(label) = opt.total_label {
                    col = col.set_total_label(label);
                }
                if let Some(function) = opt.total_function {
                    col = col.set_total_function(function.into());
                }
                col
            })
            .collect();
        mutate!(self, set_columns, &cols)
    }

    /// Set the table name (used in formulas and structured references).
    #[napi]
    pub fn set_name(&mut self, name: String) -> &Self {
        mutate!(self, set_name, name)
    }

    /// Apply a built-in table style.
    #[napi]
    pub fn set_style(&mut self, style: TableStyle) -> &Self {
        mutate!(self, set_style, style.into())
    }

    /// Turn the header row on or off (on by default).
    #[napi]
    pub fn set_header_row(&mut self, enable: bool) -> &Self {
        mutate!(self, set_header_row, enable)
    }

    /// Turn the totals row on or off.
    #[napi]
    pub fn set_total_row(&mut self, enable: bool) -> &Self {
        mutate!(self, set_total_row, enable)
    }

    /// Enable/disable banded (striped) rows.
    #[napi]
    pub fn set_banded_rows(&mut self, enable: bool) -> &Self {
        mutate!(self, set_banded_rows, enable)
    }

    /// Enable/disable banded (striped) columns.
    #[napi]
    pub fn set_banded_columns(&mut self, enable: bool) -> &Self {
        mutate!(self, set_banded_columns, enable)
    }

    /// Highlight the first column.
    #[napi]
    pub fn set_first_column(&mut self, enable: bool) -> &Self {
        mutate!(self, set_first_column, enable)
    }

    /// Highlight the last column.
    #[napi]
    pub fn set_last_column(&mut self, enable: bool) -> &Self {
        mutate!(self, set_last_column, enable)
    }

    /// Turn the header-row autofilter on or off.
    #[napi]
    pub fn set_autofilter(&mut self, enable: bool) -> &Self {
        mutate!(self, set_autofilter, enable)
    }

    /// Set alternative text for accessibility.
    #[napi]
    pub fn set_alt_text(&mut self, alt_text: String) -> &Self {
        mutate!(self, set_alt_text, alt_text)
    }
}
