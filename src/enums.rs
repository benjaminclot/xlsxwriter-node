//! napi enum mirrors of the `rust_xlsxwriter` formatting enums.
//!
//! Each enum is exposed to JavaScript and converted into its `rust_xlsxwriter`
//! counterpart via `From`. Variant order/names match the upstream crate.

use rust_xlsxwriter as x;

#[napi]
pub enum FormatAlign {
    General,
    Left,
    Center,
    Right,
    Fill,
    Justify,
    CenterAcross,
    Distributed,
    Top,
    Bottom,
    VerticalCenter,
    VerticalJustify,
    VerticalDistributed,
}

impl From<FormatAlign> for x::FormatAlign {
    fn from(value: FormatAlign) -> Self {
        match value {
            FormatAlign::General => x::FormatAlign::General,
            FormatAlign::Left => x::FormatAlign::Left,
            FormatAlign::Center => x::FormatAlign::Center,
            FormatAlign::Right => x::FormatAlign::Right,
            FormatAlign::Fill => x::FormatAlign::Fill,
            FormatAlign::Justify => x::FormatAlign::Justify,
            FormatAlign::CenterAcross => x::FormatAlign::CenterAcross,
            FormatAlign::Distributed => x::FormatAlign::Distributed,
            FormatAlign::Top => x::FormatAlign::Top,
            FormatAlign::Bottom => x::FormatAlign::Bottom,
            FormatAlign::VerticalCenter => x::FormatAlign::VerticalCenter,
            FormatAlign::VerticalJustify => x::FormatAlign::VerticalJustify,
            FormatAlign::VerticalDistributed => x::FormatAlign::VerticalDistributed,
        }
    }
}

#[napi]
pub enum FormatBorder {
    None,
    Thin,
    Medium,
    Dashed,
    Dotted,
    Thick,
    Double,
    Hair,
    MediumDashed,
    DashDot,
    MediumDashDot,
    DashDotDot,
    MediumDashDotDot,
    SlantDashDot,
}

impl From<FormatBorder> for x::FormatBorder {
    fn from(value: FormatBorder) -> Self {
        match value {
            FormatBorder::None => x::FormatBorder::None,
            FormatBorder::Thin => x::FormatBorder::Thin,
            FormatBorder::Medium => x::FormatBorder::Medium,
            FormatBorder::Dashed => x::FormatBorder::Dashed,
            FormatBorder::Dotted => x::FormatBorder::Dotted,
            FormatBorder::Thick => x::FormatBorder::Thick,
            FormatBorder::Double => x::FormatBorder::Double,
            FormatBorder::Hair => x::FormatBorder::Hair,
            FormatBorder::MediumDashed => x::FormatBorder::MediumDashed,
            FormatBorder::DashDot => x::FormatBorder::DashDot,
            FormatBorder::MediumDashDot => x::FormatBorder::MediumDashDot,
            FormatBorder::DashDotDot => x::FormatBorder::DashDotDot,
            FormatBorder::MediumDashDotDot => x::FormatBorder::MediumDashDotDot,
            FormatBorder::SlantDashDot => x::FormatBorder::SlantDashDot,
        }
    }
}

#[napi]
pub enum FormatPattern {
    None,
    Solid,
    MediumGray,
    DarkGray,
    LightGray,
    DarkHorizontal,
    DarkVertical,
    DarkDown,
    DarkUp,
    DarkGrid,
    DarkTrellis,
    LightHorizontal,
    LightVertical,
    LightDown,
    LightUp,
    LightGrid,
    LightTrellis,
    Gray125,
    Gray0625,
}

