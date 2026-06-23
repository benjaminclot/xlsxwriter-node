import { test } from 'node:test'
import assert from 'node:assert/strict'
import { zipEntries } from './helpers.mjs'
import { readFileSync, existsSync, rmSync } from 'node:fs'
import { createRequire } from 'node:module'

const require = createRequire(import.meta.url)
const {
  Workbook,
  Format,
  FormatAlign,
  FormatBorder,
  FormatPattern,
} = require('../index.js')

test('writes a styled workbook to disk and buffer', () => {
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
  sheet.write(1, 0, 'Item')
  sheet.write(1, 1, 'Qty')
  sheet.write(1, 2, 'Price')
  sheet.write(2, 0, 'Widget')
  sheet.write(2, 1, 42)
  sheet.write(2, 2, 9.99)
  sheet.write(3, 0, true)
  sheet.writeDatetime(4, 0, new Date('2026-06-23T12:00:00Z'))
  sheet.writeFormula(5, 1, '=SUM(B3:B3)')
  sheet.setColumnWidth(0, 20)

  const outPath = new URL('./out.xlsx', import.meta.url).pathname
  if (existsSync(outPath)) rmSync(outPath)
  wb.save(outPath)

  // A .xlsx is a zip; bytes 0..2 must be the local-file-header magic "PK".
  const bytes = readFileSync(outPath)
  assert.equal(bytes[0], 0x50) // 'P'
  assert.equal(bytes[1], 0x4b) // 'K'
  assert.ok(bytes.length > 1000, 'file should be non-trivial')

  // The central directory should list the worksheet XML part.
  const names = zipEntries(outPath)
  assert.match(names, /xl\/worksheets\/sheet1\.xml/)

  rmSync(outPath)
})

test('saveToBuffer returns a valid xlsx buffer', () => {
  const wb = new Workbook()
  const sheet = wb.addWorksheet()
  sheet.write(0, 0, 'hello')
  const buf = wb.saveToBuffer()
  assert.ok(Buffer.isBuffer(buf))
  assert.equal(buf[0], 0x50)
  assert.equal(buf[1], 0x4b)
})
