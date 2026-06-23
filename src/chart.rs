//! The `Chart` class.
//!
//! `rust_xlsxwriter`'s chart builders are deeply nested (`add_series()`,
//! `title()`, `x_axis()` … each return `&mut` sub-objects), which can't be handed
//! back across the FFI boundary. Instead this binding exposes a *flattened* API:
//! each method configures the chart in a single call (e.g. `addSeries` takes an
//! options object and wires up values/categories/name internally).

use rust_xlsxwriter as x;

use crate::enums::{ChartLegendPosition, ChartType};

/// Options for a single chart series. Ranges are Excel range strings such as
/// `"Sheet1!$A$1:$A$5"`. `name` may be a range or a literal label.
#[napi(object)]
pub struct ChartSeriesOptions {
    pub values: String,
    pub categories: Option<String>,
    pub name: Option<String>,
}

#[napi]
pub struct Chart {
    pub(crate) inner: x::Chart,
}

#[napi]
impl Chart {
    #[napi(constructor)]
    pub fn new(chart_type: ChartType) -> Self {
        Self {
            inner: x::Chart::new(chart_type.into()),
        }
    }

    /// Add a data series to the chart.
    #[napi]
    pub fn add_series(&mut self, options: ChartSeriesOptions) -> &Self {
        let series = self.inner.add_series();
        series.set_values(options.values.as_str());
        if let Some(categories) = &options.categories {
            series.set_categories(categories.as_str());
        }
        if let Some(name) = &options.name {
            series.set_name(name.as_str());
        }
        self
    }

    /// Set the chart title.
    #[napi]
    pub fn set_title(&mut self, title: String) -> &Self {
        self.inner.title().set_name(title.as_str());
        self
    }

    /// Set the X-axis title.
    #[napi]
    pub fn set_x_axis_name(&mut self, name: String) -> &Self {
        self.inner.x_axis().set_name(name.as_str());
        self
    }

    /// Set the Y-axis title.
    #[napi]
    pub fn set_y_axis_name(&mut self, name: String) -> &Self {
        self.inner.y_axis().set_name(name.as_str());
        self
    }

    /// Position the legend, or hide it.
    #[napi]
    pub fn set_legend_position(&mut self, position: ChartLegendPosition) -> &Self {
        self.inner.legend().set_position(position.into());
        self
    }

    /// Hide the chart legend.
    #[napi]
    pub fn set_legend_hidden(&mut self) -> &Self {
        self.inner.legend().set_hidden();
        self
    }

    /// Apply a built-in chart style (1-48).
    #[napi]
    pub fn set_style(&mut self, style: u8) -> &Self {
        self.inner.set_style(style);
        self
    }
}
