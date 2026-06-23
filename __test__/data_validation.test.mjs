import { test } from 'node:test'
import assert from 'node:assert/strict'
import { execFileSync } from 'node:child_process'
import { readFileSync, existsSync, rmSync } from 'node:fs'
import { createRequire } from 'node:module'

const require = createRequire(import.meta.url)
const { Workbook, DataValidation, DataValidationRuleType, DataValidationErrorStyle } =
  require('../index.js')

test('applies numeric and list data validations', () => {
  const wb = new Workbook()
  const sheet = wb.addWorksheet('Data')

  // Numeric range validation on B2 (0..100), with input + error messages.
  const numeric = new DataValidation()
    .allowWholeNumber(DataValidationRuleType.Between, 0, 100)
    .setInputTitle('Enter a score')
    .setInputMessage('A whole number from 0 to 100')
    .setErrorStyle(DataValidationErrorStyle.Warning)
    .setErrorMessage('Out of range')
  sheet.addDataValidation(1, 1, 1, 1, numeric)

  // Dropdown list validation on B3.
  const list = new DataValidation().allowList(['North', 'South', 'East', 'West'])
  sheet.addDataValidation(2, 1, 2, 1, list)

  const outPath = new URL('./dv-out.xlsx', import.meta.url).pathname
  if (existsSync(outPath)) rmSync(outPath)
  wb.save(outPath)

  const bytes = readFileSync(outPath)
  assert.equal(bytes[0], 0x50)
  assert.equal(bytes[1], 0x4b)

  // The worksheet XML must contain dataValidation entries.
  const xml = execFileSync('unzip', ['-p', outPath, 'xl/worksheets/sheet1.xml'], {
    encoding: 'utf8',
  })
  assert.match(xml, /dataValidation/)

  rmSync(outPath)
})

test('Between rule without a second value throws', () => {
  const dv = new DataValidation()
  assert.throws(() => dv.allowDecimalNumber(DataValidationRuleType.Between, 1))
})
