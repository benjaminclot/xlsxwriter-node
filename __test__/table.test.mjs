import { test } from 'node:test'
import assert from 'node:assert/strict'
import { zipEntries } from './helpers.mjs'
import { readFileSync, existsSync, rmSync } from 'node:fs'
import { createRequire } from 'node:module'

const require = createRequire(import.meta.url)
const { Workbook, Table, TableStyle, TableFunction } = require('../index.js')

test('adds a table with named columns and a totals row', () => {
  const wb = new Workbook()
  const sheet = wb.addWorksheet('Data')

  // Header + data rows. The table will span rows 0..4, cols 0..1, plus a
  // totals row at row 5.
  sheet.write(0, 0, 'Product')
  sheet.write(0, 1, 'Sales')
  sheet.write(1, 0, 'Apples')
  sheet.write(1, 1, 10)
  sheet.write(2, 0, 'Pears')
  sheet.write(2, 1, 20)
  sheet.write(3, 0, 'Plums')
  sheet.write(3, 1, 30)
  sheet.write(4, 0, 'Cherries')
  sheet.write(4, 1, 40)

  const table = new Table()
    .setStyle(TableStyle.Medium9)
    .setTotalRow(true)
    .setBandedRows(true)
    .setColumns([
      { header: 'Product', totalLabel: 'Total' },
      { header: 'Sales', totalFunction: TableFunction.Sum },
    ])

  sheet.addTable(0, 0, 5, 1, table)

  const outPath = new URL('./table-out.xlsx', import.meta.url).pathname
  if (existsSync(outPath)) rmSync(outPath)
  wb.save(outPath)

  const bytes = readFileSync(outPath)
  assert.equal(bytes[0], 0x50) // 'P'
  assert.equal(bytes[1], 0x4b) // 'K'

  const names = zipEntries(outPath)
  assert.match(names, /xl\/tables\/table1\.xml/)

  rmSync(outPath)
})
