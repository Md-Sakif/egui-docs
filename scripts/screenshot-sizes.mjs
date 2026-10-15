// Writes src/generated/screenshot-sizes.json: the pixel size of every example screenshot.
// The <Screenshot> component uses it to give each image its exact display size,
// so pages don't jump around while images load.
//
// Runs automatically before `npm start` and `npm run build`.
import {readdirSync, openSync, readSync, closeSync, mkdirSync, writeFileSync} from 'node:fs';
import {join, dirname} from 'node:path';
import {fileURLToPath} from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const dir = join(root, 'static', 'img', 'examples');
const sizes = {};

for (const file of readdirSync(dir).filter((f) => f.endsWith('.png')).sort()) {
  // A PNG stores its width and height as two big-endian integers at byte 16.
  const header = Buffer.alloc(24);
  const fd = openSync(join(dir, file), 'r');
  readSync(fd, header, 0, 24, 0);
  closeSync(fd);
  sizes[file] = [header.readUInt32BE(16), header.readUInt32BE(20)];
}

mkdirSync(join(root, 'src', 'generated'), {recursive: true});
writeFileSync(join(root, 'src', 'generated', 'screenshot-sizes.json'), JSON.stringify(sizes, null, 1) + '\n');
console.log(`screenshot-sizes: ${Object.keys(sizes).length} images`);