impl From<FormatPattern> for x::FormatPattern {
    fn from(value: FormatPattern) -> Self {
        match value {
            FormatPattern::None => x::FormatPattern::None,
            FormatPattern::Solid => x::FormatPattern::Solid,
            FormatPattern::MediumGray => x::FormatPattern::MediumGray,
            FormatPattern::DarkGray => x::FormatPattern::DarkGray,
            FormatPattern::LightGray => x::FormatPattern::LightGray,
            FormatPattern::DarkHorizontal => x::FormatPattern::DarkHorizontal,
            FormatPattern::DarkVertical => x::FormatPattern::DarkVertical,
            FormatPattern::DarkDown => x::FormatPattern::DarkDown,
            FormatPattern::DarkUp => x::FormatPattern::DarkUp,
            FormatPattern::DarkGrid => x::FormatPattern::DarkGrid,
            FormatPattern::DarkTrellis => x::FormatPattern::DarkTrellis,
            FormatPattern::LightHorizontal => x::FormatPattern::LightHorizontal,
            FormatPattern::LightVertical => x::FormatPattern::LightVertical,
            FormatPattern::LightDown => x::FormatPattern::LightDown,
            FormatPattern::LightUp => x::FormatPattern::LightUp,
            FormatPattern::LightGrid => x::FormatPattern::LightGrid,
            FormatPattern::LightTrellis => x::FormatPattern::LightTrellis,
            FormatPattern::Gray125 => x::FormatPattern::Gray125,
            FormatPattern::Gray0625 => x::FormatPattern::Gray0625,
        }
    }
}

#[napi]
pub enum FormatUnderline {
    None,
    Single,
    Double,
    SingleAccounting,
    DoubleAccounting,
}

impl From<FormatUnderline> for x::FormatUnderline {
    fn from(value: FormatUnderline) -> Self {
        match value {
            FormatUnderline::None => x::FormatUnderline::None,
            FormatUnderline::Single => x::FormatUnderline::Single,
            FormatUnderline::Double => x::FormatUnderline::Double,
            FormatUnderline::SingleAccounting => x::FormatUnderline::SingleAccounting,
            FormatUnderline::DoubleAccounting => x::FormatUnderline::DoubleAccounting,
        }
    }
}

#[napi]
pub enum ChartType {
    Area,
    AreaStacked,
    AreaPercentStacked,
    Bar,
    BarStacked,
    BarPercentStacked,
    Column,
    ColumnStacked,
    ColumnPercentStacked,
    Doughnut,
    Line,
    LineStacked,
    LinePercentStacked,
    Pie,
    Radar,
    RadarWithMarkers,
    RadarFilled,
    Scatter,
    ScatterStraight,
    ScatterStraightWithMarkers,
    ScatterSmooth,
    ScatterSmoothWithMarkers,
    Stock,
}

impl From<ChartType> for x::ChartType {
    fn from(value: ChartType) -> Self {
        match value {
            ChartType::Area => x::ChartType::Area,
            ChartType::AreaStacked => x::ChartType::AreaStacked,
            ChartType::AreaPercentStacked => x::ChartType::AreaPercentStacked,
            ChartType::Bar => x::ChartType::Bar,
            ChartType::BarStacked => x::ChartType::BarStacked,
            ChartType::BarPercentStacked => x::ChartType::BarPercentStacked,
            ChartType::Column => x::ChartType::Column,
            ChartType::ColumnStacked => x::ChartType::ColumnStacked,
            ChartType::ColumnPercentStacked => x::ChartType::ColumnPercentStacked,
            ChartType::Doughnut => x::ChartType::Doughnut,
            ChartType::Line => x::ChartType::Line,
            ChartType::LineStacked => x::ChartType::LineStacked,
            ChartType::LinePercentStacked => x::ChartType::LinePercentStacked,
            ChartType::Pie => x::ChartType::Pie,
            ChartType::Radar => x::ChartType::Radar,
            ChartType::RadarWithMarkers => x::ChartType::RadarWithMarkers,
            ChartType::RadarFilled => x::ChartType::RadarFilled,
            ChartType::Scatter => x::ChartType::Scatter,
            ChartType::ScatterStraight => x::ChartType::ScatterStraight,
            ChartType::ScatterStraightWithMarkers => x::ChartType::ScatterStraightWithMarkers,
            ChartType::ScatterSmooth => x::ChartType::ScatterSmooth,
            ChartType::ScatterSmoothWithMarkers => x::ChartType::ScatterSmoothWithMarkers,
            ChartType::Stock => x::ChartType::Stock,
        }
    }
}

