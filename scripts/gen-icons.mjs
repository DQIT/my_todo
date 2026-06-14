// 生成 MyToDo 图标（纯 Node，无依赖）。
// 画一个圆角强调色方块 + 居中白色圆点，输出多尺寸 PNG 与 ICO。
import { deflateSync } from "zlib";
import { writeFileSync } from "fs";
import { fileURLToPath } from "url";
import { dirname, join } from "path";

const OUT = join(dirname(fileURLToPath(import.meta.url)), "..", "src-tauri", "icons");

const ACCENT = [79, 140, 255]; // #4F8CFF
const ACCENT2 = [138, 107, 255]; // 渐变末端
const WHITE = [255, 255, 255];

function lerp(a, b, t) {
  return Math.round(a + (b - a) * t);
}

function renderRGBA(size) {
  const px = Buffer.alloc(size * size * 4);
  const r = size * 0.22; // 圆角半径
  const cx = size / 2;
  const cy = size / 2;
  const dotR = size * 0.16;
  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      const i = (y * size + x) * 4;
      // 圆角矩形遮罩
      const inset = size * 0.06;
      const minX = inset, maxX = size - inset, minY = inset, maxY = size - inset;
      let inside = x >= minX && x <= maxX && y >= minY && y <= maxY;
      // 四角圆角
      const corners = [
        [minX + r, minY + r], [maxX - r, minY + r],
        [minX + r, maxY - r], [maxX - r, maxY - r],
      ];
      if (inside) {
        const nearX = x < minX + r ? minX + r : x > maxX - r ? maxX - r : x;
        const nearY = y < minY + r ? minY + r : y > maxY - r ? maxY - r : y;
        if ((x < minX + r || x > maxX - r) && (y < minY + r || y > maxY - r)) {
          const c = corners.find((c) => Math.abs(c[0] - nearX) < 1 && Math.abs(c[1] - nearY) < 1);
          if (c) {
            const d = Math.hypot(x - c[0], y - c[1]);
            inside = d <= r;
          }
        }
      }
      if (!inside) {
        px[i] = px[i + 1] = px[i + 2] = px[i + 3] = 0;
        continue;
      }
      // 对角线渐变
      const t = (x + y) / (2 * size);
      px[i] = lerp(ACCENT[0], ACCENT2[0], t);
      px[i + 1] = lerp(ACCENT[1], ACCENT2[1], t);
      px[i + 2] = lerp(ACCENT[2], ACCENT2[2], t);
      px[i + 3] = 255;
      // 中心白点
      if (Math.hypot(x - cx, y - cy) <= dotR) {
        px[i] = WHITE[0]; px[i + 1] = WHITE[1]; px[i + 2] = WHITE[2]; px[i + 3] = 255;
      }
    }
  }
  return px;
}

function crc32(buf) {
  let c = ~0;
  for (let i = 0; i < buf.length; i++) {
    c ^= buf[i];
    for (let k = 0; k < 8; k++) c = (c >>> 1) ^ (0xedb88320 & -(c & 1));
  }
  return ~c >>> 0;
}

function chunk(type, data) {
  const len = Buffer.alloc(4);
  len.writeUInt32BE(data.length);
  const td = Buffer.concat([Buffer.from(type), data]);
  const crc = Buffer.alloc(4);
  crc.writeUInt32BE(crc32(td));
  return Buffer.concat([len, td, crc]);
}

function encodePNG(size, rgba) {
  const sig = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(size, 0);
  ihdr.writeUInt32BE(size, 4);
  ihdr[8] = 8; // bit depth
  ihdr[9] = 6; // RGBA
  // 加 filter byte (0) 每行
  const stride = size * 4;
  const raw = Buffer.alloc((stride + 1) * size);
  for (let y = 0; y < size; y++) {
    raw[y * (stride + 1)] = 0;
    rgba.copy(raw, y * (stride + 1) + 1, y * stride, y * stride + stride);
  }
  const idat = deflateSync(raw);
  return Buffer.concat([
    sig,
    chunk("IHDR", ihdr),
    chunk("IDAT", idat),
    chunk("IEND", Buffer.alloc(0)),
  ]);
}

const sizes = { "32x32.png": 32, "128x128.png": 128, "128x128@2x.png": 256, "icon.png": 512 };
const pngs = {};
for (const [name, size] of Object.entries(sizes)) {
  const png = encodePNG(size, renderRGBA(size));
  pngs[size] = png;
  writeFileSync(join(OUT, name), png);
}

// ICO：内嵌 PNG（Windows Vista+ 支持），含 16/32/48/256
const icoSizes = [16, 32, 48, 256];
const entries = [];
const blobs = [];
let offset = 6 + icoSizes.length * 16;
for (const s of icoSizes) {
  const png = encodePNG(s, renderRGBA(s));
  blobs.push(png);
  const e = Buffer.alloc(16);
  e[0] = s >= 256 ? 0 : s;
  e[1] = s >= 256 ? 0 : s;
  e[4] = 1; e[5] = 0; // color planes
  e.writeUInt16LE(32, 6); // bpp
  e.writeUInt32LE(png.length, 8);
  e.writeUInt32LE(offset, 12);
  offset += png.length;
  entries.push(e);
}
const header = Buffer.alloc(6);
header.writeUInt16LE(0, 0);
header.writeUInt16LE(1, 2); // type icon
header.writeUInt16LE(icoSizes.length, 4);
writeFileSync(join(OUT, "icon.ico"), Buffer.concat([header, ...entries, ...blobs]));

console.log("图标已生成到", OUT);
