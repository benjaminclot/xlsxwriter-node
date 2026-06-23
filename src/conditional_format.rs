//! Conditional formatting classes.
//!
//! Each concrete `rust_xlsxwriter` conditional-format type is wrapped in its own
//! `#[napi]` class holding the owned value. Setters consume `self` upstream and
//! all these types are `Clone`, so we use the clone-and-replace builder pattern
//! and return `&Self` for JS chaining. They are applied to a sheet range via the
//! per-type `addConditionalFormat*` methods on `Worksheet`.
//!
//! Supported: cell rules (numeric comparisons), formula, 2-color scale,
//! 3-color scale, and data bar.
//!
//! Omitted for now (can be added later): blank/error/duplicate/average/top/text/
//! date conditional types; icon sets; the generic `set_minimum`/`set_maximum`
//! type+value anchors on color scales and data bars; data bar direction/axis
//! position. Cell rules currently use numeric (`f64`) operands only.

use napi::bindgen_prelude::Result;
use rust_xlsxwriter as x;

use crate::enums::ConditionalFormatCellRuleType;
use crate::format::Format;

/// Apply a consuming builder method to a `Clone` inner value in place.
macro_rules! mutate {
    ($self:ident, $method:ident $(, $arg:expr )* ) => {{
        let current = $self.inner.clone();
        $self.inner = current.$method($($arg),*);
        $self
    }};
}

// ---------------------------------------------------------------------------
// ConditionalFormatCell
// ---------------------------------------------------------------------------

#[napi]
pub struct ConditionalFormatCell {
    pub(crate) inner: x::ConditionalFormatCell,
}

#[napi]
impl ConditionalFormatCell {
    #[napi(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: x::ConditionalFormatCell::new(),
        }
    }

    /// Set the comparison rule. `value2` is required for `Between`/`NotBetween`
    /// and ignored otherwise.
    #[napi]
    pub fn set_rule(
        &mut self,
        rule: ConditionalFormatCellRuleType,
        value1: f64,
        value2: Option<f64>,
    ) -> Result<&Self> {
        use x::ConditionalFormatCellRule as R;
        let needs_two = matches!(
            rule,
            ConditionalFormatCellRuleType::Between | ConditionalFormatCellRuleType::NotBetween
        );
        let v2 = if needs_two {
            value2.ok_or_else(|| {
                napi::Error::from_reason(
                    "Between/NotBetween rules require a second value".to_owned(),
                )
            })?
        } else {
            0.0
        };
        let cell_rule = match rule {
            ConditionalFormatCellRuleType::EqualTo => R::EqualTo(value1),
            ConditionalFormatCellRuleType::NotEqualTo => R::NotEqualTo(value1),
            ConditionalFormatCellRuleType::GreaterThan => R::GreaterThan(value1),
            ConditionalFormatCellRuleType::GreaterThanOrEqualTo => R::GreaterThanOrEqualTo(value1),
            ConditionalFormatCellRuleType::LessThan => R::LessThan(value1),
            ConditionalFormatCellRuleType::LessThanOrEqualTo => R::LessThanOrEqualTo(value1),
            ConditionalFormatCellRuleType::Between => R::Between(value1, v2),
            ConditionalFormatCellRuleType::NotBetween => R::NotBetween(value1, v2),
        };
        let current = self.inner.clone();
        self.inner = current.set_rule(cell_rule);
        Ok(self)
    }

    /// Format applied to cells that match the rule.
    #[napi]
    pub fn set_format(&mut self, format: &Format) -> &Self {
        mutate!(self, set_format, format.inner.clone())
    }

    /// Apply to additional (non-contiguous) ranges, e.g. `"A1:A10 C1:C10"`.
    #[napi]
    pub fn set_multi_range(&mut self, range: String) -> &Self {
        mutate!(self, set_multi_range, range)
    }

    /// Stop evaluating later rules if this one matches.
    #[napi]
    pub fn set_stop_if_true(&mut self, enable: bool) -> &Self {
        mutate!(self, set_stop_if_true, enable)
    }
}

// ---------------------------------------------------------------------------
// ConditionalFormatFormula
// ---------------------------------------------------------------------------

#[napi]
pub struct ConditionalFormatFormula {
    pub(crate) inner: x::ConditionalFormatFormula,
}

