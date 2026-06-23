//! Data validation.
//!
//! A single `#[napi]` `DataValidation` class wraps the owned
//! `rust_xlsxwriter::DataValidation`. It is `Clone` and its setters consume
//! `self`, so we use the clone-and-replace builder pattern and return `&Self`
//! for JS chaining. Apply it to a range with `Worksheet.addDataValidation`.
//!
//! The upstream rule type is the generic `DataValidationRule<T>`; rather than
//! exposing it, the `allowWholeNumber`/`allowDecimalNumber`/`allowTextLength`
//! methods take a [`DataValidationRuleType`] plus numeric operands and build the
//! rule internally (`Between`/`NotBetween` require a second value).
//!
//! Supported allow-types: whole number, decimal number, text length, list of
//! literal strings, list-from-formula, custom formula, and any-value.
//!
//! Omitted for now (can be added later): date/time rules and the date/time
//! `*_formula` variants (would need datetime operands). Numeric operands are
//! taken as JS numbers; whole-number and text-length values are truncated to
//! integers.

use napi::bindgen_prelude::Result;
use rust_xlsxwriter as x;

use crate::enums::{DataValidationErrorStyle, DataValidationRuleType};

/// Build an upstream `DataValidationRule<T>` from a napi rule type and operands.
fn make_rule<T: x::IntoDataValidationValue>(
    rule: DataValidationRuleType,
    value1: T,
    value2: Option<T>,
) -> Result<x::DataValidationRule<T>> {
    use x::DataValidationRule as R;
    let second = || {
        value2.ok_or_else(|| {
            napi::Error::from_reason(
                "Between/NotBetween rules require a second value".to_owned(),
            )
        })
    };
    Ok(match rule {
        DataValidationRuleType::EqualTo => R::EqualTo(value1),
        DataValidationRuleType::NotEqualTo => R::NotEqualTo(value1),
        DataValidationRuleType::GreaterThan => R::GreaterThan(value1),
        DataValidationRuleType::GreaterThanOrEqualTo => R::GreaterThanOrEqualTo(value1),
        DataValidationRuleType::LessThan => R::LessThan(value1),
        DataValidationRuleType::LessThanOrEqualTo => R::LessThanOrEqualTo(value1),
        DataValidationRuleType::Between => R::Between(value1, second()?),
        DataValidationRuleType::NotBetween => R::NotBetween(value1, second()?),
    })
}

#[napi]
pub struct DataValidation {
    pub(crate) inner: x::DataValidation,
}

/// Apply a consuming builder method to the `Clone` inner value in place.
macro_rules! mutate {
    ($self:ident, $method:ident $(, $arg:expr )* ) => {{
        let current = $self.inner.clone();
        $self.inner = current.$method($($arg),*);
        $self
    }};
}

/// Like `mutate!` but for setters that return `Result<DataValidation, _>`.
macro_rules! try_mutate {
    ($self:ident, $method:ident $(, $arg:expr )* ) => {{
        let current = $self.inner.clone();
        $self.inner = current.$method($($arg),*).map_err(crate::error::to_napi)?;
        Ok($self)
    }};
}

