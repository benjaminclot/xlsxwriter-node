//! The `Worksheet` class: a lightweight handle (shared workbook `Rc` + sheet
//! index) exposing cell writes and layout methods. Every call re-borrows the
//! worksheet from the workbook by index, so no borrowed reference escapes to JS.

use std::cell::RefCell;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use napi::bindgen_prelude::{Either4, Result};
use rust_xlsxwriter as x;

use crate::chart::Chart;
use crate::conditional_format::{
    ConditionalFormat2ColorScale, ConditionalFormat3ColorScale, ConditionalFormatCell,
    ConditionalFormatDataBar, ConditionalFormatFormula,
};
use crate::data_validation::DataValidation;
use crate::error::to_napi;
use crate::format::Format;
use crate::image::Image;
use crate::note::Note;
use crate::table::Table;

/// A JS `Date` arrives as a UTC `DateTime`; `rust_xlsxwriter` wants a naive
/// datetime, so we drop the (UTC) zone with `naive_utc()`.
type JsDate = DateTime<Utc>;

/// A cell value accepted by the generic `write`: number, boolean, string or Date.
type CellValue = Either4<f64, bool, String, JsDate>;

#[napi]
pub struct Worksheet {
    pub(crate) wb: Rc<RefCell<x::Workbook>>,
    pub(crate) index: usize,
}

impl Worksheet {
    /// Borrow the underlying worksheet by index and run `f` against it.
    fn with<R>(
        &self,
        f: impl FnOnce(&mut x::Worksheet) -> std::result::Result<R, x::XlsxError>,
    ) -> Result<R> {
        let mut wb = self.wb.borrow_mut();
        let sheet = wb.worksheet_from_index(self.index).map_err(to_napi)?;
        f(sheet).map_err(to_napi)
    }
}

