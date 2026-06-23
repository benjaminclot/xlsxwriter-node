// Tiny dependency-free PNG generator used by the showcase to make real images
// (a colored background with text) instead of blank 1x1 pixels.
//
// Produces a 24-bit RGB PNG using Node's built-in zlib. Text is drawn with an
// embedded 5x7 bitmap font (uppercase letters, digits and a few symbols).

import zlib from 'node:zlib'

// --- CRC32 (for PNG chunks) -------------------------------------------------
const CRC_TABLE = new Uint32Array(256)
for (let n = 0; n < 256; n++) {
  let c = n
  for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1
  CRC_TABLE[n] = c >>> 0
}
function crc32(buf) {
  let c = 0xffffffff
  for (const b of buf) c = CRC_TABLE[(c ^ b) & 0xff] ^ (c >>> 8)
  return (c ^ 0xffffffff) >>> 0
}

function chunk(type, data) {
  const len = Buffer.alloc(4)
  len.writeUInt32BE(data.length, 0)
  const typeBuf = Buffer.from(type, 'ascii')
  const crc = Buffer.alloc(4)
  crc.writeUInt32BE(crc32(Buffer.concat([typeBuf, data])), 0)
  return Buffer.concat([len, typeBuf, data, crc])
}

function encodePng(width, height, rgb) {
  const sig = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10])
  const ihdr = Buffer.alloc(13)
  ihdr.writeUInt32BE(width, 0)
  ihdr.writeUInt32BE(height, 4)
  ihdr[8] = 8 // bit depth
  ihdr[9] = 2 // color type: truecolor RGB
  const stride = width * 3
  const raw = Buffer.alloc((stride + 1) * height)
  for (let y = 0; y < height; y++) {
    raw[y * (stride + 1)] = 0 // filter: none
    rgb.copy(raw, y * (stride + 1) + 1, y * stride, y * stride + stride)
  }
  return Buffer.concat([
    sig,
    chunk('IHDR', ihdr),
    chunk('IDAT', zlib.deflateSync(raw)),
    chunk('IEND', Buffer.alloc(0)),
  ])
}

// --- 5x7 bitmap font --------------------------------------------------------
// prettier-ignore
const FONT = {
  ' ': ['.....', '.....', '.....', '.....', '.....', '.....', '.....'],
  'A': ['.###.', '#...#', '#...#', '#####', '#...#', '#...#', '#...#'],
  'B': ['####.', '#...#', '#...#', '####.', '#...#', '#...#', '####.'],
  'C': ['.###.', '#...#', '#....', '#....', '#....', '#...#', '.###.'],
  'D': ['###..', '#..#.', '#...#', '#...#', '#...#', '#..#.', '###..'],
  'E': ['#####', '#....', '#....', '####.', '#....', '#....', '#####'],
  'F': ['#####', '#....', '#....', '####.', '#....', '#....', '#....'],
  'G': ['.###.', '#...#', '#....', '#.###', '#...#', '#...#', '.###.'],
  'H': ['#...#', '#...#', '#...#', '#####', '#...#', '#...#', '#...#'],
  'I': ['#####', '..#..', '..#..', '..#..', '..#..', '..#..', '#####'],
  'L': ['#....', '#....', '#....', '#....', '#....', '#....', '#####'],
  'M': ['#...#', '##.##', '#.#.#', '#...#', '#...#', '#...#', '#...#'],
  'N': ['#...#', '##..#', '#.#.#', '#.#.#', '#..##', '#...#', '#...#'],
  'O': ['.###.', '#...#', '#...#', '#...#', '#...#', '#...#', '.###.'],
  'P': ['####.', '#...#', '#...#', '####.', '#....', '#....', '#....'],
  'R': ['####.', '#...#', '#...#', '####.', '#.#..', '#..#.', '#...#'],
  'S': ['.####', '#....', '#....', '.###.', '....#', '....#', '####.'],
  'T': ['#####', '..#..', '..#..', '..#..', '..#..', '..#..', '..#..'],
  'U': ['#...#', '#...#', '#...#', '#...#', '#...#', '#...#', '.###.'],
  'X': ['#...#', '#...#', '.#.#.', '..#..', '.#.#.', '#...#', '#...#'],
}

const GLYPH_W = 5
const GLYPH_H = 7

/**
 * Render `text` as a PNG: a solid `bg` background with `fg` text, auto-sized to
 * the text. Colors are `[r, g, b]`. `scale` is the pixel size of each font dot.
 * Returns a Buffer (PNG bytes).
 */
export function labelPng(text, { bg = [31, 78, 120], fg = [255, 255, 255], scale = 8 } = {}) {
  text = text.toUpperCase()
  const pad = scale * 3
  const textW = text.length * (GLYPH_W + 1) * scale - scale
  const width = textW + pad * 2
  const height = GLYPH_H * scale + pad * 2

  const rgb = Buffer.alloc(width * height * 3)
  for (let i = 0; i < width * height; i++) {
    rgb[i * 3] = bg[0]
    rgb[i * 3 + 1] = bg[1]
    rgb[i * 3 + 2] = bg[2]
  }

  const put = (px, py) => {
    if (px < 0 || py < 0 || px >= width || py >= height) return
    const o = (py * width + px) * 3
    rgb[o] = fg[0]
    rgb[o + 1] = fg[1]
    rgb[o + 2] = fg[2]
  }

  let x0 = pad
  const y0 = pad
  for (const ch of text) {
    const glyph = FONT[ch] ?? FONT[' ']
    for (let gy = 0; gy < GLYPH_H; gy++) {
      for (let gx = 0; gx < GLYPH_W; gx++) {
        if (glyph[gy][gx] === '#') {
          for (let dy = 0; dy < scale; dy++) {
            for (let dx = 0; dx < scale; dx++) {
              put(x0 + gx * scale + dx, y0 + gy * scale + dy)
            }
          }
        }
      }
    }
    x0 += (GLYPH_W + 1) * scale
  }

  return encodePng(width, height, rgb)
}
