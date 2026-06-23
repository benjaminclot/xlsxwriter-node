import { test } from 'node:test'
import assert from 'node:assert/strict'
import { zipRead, xlsxPath } from './helpers.mjs'
import { readFileSync, rmSync } from 'node:fs'
import { createRequire } from 'node:module'

const require = createRequire(import.meta.url)
const {
  Workbook,
  Format,
  ConditionalFormatCell,
  ConditionalFormat3ColorScale,
  ConditionalFormatDataBar,
  ConditionalFormatCellRuleType,
} = require('../index.js')

test('applies conditional formats and produces a valid xlsx', () => {
  const wb = new Workbook()
  const sheet = wb.addWorksheet('CF')

  // A grid of numbers in A1:A10.
  for (let r = 0; r < 10; r++) sheet.write(r, 0, (r + 1) * 7)
  // A second column for the color scale / data bar.
  for (let r = 0; r < 10; r++) sheet.write(r, 1, r * r)

  // Cell rule: highlight values > 35 in red.
  const red = new Format().setBackgroundColor(0xffc7ce).setFontColor(0x9c0006)
  const cell = new ConditionalFormatCell()
    .setRule(ConditionalFormatCellRuleType.GreaterThan, 35)
    .setFormat(red)
  sheet.addConditionalFormatCell(0, 0, 9, 0, cell)

  // Between-rule needs a second value.
  const between = new ConditionalFormatCell().setRule(
    ConditionalFormatCellRuleType.Between,
    10,
    50,
  )
  sheet.addConditionalFormatCell(0, 0, 9, 0, between)

  // 3-color scale and a data bar on column B.
  const scale = new ConditionalFormat3ColorScale()
    .setMinimumColor(0xf8696b)
    .setMidpointColor(0xffeb84)
    .setMaximumColor(0x63be7b)
  sheet.addConditionalFormat3ColorScale(0, 1, 9, 1, scale)

  const bar = new ConditionalFormatDataBar().setFillColor(0x638ec6).setSolidFill(true)
  sheet.addConditionalFormatDataBar(0, 1, 9, 1, bar)

  const outPath = xlsxPath('cf-out.xlsx')
  wb.save(outPath)

  const bytes = readFileSync(outPath)
  assert.equal(bytes[0], 0x50)
  assert.equal(bytes[1], 0x4b)

  // The sheet XML must contain a conditionalFormatting block.
  const xml = zipRead(outPath, 'xl/worksheets/sheet1.xml')
  assert.match(xml, /<conditionalFormatting/)

  rmSync(outPath)
})

test('Between rule without a second value throws', () => {
  const cf = new ConditionalFormatCell()
  assert.throws(() => cf.setRule(ConditionalFormatCellRuleType.Between, 10))
})
