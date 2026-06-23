# Coverage & roadmap

How much of [`rust_xlsxwriter`](https://docs.rs/rust_xlsxwriter) v0.95.0 this Node.js
binding currently exposes. Also serves as the roadmap — unchecked items are the
backlog, roughly in priority order within each section.

**Legend:** ✅ full · 🟡 partial · ⬜ not yet exposed

Counts are indicative (binding members exposed vs. upstream public methods/types).

## Summary

| Area | Status | Binding | Upstream | Notes |
|------|:------:|--------:|---------:|-------|
| Workbook | 🟡 | 5 | ~28 | create / sheets / save / buffer / properties |
| Worksheet — cell writes | 🟡 | — | — | scalars + generic `write`; no bulk/array/rich |
| Worksheet — layout & print | 🟡 | ~16 | ~60 | common page-setup; gaps below |
| Format | 🟡 | 17 | ~50 | common styling; gaps below |
| Image | 🟡 | 5 | ~15 | insert + scale + alt text |
| Chart | 🟡 | 8 | ~40 types | flattened series API only |
| Table | ✅ | 12 | 14 | columns/totals/styles |
| Conditional formats | 🟡 | 5 types | 12 types | cell/formula/scales/data-bar |
| Data validation | ✅ | 18 | ~22 | all rule kinds except date/time |
| Document properties | ✅ | 12 | 14 | strings + custom props |
| Cell notes | ✅ | 11 | ~14 | comments via `Note` |
| Sparklines | ⬜ | 0 | — | not started |
| Shapes / textboxes | ⬜ | 0 | — | not started |
| Buttons / checkboxes | ⬜ | 0 | — | not started |
| Worksheet protection | ⬜ | 0 | — | not started |
| Autofilter conditions | 🟡 | range only | — | per-column filters missing |
| Serde (`#[derive]`) | ⬜ | n/a | — | Rust-only, out of scope |
| VBA / macros | ⬜ | 0 | — | not started |

Overall the binding covers the **common spreadsheet-generation path** end to end
(data, formatting, images, charts, tables, conditional formats, data validation,
page setup, notes, document properties). The large remaining gaps are advanced
worksheet features, the full chart object model, and a handful of whole subsystems.

---

## Workbook — 🟡

Exposed: `new`, `addWorksheet(name?)`, `setProperties`, `save`, `saveToBuffer`.

Backlog:
- [ ] `defineName` (defined names / named ranges)
- [ ] `registerFormat` / `setDefaultFormat`
- [ ] `addChartsheet` (dedicated chart sheets)
- [ ] `saveToWriter` (stream to an arbitrary sink — needs a JS stream bridge)
- [ ] constant-memory / low-memory worksheet modes
- [ ] custom / Excel-2023 themes
- [ ] `readOnlyRecommended`, `useZipLargeFile`, `setTempdir`
- [ ] VBA project (`addVbaProject`, `setVbaName`)
- n/a `worksheetFromName` / `worksheetFromIndex` — not needed; `addWorksheet`
  already returns a live handle.

## Worksheet — cell writes — 🟡

Exposed: `write` (auto-dispatch number/boolean/string/Date), `writeWithFormat`,
`writeString`, `writeNumber`, `writeBoolean`, `writeFormula`, `writeDatetime`,
`writeUrl`.

The generic `write` / `writeWithFormat` subsume the upstream `write_*_with_format`
typed variants, so those aren't exposed individually.

Backlog:
- [ ] Bulk writers: `writeRow`, `writeColumn`, `writeRowMatrix`, `writeColumnMatrix`
- [ ] `writeArrayFormula`, `writeDynamicArrayFormula`, `writeDynamicFormula`
- [ ] `writeRichString` (multi-format runs in one cell)
- [ ] `writeUrlWithText` / `writeUrlWithOptions` (link text, tooltip, format)
- [ ] Explicit `writeDate` / `writeTime` (currently only datetime)
- [ ] `writeBlank`, `clearCell`, `clearCellFormat`
- [ ] `embedImage` (image scaled into a cell, not floating)
- [ ] `setFormulaResult` (cached formula values for non-Excel readers)

## Worksheet — layout & print — 🟡

Exposed: `setColumnWidth`, `setRowHeight`, `setColumnFormat`, `setRowFormat`,
`mergeRange`, `setFreezePanes`, `autofilter`, `setLandscape`, `setPortrait`,
`setPaperSize`, `setZoom`, `setMargins`, `setHeader`, `setFooter`,
`setPrintGridlines`, `setScreenGridlines`, `setPrintFitToPages`, `setPrintArea`,
`setRepeatRows`, `setRepeatColumns`, `setTabColor`, `setRightToLeft`, `setHidden`.

Backlog:
- [ ] `autofit` / `setColumnAutofitWidth` (auto column sizing)
- [ ] Row/column grouping & outlines (`groupRows`, `groupColumns`, collapsed/symbols)
- [ ] `setRowHidden` / `setColumnHidden`, range variants, `hideUnusedRows`
- [ ] Worksheet protection (`protect`, `protectWithPassword`, `protectWithOptions`)
- [ ] View state: `setActive`, `setSelected`, `setSelection`, `setTopLeftCell`,
      page-break-preview / page-layout views
- [ ] Page breaks (`setPageBreaks`, `setVerticalPageBreaks`)
- [ ] More print options: `setPrintHeadings`, `setPrintScale`,
      `setPrintCenterHorizontally`/`Vertically`, `setPrintFirstPageNumber`,
      black-and-white / draft
- [ ] Header/footer images, scale-with-doc / align-with-page
- [ ] `ignoreError` (suppress green-triangle warnings)
- [ ] NaN/Infinity value handling

## Format — 🟡

Exposed (17): `setBold`, `setItalic`, `setFontStrikethrough`, `setTextWrap`,
`setFontName`, `setFontSize`, `setFontColor`, `setUnderline`, `setNumFormat`,
`setAlign`, `setIndent`, `setRotation`, `setBackgroundColor`,
`setForegroundColor`, `setPattern`, `setBorder`.

Backlog:
- [ ] Per-side borders (`setBorderTop`/`Bottom`/`Left`/`Right`) + colors
- [ ] Diagonal borders (`FormatDiagonalBorder`)
- [ ] `setNumFormatIndex`, built-in format indices
- [ ] Cell protection flags: `setLocked`, `setUnlocked`, `setHidden`
- [ ] `setFontScheme`, `setFontScript` (super/subscript), `setFontCharset`
- [ ] `setShrink`, `setReadingDirection`, `setQuotePrefix`
- [ ] Named colors via the upstream `Color` enum (binding uses 24-bit RGB ints only)

## Image — 🟡

Exposed: `fromPath`, `fromBuffer`, `setScaleWidth`, `setScaleHeight`, `setAltText`.

Backlog:
- [ ] `setScaleToSize`, `setObjectMovement` (needs `ObjectMovement` enum)
- [ ] `insertImageWithOffset`, `insertImageFitToCell`(`Centered`)
- [ ] `insertBackgroundImage`, header/footer images
- [ ] DPI / decorative / URL metadata setters

## Chart — 🟡

Exposed: constructor (`ChartType`), `addSeries({values, categories?, name?})`,
`setTitle`, `setXAxisName`, `setYAxisName`, `setLegendPosition`, `setLegendHidden`,
`setStyle`. All 23 `ChartType` variants and `ChartLegendPosition` are mapped.

This is a deliberately **flattened** subset. The upstream chart object model has
~40 types (series, axes, markers, data labels, trendlines, error bars, fills,
fonts, gradients, layouts). Backlog, roughly by value:
- [ ] Series styling: line/fill/marker, data labels, per-point formatting
- [ ] Axis configuration: min/max, units, number format, gridlines, label position
- [ ] `insertChartWithOffset`, chart size, plot/chart-area formatting
- [ ] Trendlines, error bars, up/down bars, data table
- [ ] Combined charts, secondary axes
- [ ] Chartsheets (via `Workbook.addChartsheet`)

## Table — ✅

Exposed: `setColumns([{header?, totalLabel?, totalFunction?}])`, `setName`,
`setStyle`, `setHeaderRow`, `setTotalRow`, `setBandedRows`, `setBandedColumns`,
`setFirstColumn`, `setLastColumn`, `setAutofilter`, `setAltText`. All `TableStyle`
and `TableFunction` variants mapped.

Backlog:
- [ ] Per-column header/cell `Format` and custom column formulas
- [ ] `TableFunction::Custom` (custom totals formula — generic upstream variant)

## Conditional formats — 🟡 (5 of 12 types)

Exposed types: `ConditionalFormatCell`, `ConditionalFormatFormula`,
`ConditionalFormat2ColorScale`, `ConditionalFormat3ColorScale`,
`ConditionalFormatDataBar` — each with a matching `Worksheet.addConditionalFormat*`
method. `ConditionalFormatCellRuleType` enum covers the comparison operators.

Backlog:
- [ ] `ConditionalFormatAverage`, `ConditionalFormatTop` (top/bottom N or %)
- [ ] `ConditionalFormatDuplicate` (duplicate/unique)
- [ ] `ConditionalFormatText` (contains / begins / ends)
- [ ] `ConditionalFormatDate` (date-occurring rules)
- [ ] `ConditionalFormatBlank`, `ConditionalFormatError`
- [ ] `ConditionalFormatIconSet` (+ custom icons)
- [ ] Scale/data-bar min/max **type** anchors (num/percent/percentile/formula),
      data-bar direction & axis position
- [ ] String operands for cell rules (currently numeric `f64` only)

## Data validation — ✅

Exposed (18): `allowWholeNumber`, `allowDecimalNumber`, `allowTextLength`,
`allowList`, `allowListFormula`, `allowCustom`, `allowAnyValue`, plus
`setIgnoreBlank`, `setShowDropdown`, `setShowInputMessage`, `setShowErrorMessage`,
`setInputTitle`, `setInputMessage`, `setErrorTitle`, `setErrorMessage`,
`setErrorStyle`, `setMultiRange`. `DataValidationRuleType` and
`DataValidationErrorStyle` mapped.

Backlog:
- [ ] Date / time rules and their `*_formula` variants (need datetime operands)

## Document properties — ✅

Exposed (12): `setTitle`, `setSubject`, `setAuthor`, `setManager`, `setCompany`,
`setCategory`, `setKeywords`, `setComment`, `setStatus`, `setHyperlinkBase`,
`setCustomProperty(name, value)`.

Backlog:
- [ ] `setCreationDatetime`
- [ ] Non-string custom properties (number / bool / datetime)

## Cell notes — ✅

Exposed (11): `new(text)`, `setAuthor`, `addAuthorPrefix`, `setWidth`, `setHeight`,
`setVisible`, `setBackgroundColor`, `setFontName`, `setFontSize`, `setFormat`,
`setAltText`. Inserted via `Worksheet.insertNote`.

Backlog:
- [ ] `setObjectMovement` (needs `ObjectMovement` enum), `resetText`

---

## Whole subsystems not yet started — ⬜

- [ ] **Sparklines** (`Sparkline`, `SparklineType`; `addSparkline`/`addSparklineGroup`)
- [ ] **Shapes / textboxes** (`Shape` + its fill/line/font/text types)
- [ ] **Buttons & checkboxes** (`Button`; `insertButton`, `insertCheckbox`)
- [ ] **Worksheet protection** (`ProtectionOptions`)
- [ ] **Per-column autofilter conditions** (`FilterCondition`, `FilterCriteria`)
- [ ] **Rich text** (`writeRichString` with multiple `Format` runs)
- [ ] **`ExcelDateTime`** value type (binding uses JS `Date` → UTC only)
- [ ] **`Formula` / `Url` option objects** (currently plain strings)
- [ ] **VBA / macros**, **custom themes**, **chartsheets**
- n/a **Serde `XlsxSerialize` derive** — a Rust compile-time feature; not
  applicable to a JS binding (a JS-object → rows helper could be added instead).

## Contributing a feature

See `CLAUDE.md` → "Adding a new feature module" for the module/enum/wiring/test
pattern. The established conventions (owned `inner` wrappers, flattened builders,
`&Self` chaining, `*RuleType` enums for generic upstream rule enums) make most of
the backlog mechanical.

Every ✅/🟡 item above is demonstrated end-to-end in
[`examples/showcase.mjs`](examples/showcase.mjs) (`npm run example`), which calls
every exposed class and method. When you implement a backlog item, update this file
**and** extend the showcase so it keeps covering the whole surface.