#[napi]
impl Worksheet {
    /// Write a value, dispatching on its JS type (number | boolean | string | Date).
    #[napi(
        ts_args_type = "row: number, col: number, value: number | boolean | string | Date"
    )]
    pub fn write(&self, row: u32, col: u16, value: CellValue) -> Result<()> {
        match value {
            Either4::A(n) => self.with(|s| s.write_number(row, col, n).map(|_| ())),
            Either4::B(b) => self.with(|s| s.write_boolean(row, col, b).map(|_| ())),
            Either4::C(s) => self.with(|w| w.write_string(row, col, &s).map(|_| ())),
            Either4::D(d) => self.with(|s| s.write_datetime(row, col, d.naive_utc()).map(|_| ())),
        }
    }

    /// Write a value with a `Format` applied.
    #[napi(
        ts_args_type = "row: number, col: number, value: number | boolean | string | Date, format: Format"
    )]
    pub fn write_with_format(
        &self,
        row: u32,
        col: u16,
        value: CellValue,
        format: &Format,
    ) -> Result<()> {
        let fmt = format.inner.clone();
        match value {
            Either4::A(n) => self.with(|s| s.write_with_format(row, col, n, &fmt).map(|_| ())),
            Either4::B(b) => self.with(|s| s.write_with_format(row, col, b, &fmt).map(|_| ())),
            Either4::C(s) => self.with(|w| w.write_with_format(row, col, &s, &fmt).map(|_| ())),
            Either4::D(d) => {
                self.with(|s| s.write_with_format(row, col, &d.naive_utc(), &fmt).map(|_| ()))
            }
        }
    }

    #[napi]
    pub fn write_string(&self, row: u32, col: u16, value: String) -> Result<()> {
        self.with(|s| s.write_string(row, col, &value).map(|_| ()))
    }

    #[napi]
    pub fn write_number(&self, row: u32, col: u16, value: f64) -> Result<()> {
        self.with(|s| s.write_number(row, col, value).map(|_| ()))
    }

    #[napi]
    pub fn write_boolean(&self, row: u32, col: u16, value: bool) -> Result<()> {
        self.with(|s| s.write_boolean(row, col, value).map(|_| ()))
    }

    /// Write a formula, e.g. `"=SUM(A1:A10)"`.
    #[napi]
    pub fn write_formula(&self, row: u32, col: u16, formula: String) -> Result<()> {
        self.with(|s| s.write_formula(row, col, formula.as_str()).map(|_| ()))
    }

    #[napi]
    pub fn write_datetime(&self, row: u32, col: u16, datetime: JsDate) -> Result<()> {
        self.with(|s| s.write_datetime(row, col, datetime.naive_utc()).map(|_| ()))
    }

    /// Write a URL hyperlink.
    #[napi]
    pub fn write_url(&self, row: u32, col: u16, url: String) -> Result<()> {
        self.with(|s| s.write_url(row, col, url.as_str()).map(|_| ()))
    }

    #[napi]
    pub fn set_name(&self, name: String) -> Result<()> {
        self.with(|s| s.set_name(name).map(|_| ()))
    }

    /// Column width in Excel character units.
    #[napi]
    pub fn set_column_width(&self, col: u16, width: f64) -> Result<()> {
        self.with(|s| s.set_column_width(col, width).map(|_| ()))
    }

    /// Row height in points.
    #[napi]
    pub fn set_row_height(&self, row: u32, height: f64) -> Result<()> {
        self.with(|s| s.set_row_height(row, height).map(|_| ()))
    }

    /// Apply a default format to a whole column.
    #[napi]
    pub fn set_column_format(&self, col: u16, format: &Format) -> Result<()> {
        let fmt = format.inner.clone();
        self.with(|s| s.set_column_format(col, &fmt).map(|_| ()))
    }

    /// Apply a default format to a whole row.
    #[napi]
    pub fn set_row_format(&self, row: u32, format: &Format) -> Result<()> {
        let fmt = format.inner.clone();
        self.with(|s| s.set_row_format(row, &fmt).map(|_| ()))
    }

    /// Merge a cell range and write a (formatted) string into it.
    #[napi]
    pub fn merge_range(
        &self,
        first_row: u32,
        first_col: u16,
        last_row: u32,
        last_col: u16,
        value: String,
        format: &Format,
    ) -> Result<()> {
        let fmt = format.inner.clone();
        self.with(|s| {
            s.merge_range(first_row, first_col, last_row, last_col, &value, &fmt)
                .map(|_| ())
        })
    }

    /// Freeze panes below/right of the given cell.
    #[napi]
    pub fn set_freeze_panes(&self, row: u32, col: u16) -> Result<()> {
        self.with(|s| s.set_freeze_panes(row, col).map(|_| ()))
    }

    /// Insert an image with its top-left corner anchored at the given cell.
    #[napi]
    pub fn insert_image(&self, row: u32, col: u16, image: &Image) -> Result<()> {
        self.with(|s| s.insert_image(row, col, &image.inner).map(|_| ()))
    }

    /// Insert a chart with its top-left corner anchored at the given cell.
    #[napi]
    pub fn insert_chart(&self, row: u32, col: u16, chart: &Chart) -> Result<()> {
        self.with(|s| s.insert_chart(row, col, &chart.inner).map(|_| ()))
    }

    /// Add a table over the given cell range (inclusive of header and totals rows).
    #[napi]
    pub fn add_table(
        &self,
        first_row: u32,
        first_col: u16,
        last_row: u32,
        last_col: u16,
        table: &Table,
    ) -> Result<()> {
        self.with(|s| {
            s.add_table(first_row, first_col, last_row, last_col, &table.inner)
                .map(|_| ())
        })
    }

    /// Apply a cell-rule conditional format to a range.
    #[napi]
    pub fn add_conditional_format_cell(
        &self,
        first_row: u32,
        first_col: u16,
        last_row: u32,
        last_col: u16,
        conditional_format: &ConditionalFormatCell,
    ) -> Result<()> {
        self.with(|s| {
            s.add_conditional_format(
                first_row,
                first_col,
                last_row,
                last_col,
                &conditional_format.inner,
            )
            .map(|_| ())
        })
    }

    /// Apply a formula-based conditional format to a range.
    #[napi]
    pub fn add_conditional_format_formula(
        &self,
        first_row: u32,
        first_col: u16,
        last_row: u32,
        last_col: u16,
        conditional_format: &ConditionalFormatFormula,
    ) -> Result<()> {
        self.with(|s| {
            s.add_conditional_format(
                first_row,
                first_col,
                last_row,
                last_col,
                &conditional_format.inner,
            )
            .map(|_| ())
        })
    }

    /// Apply a 2-color scale conditional format to a range.
    #[napi]
    pub fn add_conditional_format2_color_scale(
        &self,
        first_row: u32,
        first_col: u16,
        last_row: u32,
        last_col: u16,
        conditional_format: &ConditionalFormat2ColorScale,
    ) -> Result<()> {
        self.with(|s| {
            s.add_conditional_format(
                first_row,
                first_col,
                last_row,
                last_col,
                &conditional_format.inner,
            )
            .map(|_| ())
        })
    }

    /// Apply a 3-color scale conditional format to a range.
    #[napi]
    pub fn add_conditional_format3_color_scale(
        &self,
        first_row: u32,
        first_col: u16,
        last_row: u32,
        last_col: u16,
        conditional_format: &ConditionalFormat3ColorScale,
    ) -> Result<()> {
        self.with(|s| {
            s.add_conditional_format(
                first_row,
                first_col,
                last_row,
                last_col,
                &conditional_format.inner,
            )
            .map(|_| ())
        })
    }

    /// Apply a data-bar conditional format to a range.
    #[napi]
    pub fn add_conditional_format_data_bar(
        &self,
        first_row: u32,
        first_col: u16,
        last_row: u32,
        last_col: u16,
        conditional_format: &ConditionalFormatDataBar,
    ) -> Result<()> {
        self.with(|s| {
            s.add_conditional_format(
                first_row,
                first_col,
                last_row,
                last_col,
                &conditional_format.inner,
            )
            .map(|_| ())
        })
    }

    /// Apply a data validation to a cell range.
    #[napi]
    pub fn add_data_validation(
        &self,
        first_row: u32,
        first_col: u16,
        last_row: u32,
        last_col: u16,
        data_validation: &DataValidation,
    ) -> Result<()> {
        self.with(|s| {
            s.add_data_validation(
                first_row,
                first_col,
                last_row,
                last_col,
                &data_validation.inner,
            )
            .map(|_| ())
        })
    }

    // --- Cell notes ---------------------------------------------------------

    /// Insert a cell note (comment) anchored at the given cell.
    #[napi]
    pub fn insert_note(&self, row: u32, col: u16, note: &Note) -> Result<()> {
        self.with(|s| s.insert_note(row, col, &note.inner).map(|_| ()))
    }

    // --- Autofilter ---------------------------------------------------------

    /// Add an autofilter over the given cell range. Per-column filter conditions
    /// are not exposed yet (the upstream `FilterCondition` API is omitted).
    #[napi]
    pub fn autofilter(
        &self,
        first_row: u32,
        first_col: u16,
        last_row: u32,
        last_col: u16,
    ) -> Result<()> {
        self.with(|s| {
            s.autofilter(first_row, first_col, last_row, last_col)
                .map(|_| ())
        })
    }

    // --- Page setup & print options ----------------------------------------

    /// Set the page orientation to landscape.
    #[napi]
    pub fn set_landscape(&self) -> Result<()> {
        self.with(|s| {
            s.set_landscape();
            Ok(())
        })
    }

    /// Set the page orientation to portrait (the default).
    #[napi]
    pub fn set_portrait(&self) -> Result<()> {
        self.with(|s| {
            s.set_portrait();
            Ok(())
        })
    }

    /// Set the paper size using an Excel paper-size index (e.g. 1 = Letter, 9 = A4).
    #[napi]
    pub fn set_paper_size(&self, paper_size: u8) -> Result<()> {
        self.with(|s| {
            s.set_paper_size(paper_size);
            Ok(())
        })
    }

    /// Set the worksheet zoom level (10-400, as a percentage).
    #[napi]
    pub fn set_zoom(&self, zoom: u16) -> Result<()> {
        self.with(|s| {
            s.set_zoom(zoom);
            Ok(())
        })
    }

    /// Set all page margins in inches (header/footer margins included).
    #[napi]
    pub fn set_margins(
        &self,
        left: f64,
        right: f64,
        top: f64,
        bottom: f64,
        header: f64,
        footer: f64,
    ) -> Result<()> {
        self.with(|s| {
            s.set_margins(left, right, top, bottom, header, footer);
            Ok(())
        })
    }

    /// Set the page header string (supports Excel header/footer formatting codes).
    #[napi]
    pub fn set_header(&self, header: String) -> Result<()> {
        self.with(|s| {
            s.set_header(header);
            Ok(())
        })
    }

    /// Set the page footer string (supports Excel header/footer formatting codes).
    #[napi]
    pub fn set_footer(&self, footer: String) -> Result<()> {
        self.with(|s| {
            s.set_footer(footer);
            Ok(())
        })
    }

    /// Show/hide gridlines when printing.
    #[napi]
    pub fn set_print_gridlines(&self, enable: bool) -> Result<()> {
        self.with(|s| {
            s.set_print_gridlines(enable);
            Ok(())
        })
    }

    /// Show/hide gridlines on screen.
    #[napi]
    pub fn set_screen_gridlines(&self, enable: bool) -> Result<()> {
        self.with(|s| {
            s.set_screen_gridlines(enable);
            Ok(())
        })
    }

    /// Fit the printed output to the given number of pages wide and tall.
    #[napi]
    pub fn set_print_fit_to_pages(&self, width: u16, height: u16) -> Result<()> {
        self.with(|s| {
            s.set_print_fit_to_pages(width, height);
            Ok(())
        })
    }

    /// Set the print area to the given cell range.
    #[napi]
    pub fn set_print_area(
        &self,
        first_row: u32,
        first_col: u16,
        last_row: u32,
        last_col: u16,
    ) -> Result<()> {
        self.with(|s| {
            s.set_print_area(first_row, first_col, last_row, last_col)
                .map(|_| ())
        })
    }

    /// Repeat the given rows at the top of each printed page.
    #[napi]
    pub fn set_repeat_rows(&self, first_row: u32, last_row: u32) -> Result<()> {
        self.with(|s| s.set_repeat_rows(first_row, last_row).map(|_| ()))
    }

    /// Repeat the given columns at the left of each printed page.
    #[napi]
    pub fn set_repeat_columns(&self, first_col: u16, last_col: u16) -> Result<()> {
        self.with(|s| s.set_repeat_columns(first_col, last_col).map(|_| ()))
    }

    /// Set the worksheet tab color from a 24-bit RGB integer.
    #[napi]
    pub fn set_tab_color(&self, rgb: u32) -> Result<()> {
        self.with(|s| {
            s.set_tab_color(rgb);
            Ok(())
        })
    }

    /// Display the worksheet right-to-left.
    #[napi]
    pub fn set_right_to_left(&self, enable: bool) -> Result<()> {
        self.with(|s| {
            s.set_right_to_left(enable);
            Ok(())
        })
    }

    /// Hide the worksheet.
    #[napi]
    pub fn set_hidden(&self, enable: bool) -> Result<()> {
        self.with(|s| {
            s.set_hidden(enable);
            Ok(())
        })
    }
}