#[napi]
impl DataValidation {
    #[napi(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            inner: x::DataValidation::new(),
        }
    }

    /// Restrict input to whole (integer) numbers matching the rule. Operands are
    /// truncated to integers.
    #[napi]
    pub fn allow_whole_number(
        &mut self,
        rule: DataValidationRuleType,
        value1: f64,
        value2: Option<f64>,
    ) -> Result<&Self> {
        let rule = make_rule(rule, value1 as i32, value2.map(|v| v as i32))?;
        mutate_ok(self, |dv| dv.allow_whole_number(rule))
    }

    /// Restrict input to decimal numbers matching the rule.
    #[napi]
    pub fn allow_decimal_number(
        &mut self,
        rule: DataValidationRuleType,
        value1: f64,
        value2: Option<f64>,
    ) -> Result<&Self> {
        let rule = make_rule(rule, value1, value2)?;
        mutate_ok(self, |dv| dv.allow_decimal_number(rule))
    }

    /// Restrict input by text length matching the rule. Operands are truncated to
    /// non-negative integers.
    #[napi]
    pub fn allow_text_length(
        &mut self,
        rule: DataValidationRuleType,
        value1: f64,
        value2: Option<f64>,
    ) -> Result<&Self> {
        let rule = make_rule(rule, value1 as u32, value2.map(|v| v as u32))?;
        mutate_ok(self, |dv| dv.allow_text_length(rule))
    }

    /// Restrict input to a dropdown list of literal string values.
    #[napi]
    pub fn allow_list(&mut self, values: Vec<String>) -> Result<&Self> {
        try_mutate!(self, allow_list_strings, &values)
    }

    /// Restrict input to a list defined by a formula/range, e.g. `"=$A$1:$A$5"`.
    #[napi]
    pub fn allow_list_formula(&mut self, formula: String) -> &Self {
        mutate!(self, allow_list_formula, x::Formula::new(formula.as_str()))
    }

    /// Restrict input using a custom formula, e.g. `"=ISNUMBER(A1)"`.
    #[napi]
    pub fn allow_custom(&mut self, formula: String) -> &Self {
        mutate!(self, allow_custom, x::Formula::new(formula.as_str()))
    }

    /// Allow any value (clears restrictions; useful with an input message only).
    #[napi]
    pub fn allow_any_value(&mut self) -> &Self {
        mutate!(self, allow_any_value)
    }

    /// Whether blank cells are allowed (default: true).
    #[napi]
    pub fn set_ignore_blank(&mut self, enable: bool) -> &Self {
        mutate!(self, ignore_blank, enable)
    }

    /// Whether to show the in-cell dropdown for list rules (default: true).
    #[napi]
    pub fn set_show_dropdown(&mut self, enable: bool) -> &Self {
        mutate!(self, show_dropdown, enable)
    }

    /// Whether to show the input message when the cell is selected.
    #[napi]
    pub fn set_show_input_message(&mut self, enable: bool) -> &Self {
        mutate!(self, show_input_message, enable)
    }

    /// Whether to show the error alert when invalid data is entered.
    #[napi]
    pub fn set_show_error_message(&mut self, enable: bool) -> &Self {
        mutate!(self, show_error_message, enable)
    }

    /// Title of the input message popup (max 32 chars).
    #[napi]
    pub fn set_input_title(&mut self, text: String) -> Result<&Self> {
        try_mutate!(self, set_input_title, text)
    }

    /// Body of the input message popup (max 255 chars).
    #[napi]
    pub fn set_input_message(&mut self, text: String) -> Result<&Self> {
        try_mutate!(self, set_input_message, text)
    }

    /// Title of the error alert dialog (max 32 chars).
    #[napi]
    pub fn set_error_title(&mut self, text: String) -> Result<&Self> {
        try_mutate!(self, set_error_title, text)
    }

    /// Body of the error alert dialog (max 255 chars).
    #[napi]
    pub fn set_error_message(&mut self, text: String) -> Result<&Self> {
        try_mutate!(self, set_error_message, text)
    }

    /// Style of the error alert dialog.
    #[napi]
    pub fn set_error_style(&mut self, style: DataValidationErrorStyle) -> &Self {
        mutate!(self, set_error_style, style.into())
    }

    /// Apply the validation to an additional, non-contiguous range.
    #[napi]
    pub fn set_multi_range(&mut self, range: String) -> &Self {
        mutate!(self, set_multi_range, range)
    }
}

/// Apply a consuming `allow_*` method (taking the prebuilt rule by move) and
/// return `&Self`. Kept as a free fn because the rule is captured in a closure.
fn mutate_ok<F>(this: &mut DataValidation, f: F) -> Result<&DataValidation>
where
    F: FnOnce(x::DataValidation) -> x::DataValidation,
{
    let current = this.inner.clone();
    this.inner = f(current);
    Ok(this)
}
