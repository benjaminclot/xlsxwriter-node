// Pure-Node zip readers for inspecting generated .xlsx files in tests, so the
// suite needs no external `unzip` binary (which isn't present on Windows CI).
//
// xlsx files are standard zips with small entries (no zip64), stored either
// uncompressed (method 0) or deflated (method 8).

import { readFileSync } from 'node:fs'
import { inflateRawSync } from 'node:zlib'

const SIG_EOCD = 0x06054b50
const SIG_CENTRAL = 0x02014b50
const SIG_LOCAL = 0x04034b50

function findEocd(buf) {
  for (let i = buf.length - 22; i >= 0; i--) {
    if (buf.readUInt32LE(i) === SIG_EOCD) return i
  }
  throw new Error('End of central directory not found (not a zip?)')
}

function centralEntries(buf) {
  const eocd = findEocd(buf)
  const count = buf.readUInt16LE(eocd + 10)
  let off = buf.readUInt32LE(eocd + 16)
  const entries = []
  for (let i = 0; i < count; i++) {
    if (buf.readUInt32LE(off) !== SIG_CENTRAL) throw new Error('Bad central directory header')
    const method = buf.readUInt16LE(off + 10)
    const compSize = buf.readUInt32LE(off + 20)
    const nameLen = buf.readUInt16LE(off + 28)
    const extraLen = buf.readUInt16LE(off + 30)
    const commentLen = buf.readUInt16LE(off + 32)
    const localOff = buf.readUInt32LE(off + 42)
    const name = buf.toString('utf8', off + 46, off + 46 + nameLen)
    entries.push({ name, method, compSize, localOff })
    off += 46 + nameLen + extraLen + commentLen
  }
  return entries
}

/** Newline-joined list of entry names (drop-in for `unzip -Z1`). */
export function zipEntries(path) {
  return centralEntries(readFileSync(path))
    .map((e) => e.name)
    .join('\n')
}

/** UTF-8 text of a single entry (drop-in for `unzip -p <file> <entry>`). */
export function zipRead(path, entryName) {
  const buf = readFileSync(path)
  const entry = centralEntries(buf).find((e) => e.name === entryName)
  if (!entry) throw new Error(`Entry not found: ${entryName}`)
  const lh = entry.localOff
  if (buf.readUInt32LE(lh) !== SIG_LOCAL) throw new Error('Bad local file header')
  const nameLen = buf.readUInt16LE(lh + 26)
  const extraLen = buf.readUInt16LE(lh + 28)
  const start = lh + 30 + nameLen + extraLen
  const data = buf.subarray(start, start + entry.compSize)
  const out = entry.method === 0 ? data : inflateRawSync(data)
  return out.toString('utf8')
}