#[napi]
pub enum ChartLegendPosition {
    Right,
    Left,
    Top,
    Bottom,
    TopRight,
}

impl From<ChartLegendPosition> for x::ChartLegendPosition {
    fn from(value: ChartLegendPosition) -> Self {
        match value {
            ChartLegendPosition::Right => x::ChartLegendPosition::Right,
            ChartLegendPosition::Left => x::ChartLegendPosition::Left,
            ChartLegendPosition::Top => x::ChartLegendPosition::Top,
            ChartLegendPosition::Bottom => x::ChartLegendPosition::Bottom,
            ChartLegendPosition::TopRight => x::ChartLegendPosition::TopRight,
        }
    }
}

#[napi]
pub enum TableStyle {
    None,
    Light1,
    Light2,
    Light3,
    Light4,
    Light5,
    Light6,
    Light7,
    Light8,
    Light9,
    Light10,
    Light11,
    Light12,
    Light13,
    Light14,
    Light15,
    Light16,
    Light17,
    Light18,
    Light19,
    Light20,
    Light21,
    Medium1,
    Medium2,
    Medium3,
    Medium4,
    Medium5,
    Medium6,
    Medium7,
    Medium8,
    Medium9,
    Medium10,
    Medium11,
    Medium12,
    Medium13,
    Medium14,
    Medium15,
    Medium16,
    Medium17,
    Medium18,
    Medium19,
    Medium20,
    Medium21,
    Medium22,
    Medium23,
    Medium24,
    Medium25,
    Medium26,
    Medium27,
    Medium28,
    Dark1,
    Dark2,
    Dark3,
    Dark4,
    Dark5,
    Dark6,
    Dark7,
    Dark8,
    Dark9,
    Dark10,
    Dark11,
}

impl From<TableStyle> for x::TableStyle {
    fn from(value: TableStyle) -> Self {
        match value {
            TableStyle::None => x::TableStyle::None,
            TableStyle::Light1 => x::TableStyle::Light1,
            TableStyle::Light2 => x::TableStyle::Light2,
            TableStyle::Light3 => x::TableStyle::Light3,
            TableStyle::Light4 => x::TableStyle::Light4,
            TableStyle::Light5 => x::TableStyle::Light5,
            TableStyle::Light6 => x::TableStyle::Light6,
            TableStyle::Light7 => x::TableStyle::Light7,
            TableStyle::Light8 => x::TableStyle::Light8,
            TableStyle::Light9 => x::TableStyle::Light9,
            TableStyle::Light10 => x::TableStyle::Light10,
            TableStyle::Light11 => x::TableStyle::Light11,
            TableStyle::Light12 => x::TableStyle::Light12,
            TableStyle::Light13 => x::TableStyle::Light13,
            TableStyle::Light14 => x::TableStyle::Light14,
            TableStyle::Light15 => x::TableStyle::Light15,
            TableStyle::Light16 => x::TableStyle::Light16,
            TableStyle::Light17 => x::TableStyle::Light17,
            TableStyle::Light18 => x::TableStyle::Light18,
            TableStyle::Light19 => x::TableStyle::Light19,
            TableStyle::Light20 => x::TableStyle::Light20,
            TableStyle::Light21 => x::TableStyle::Light21,
            TableStyle::Medium1 => x::TableStyle::Medium1,
            TableStyle::Medium2 => x::TableStyle::Medium2,
            TableStyle::Medium3 => x::TableStyle::Medium3,
            TableStyle::Medium4 => x::TableStyle::Medium4,
            TableStyle::Medium5 => x::TableStyle::Medium5,
            TableStyle::Medium6 => x::TableStyle::Medium6,
            TableStyle::Medium7 => x::TableStyle::Medium7,
            TableStyle::Medium8 => x::TableStyle::Medium8,
            TableStyle::Medium9 => x::TableStyle::Medium9,
            TableStyle::Medium10 => x::TableStyle::Medium10,
            TableStyle::Medium11 => x::TableStyle::Medium11,
            TableStyle::Medium12 => x::TableStyle::Medium12,
            TableStyle::Medium13 => x::TableStyle::Medium13,
            TableStyle::Medium14 => x::TableStyle::Medium14,
            TableStyle::Medium15 => x::TableStyle::Medium15,
            TableStyle::Medium16 => x::TableStyle::Medium16,
            TableStyle::Medium17 => x::TableStyle::Medium17,
            TableStyle::Medium18 => x::TableStyle::Medium18,
            TableStyle::Medium19 => x::TableStyle::Medium19,
            TableStyle::Medium20 => x::TableStyle::Medium20,
            TableStyle::Medium21 => x::TableStyle::Medium21,
            TableStyle::Medium22 => x::TableStyle::Medium22,
            TableStyle::Medium23 => x::TableStyle::Medium23,
            TableStyle::Medium24 => x::TableStyle::Medium24,
            TableStyle::Medium25 => x::TableStyle::Medium25,
            TableStyle::Medium26 => x::TableStyle::Medium26,
            TableStyle::Medium27 => x::TableStyle::Medium27,
            TableStyle::Medium28 => x::TableStyle::Medium28,
            TableStyle::Dark1 => x::TableStyle::Dark1,
            TableStyle::Dark2 => x::TableStyle::Dark2,
            TableStyle::Dark3 => x::TableStyle::Dark3,
            TableStyle::Dark4 => x::TableStyle::Dark4,
            TableStyle::Dark5 => x::TableStyle::Dark5,
            TableStyle::Dark6 => x::TableStyle::Dark6,
            TableStyle::Dark7 => x::TableStyle::Dark7,
            TableStyle::Dark8 => x::TableStyle::Dark8,
            TableStyle::Dark9 => x::TableStyle::Dark9,
            TableStyle::Dark10 => x::TableStyle::Dark10,
            TableStyle::Dark11 => x::TableStyle::Dark11,
        }
    }
}

