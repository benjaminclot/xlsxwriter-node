import { test } from 'node:test'
import assert from 'node:assert/strict'
import { zipRead } from './helpers.mjs'
import { readFileSync, rmSync } from 'node:fs'
import { createRequire } from 'node:module'

const require = createRequire(import.meta.url)
const { Workbook, DocProperties, Note } = require('../index.js')

test('page setup, autofilter, properties and notes', () => {
  const wb = new Workbook()

  const props = new DocProperties()
    .setTitle('Quarterly Report')
    .setAuthor('Ada Lovelace')
    .setCompany('Analytical Engines')
  wb.setProperties(props)

  const sheet = wb.addWorksheet('Data')

  // Header row + a few data rows for the autofilter.
  sheet.write(0, 0, 'Name')
  sheet.write(0, 1, 'Score')
  sheet.write(1, 0, 'Alice')
  sheet.write(1, 1, 90)
  sheet.write(2, 0, 'Bob')
  sheet.write(2, 1, 75)

  // Page setup / print options.
  sheet.setLandscape()
  sheet.setMargins(0.5, 0.5, 0.75, 0.75, 0.3, 0.3)
  sheet.setHeader('&CQuarterly Report')
  sheet.setFooter('&CPage &P of &N')
  sheet.setPrintGridlines(true)
  sheet.setZoom(125)
  sheet.setPrintFitToPages(1, 0)
  sheet.setTabColor(0x1f78b4)

  // Autofilter over the populated range.
  sheet.autofilter(0, 0, 2, 1)

  // A cell note.
  const note = new Note('Top performer').setAuthor('Ada').setVisible(true)
  sheet.insertNote(1, 1, note)

  const outPath = new URL('./layout-out.xlsx', import.meta.url).pathname
  wb.save(outPath)

  const bytes = readFileSync(outPath)
  assert.equal(bytes[0], 0x50) // 'P'
  assert.equal(bytes[1], 0x4b) // 'K'

  // The core properties part should carry the title.
  const core = zipRead(outPath, 'docProps/core.xml')
  assert.match(core, /Quarterly Report/)

  // The worksheet XML should carry an autoFilter and pageSetup/landscape.
  const sheetXml = zipRead(outPath, 'xl/worksheets/sheet1.xml')
  assert.match(sheetXml, /autoFilter/)
  assert.match(sheetXml, /landscape/)

  rmSync(outPath)
})
