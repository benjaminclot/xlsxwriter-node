# xlsxwriter-node

Node.js bindings for the [`rust_xlsxwriter`](https://github.com/jmcnamara/rust_xlsxwriter)
Excel `.xlsx` writer, built with [napi-rs](https://napi.rs/).

## Build

```bash
npm install
npm run build         # release build: napi build --platform --release
npm run build:debug   # faster, unoptimized
```

This produces a native `*.node` addon plus generated `index.js` (loader) and
`index.d.ts` (TypeScript types). Requires a Rust toolchain.

## Usage

```js
const { Workbook, Format, FormatAlign, FormatPattern, FormatBorder, Image } = require('xlsxwriter-node')

const wb = new Workbook()
const sheet = wb.addWorksheet('Report')

const title = new Format()
  .setBold()
  .setFontColor(0xff0000)
  .setAlign(FormatAlign.Center)
  .setBackgroundColor(0xffff00)
  .setPattern(FormatPattern.Solid)
  .setBorder(FormatBorder.Thin)

sheet.mergeRange(0, 0, 0, 2, 'Sales Report', title)
sheet.write(1, 0, 'Widget')      // string
sheet.write(1, 1, 42)            // number
sheet.write(1, 2, true)          // boolean
sheet.writeDatetime(2, 0, new Date())
sheet.writeFormula(3, 1, '=SUM(B2:B2)')
sheet.setColumnWidth(0, 20)

sheet.insertImage(5, 0, Image.fromPath('logo.png').setScaleWidth(0.5))

await wb.save('report.xlsx')
// or, for HTTP responses / streaming, get the bytes directly:
const buf = wb.saveToBuffer()    // Node Buffer
```

## Example

[`examples/showcase.mjs`](examples/showcase.mjs) exercises **every** exposed feature
and writes a workbook you can open in Excel to verify the output (it is not deleted,
unlike the test outputs):

```bash
npm run example                      # writes examples/showcase.xlsx
npm run example -- path/to/out.xlsx  # or a path of your choice
```

## API surface (current)

- **`Workbook`**: `new`, `addWorksheet(name?)`, `setProperties(docProperties)`,
  `save(path)`, `saveToBuffer()`.
- **`Worksheet`**: `write` (auto-dispatch on number/boolean/string/Date),
  `writeWithFormat`, `writeString`, `writeNumber`, `writeBoolean`, `writeFormula`,
  `writeDatetime`, `writeUrl`, `setName`, `setColumnWidth`, `setRowHeight`,
  `setColumnFormat`, `setRowFormat`, `mergeRange`, `setFreezePanes`, `insertImage`,
  `insertChart`, `addTable`, `addConditionalFormatCell`, `addConditionalFormatFormula`,
  `addConditionalFormat2ColorScale`, `addConditionalFormat3ColorScale`,
  `addConditionalFormatDataBar`, `addDataValidation`, `insertNote`, `autofilter`.
  - Page setup / print: `setLandscape`, `setPortrait`, `setPaperSize`, `setZoom`,
    `setMargins`, `setHeader`, `setFooter`, `setPrintGridlines`, `setScreenGridlines`,
    `setPrintFitToPages`, `setPrintArea`, `setRepeatRows`, `setRepeatColumns`,
    `setTabColor`, `setRightToLeft`, `setHidden`.
- **`Format`**: chainable setters — `setBold`, `setItalic`, `setFontStrikethrough`,
  `setTextWrap`, `setFontName`, `setFontSize`, `setFontColor`, `setUnderline`,
  `setNumFormat`, `setAlign`, `setIndent`, `setRotation`, `setBackgroundColor`,
  `setForegroundColor`, `setPattern`, `setBorder`.
- **`Image`**: `fromPath`, `fromBuffer`, `setScaleWidth`, `setScaleHeight`, `setAltText`.
- **`Table`**: chainable builder — `setColumns([{ header?, totalLabel?, totalFunction? }])`,
  `setName`, `setStyle`, `setHeaderRow`, `setTotalRow`, `setBandedRows`,
  `setBandedColumns`, `setFirstColumn`, `setLastColumn`, `setAutofilter`, `setAltText`.
- **`Chart`** (constructed with a `ChartType`, inserted via `Worksheet.insertChart`):
  chainable builder — `addSeries({ values, categories?, name? })`, `setTitle`,
  `setXAxisName`, `setYAxisName`, `setLegendPosition`, `setLegendHidden`, `setStyle`.
- **Conditional formats** (each chainable, applied via the matching `Worksheet`
  `addConditionalFormat*` method):
  - `ConditionalFormatCell`: `setRule(ruleType, value1, value2?)`, `setFormat`,
    `setMultiRange`, `setStopIfTrue`.
  - `ConditionalFormatFormula`: `setRule(formula)`, `setFormat`, `setMultiRange`, `setStopIfTrue`.
  - `ConditionalFormat2ColorScale`: `setMinimumColor`, `setMaximumColor`, `setMultiRange`.
  - `ConditionalFormat3ColorScale`: `setMinimumColor`, `setMidpointColor`,
    `setMaximumColor`, `setMultiRange`.
  - `ConditionalFormatDataBar`: `setFillColor`, `setBorderColor`, `setNegativeFillColor`,
    `setSolidFill`, `setBorderOff`, `setMultiRange`.
- **`DataValidation`** (chainable builder, applied via `Worksheet.addDataValidation`):
  `allowWholeNumber(ruleType, value1, value2?)`, `allowDecimalNumber(...)`,
  `allowTextLength(...)`, `allowList(values[])`, `allowListFormula(formula)`,
  `allowCustom(formula)`, `allowAnyValue`, `setIgnoreBlank`, `setShowDropdown`,
  `setShowInputMessage`, `setShowErrorMessage`, `setInputTitle`, `setInputMessage`,
  `setErrorTitle`, `setErrorMessage`, `setErrorStyle`, `setMultiRange`.
- **`DocProperties`** (chainable builder, applied via `Workbook.setProperties`):
  `setTitle`, `setSubject`, `setAuthor`, `setManager`, `setCompany`, `setCategory`,
  `setKeywords`, `setComment`, `setStatus`, `setHyperlinkBase`,
  `setCustomProperty(name, value)`.
- **`Note`** (chainable builder, inserted via `Worksheet.insertNote`): `new(text)`,
  `setAuthor`, `addAuthorPrefix`, `setWidth`, `setHeight`, `setVisible`,
  `setBackgroundColor`, `setFontName`, `setFontSize`, `setFormat`, `setAltText`.
- **Enums**: `FormatAlign`, `FormatBorder`, `FormatPattern`, `FormatUnderline`,
  `TableStyle`, `TableFunction`, `ChartType`, `ChartLegendPosition`,
  `ConditionalFormatCellRuleType`, `DataValidationRuleType`, `DataValidationErrorStyle`.

## Notes & caveats

- **Rows/columns are zero-indexed.** Rows are `u32` (max 1,048,575), columns are
  `u16` (max 16,383) — the Excel limits.
- **Colors** are 24-bit RGB integers, e.g. `0xFF0000` for red.
- **Dates**: a JS `Date` is interpreted in **UTC** (its wall-clock UTC value is
  written as the Excel datetime). Convert to the desired zone before writing if needed.
- **Threading**: the binding uses `Rc<RefCell<..>>` internally and is intended for
  single-threaded (main-thread) use, matching normal Node usage. Do not share a
  `Workbook`/`Worksheet` across worker threads.

## Versioning

This package's version **tracks the wrapped `rust_xlsxwriter`'s `MAJOR.MINOR` and owns
the `PATCH`**: `0.<rust_xlsxwriter-minor>.<release>`. So any `0.95.x` wraps
`rust_xlsxwriter` 0.95; the next upstream minor moves the package to `0.96.0`.

- The package's patch number is an independent release counter — it does **not** mirror
  upstream's patch.
- The exact wrapped `rust_xlsxwriter` version is pinned in `Cargo.toml` (and recorded in
  `Cargo.lock`).
- Because this is `0.x`, treat any release as potentially containing changes — pin a
  version range accordingly.

## Architecture

`rust_xlsxwriter` hands out worksheets as `&mut` references borrowed from the
`Workbook`, which cannot cross into a GC-owned JS object. So the `Workbook` holds
its inner value in an `Rc<RefCell<rust_xlsxwriter::Workbook>>`, and each JS
`Worksheet` is a lightweight handle holding a clone of that `Rc` plus the sheet
index. Every worksheet call re-borrows the sheet by index. Helper objects
(`Format`, `Image`, …) are owned wrappers passed into worksheet calls by reference.

## Coverage & roadmap

Implemented: core writes, formatting, merge, freeze panes, column/row sizing, images,
charts, tables, conditional formats, data validation, page setup / print options,
autofilter, document properties, and cell notes.

See **[COVERAGE.md](COVERAGE.md)** for a feature-by-feature comparison against the
upstream `rust_xlsxwriter` crate and the prioritized backlog (sparklines, shapes,
buttons, worksheet protection, the full chart object model, per-column autofilter
conditions, and more).