/// Total-row aggregation function. (`Custom` formulas are not exposed here.)
#[napi]
pub enum TableFunction {
    None,
    Average,
    Count,
    CountNumbers,
    Max,
    Min,
    Sum,
    StdDev,
    Var,
}

impl From<TableFunction> for x::TableFunction {
    fn from(value: TableFunction) -> Self {
        match value {
            TableFunction::None => x::TableFunction::None,
            TableFunction::Average => x::TableFunction::Average,
            TableFunction::Count => x::TableFunction::Count,
            TableFunction::CountNumbers => x::TableFunction::CountNumbers,
            TableFunction::Max => x::TableFunction::Max,
            TableFunction::Min => x::TableFunction::Min,
            TableFunction::Sum => x::TableFunction::Sum,
            TableFunction::StdDev => x::TableFunction::StdDev,
            TableFunction::Var => x::TableFunction::Var,
        }
    }
}

/// Comparison rule for `ConditionalFormatCell`. `Between`/`NotBetween` require a
/// second value; all others use a single value.
#[napi]
pub enum ConditionalFormatCellRuleType {
    EqualTo,
    NotEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    Between,
    NotBetween,
}

/// Comparison rule for numeric/text-length `DataValidation` rules.
/// `Between`/`NotBetween` require a second value; all others use a single value.
/// (Mapped internally to the upstream generic `DataValidationRule<T>`.)
#[napi]
pub enum DataValidationRuleType {
    EqualTo,
    NotEqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    Between,
    NotBetween,
}

/// Dialog style shown when invalid data is entered.
#[napi]
pub enum DataValidationErrorStyle {
    Stop,
    Warning,
    Information,
}

impl From<DataValidationErrorStyle> for x::DataValidationErrorStyle {
    fn from(value: DataValidationErrorStyle) -> Self {
        match value {
            DataValidationErrorStyle::Stop => x::DataValidationErrorStyle::Stop,
            DataValidationErrorStyle::Warning => x::DataValidationErrorStyle::Warning,
            DataValidationErrorStyle::Information => x::DataValidationErrorStyle::Information,
        }
    }
}
