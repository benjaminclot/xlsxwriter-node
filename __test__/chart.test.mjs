import { test } from 'node:test'
import assert from 'node:assert/strict'
import { readFileSync, existsSync, rmSync } from 'node:fs'
import { createRequire } from 'node:module'
import { zipEntries, zipRead } from './helpers.mjs'

const require = createRequire(import.meta.url)
const { Workbook, Chart, ChartType, ChartLegendPosition } = require('../index.js')

function withData(sheet) {
  // Categories in column A, two value series in B and C.
  const rows = [
    ['Q1', 10, 30],
    ['Q2', 40, 20],
    ['Q3', 50, 60],
    ['Q4', 20, 10],
  ]
  rows.forEach((r, i) => {
    sheet.write(i, 0, r[0])
    sheet.write(i, 1, r[1])
    sheet.write(i, 2, r[2])
  })
}

test('inserts a column chart with two series into the workbook', () => {
  const wb = new Workbook()
  const sheet = wb.addWorksheet('Sheet1')
  withData(sheet)

  const chart = new Chart(ChartType.Column)
  chart
    .addSeries({
      name: 'Sales',
      categories: 'Sheet1!$A$1:$A$4',
      values: 'Sheet1!$B$1:$B$4',
    })
    .addSeries({
      name: 'Costs',
      categories: 'Sheet1!$A$1:$A$4',
      values: 'Sheet1!$C$1:$C$4',
    })
    .setTitle('Quarterly Performance')
    .setXAxisName('Quarter')
    .setYAxisName('Amount')
    .setLegendPosition(ChartLegendPosition.Bottom)
    .setStyle(12)

  sheet.insertChart(5, 1, chart)

  const outPath = new URL('./chart-out.xlsx', import.meta.url).pathname
  if (existsSync(outPath)) rmSync(outPath)
  wb.save(outPath)

  // Valid .xlsx zip.
  const bytes = readFileSync(outPath)
  assert.equal(bytes[0], 0x50) // 'P'
  assert.equal(bytes[1], 0x4b) // 'K'

  // The chart part must exist and carry our configuration.
  const entries = zipEntries(outPath)
  assert.match(entries, /xl\/charts\/chart1\.xml/)

  const chartXml = zipRead(outPath, 'xl/charts/chart1.xml')
  assert.match(chartXml, /<c:barChart>/) // column chart -> barChart with col direction
  assert.match(chartXml, /Quarterly Performance/) // title text
  assert.match(chartXml, /Sheet1!\$B\$1:\$B\$4/) // first series values range
  assert.match(chartXml, /Sheet1!\$C\$1:\$C\$4/) // second series values range

  rmSync(outPath)
})

test('inserts a pie chart with a single series and hidden legend', () => {
  const wb = new Workbook()
  const sheet = wb.addWorksheet('Sheet1')
  withData(sheet)

  const chart = new Chart(ChartType.Pie)
  chart
    .addSeries({
      categories: 'Sheet1!$A$1:$A$4',
      values: 'Sheet1!$B$1:$B$4',
    })
    .setTitle('Share')
    .setLegendHidden()

  sheet.insertChart(5, 1, chart)

  const buf = wb.saveToBuffer()
  assert.ok(Buffer.isBuffer(buf))
  assert.equal(buf[0], 0x50)
  assert.equal(buf[1], 0x4b)
})