#[napi]
impl ConditionalFormatFormula {
    #[napi(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: x::ConditionalFormatFormula::new(),
        }
    }

    /// Set the formula, e.g. `"=$A1>50"`.
    #[napi]
    pub fn set_rule(&mut self, formula: String) -> &Self {
        mutate!(self, set_rule, formula.as_str())
    }

    #[napi]
    pub fn set_format(&mut self, format: &Format) -> &Self {
        mutate!(self, set_format, format.inner.clone())
    }

    #[napi]
    pub fn set_multi_range(&mut self, range: String) -> &Self {
        mutate!(self, set_multi_range, range)
    }

    #[napi]
    pub fn set_stop_if_true(&mut self, enable: bool) -> &Self {
        mutate!(self, set_stop_if_true, enable)
    }
}

// ---------------------------------------------------------------------------
// ConditionalFormat2ColorScale
// ---------------------------------------------------------------------------

#[napi]
pub struct ConditionalFormat2ColorScale {
    pub(crate) inner: x::ConditionalFormat2ColorScale,
}

#[napi]
impl ConditionalFormat2ColorScale {
    #[napi(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: x::ConditionalFormat2ColorScale::new(),
        }
    }

    /// Color (24-bit RGB) for the minimum value.
    #[napi]
    pub fn set_minimum_color(&mut self, rgb: u32) -> &Self {
        mutate!(self, set_minimum_color, rgb)
    }

    /// Color (24-bit RGB) for the maximum value.
    #[napi]
    pub fn set_maximum_color(&mut self, rgb: u32) -> &Self {
        mutate!(self, set_maximum_color, rgb)
    }

    #[napi]
    pub fn set_multi_range(&mut self, range: String) -> &Self {
        mutate!(self, set_multi_range, range)
    }
}

// ---------------------------------------------------------------------------
// ConditionalFormat3ColorScale
// ---------------------------------------------------------------------------

#[napi]
pub struct ConditionalFormat3ColorScale {
    pub(crate) inner: x::ConditionalFormat3ColorScale,
}

#[napi]
impl ConditionalFormat3ColorScale {
    #[napi(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: x::ConditionalFormat3ColorScale::new(),
        }
    }

    #[napi]
    pub fn set_minimum_color(&mut self, rgb: u32) -> &Self {
        mutate!(self, set_minimum_color, rgb)
    }

    #[napi]
    pub fn set_midpoint_color(&mut self, rgb: u32) -> &Self {
        mutate!(self, set_midpoint_color, rgb)
    }

    #[napi]
    pub fn set_maximum_color(&mut self, rgb: u32) -> &Self {
        mutate!(self, set_maximum_color, rgb)
    }

    #[napi]
    pub fn set_multi_range(&mut self, range: String) -> &Self {
        mutate!(self, set_multi_range, range)
    }
}

// ---------------------------------------------------------------------------
// ConditionalFormatDataBar
// ---------------------------------------------------------------------------

#[napi]
pub struct ConditionalFormatDataBar {
    pub(crate) inner: x::ConditionalFormatDataBar,
}

#[napi]
impl ConditionalFormatDataBar {
    #[napi(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: x::ConditionalFormatDataBar::new(),
        }
    }

    /// Bar fill color (24-bit RGB).
    #[napi]
    pub fn set_fill_color(&mut self, rgb: u32) -> &Self {
        mutate!(self, set_fill_color, rgb)
    }

    /// Bar border color (24-bit RGB).
    #[napi]
    pub fn set_border_color(&mut self, rgb: u32) -> &Self {
        mutate!(self, set_border_color, rgb)
    }

    /// Fill color for negative values (24-bit RGB).
    #[napi]
    pub fn set_negative_fill_color(&mut self, rgb: u32) -> &Self {
        mutate!(self, set_negative_fill_color, rgb)
    }

    /// Use a solid fill instead of a gradient.
    #[napi]
    pub fn set_solid_fill(&mut self, enable: bool) -> &Self {
        mutate!(self, set_solid_fill, enable)
    }

    /// Turn the bar border off.
    #[napi]
    pub fn set_border_off(&mut self, enable: bool) -> &Self {
        mutate!(self, set_border_off, enable)
    }

    #[napi]
    pub fn set_multi_range(&mut self, range: String) -> &Self {
        mutate!(self, set_multi_range, range)
    }
}
