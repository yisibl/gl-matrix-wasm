#!/usr/bin/env node
/**
 * @File   : wasm-opt.js
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 2018/12/11 下午11:55:43
 * @Description:
 */
const binaryen = require("binaryen");
const path = require('path');
const fs = require('fs');

const package = require('./package.json');
const header = `/** 
 * @license gl-matrix-wasm v${package.version}
 * Copyright (c) 2018-present Tianyu Dai (dtysky)<dtysky@Outlook.com>.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
`;

let fp = path.resolve(__dirname, './pkg/gl_matrix_wasm_bg.wasm');
const originBuffer = fs.readFileSync(fp);

const wasm = binaryen.readBinary(originBuffer);
binaryen.setOptimizeLevel(0);
binaryen.setShrinkLevel(0);
wasm.optimize();

const wast = wasm.emitText()
  // .replace(/\(br_if \$label\$1[\s\n]+?\(i32.eq\n[\s\S\n]+?i32.const -1\)[\s\n]+\)[\s\n]+\)/g, '');
  .replace(/\(br_if \$label\$\d\n\s+\(i32\.eq\n\s+\(tee_local \$\d+?\n\s+\(i32\.load\n\s+\(get_local \$\d\)\n\s+\)\n\s+\)\n\s+\(i32\.const -1\)\n\s+\)\n\s+\)/g, '')
  .replace(/\(func \$\S+?_elements.+?\(type \$1\) \(param \$0 i32\) \(param \$1 i32\)[\s\S\n]+?\n  \(unreachable\)\n\s*\)/g, '')
  .replace(/\(export "\S+_elements" \(func \S+\)\)/g, '')
fs.writeFileSync(fp.replace('.wasm', '.wast'), wast);

const distBuffer = binaryen.parseText(wast).emitBinary();
fs.writeFileSync(fp, distBuffer);

fp = path.resolve(__dirname, './pkg/gl_matrix_wasm.d.ts');
fs.writeFileSync(fp, header + fs.readFileSync(fp, {encoding: 'utf8'})
  .replace(/get elements\(\)/g, 'readonly elements')
  .replace(
    'export default function init (module_or_path: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;',
    'export function init (): Promise<any>;'
  )
  .replace(/(@param {\S+?} out[\s\S\n]+?@returns {void} \n\*\/)\n\s+static (\S+?out[\s\S]+?): void;/g, (_, comm, funh) => {
    const type = /@param {(\S+)} out/.exec(comm)[1];
    
    return `${comm.replace('void', type)}
    static ${funh}: ${type};
    `
  })
);
fp = path.resolve(__dirname, './pkg/gl_matrix_wasm.js');

const offsets = {
  Matrix2: 4,
  Matrix2d: 6,
  Matrix3: 9,
  Matrix4: 16,
  Vector2: 2,
  Vector3: 3,
  Vector4: 4,
  Quaternion: 4,
  Quaternion2: 8
};
const content = fs.readFileSync(fp, {encoding: 'utf8'})
  .replace(
    /get elements\(\) {[\s\S\n]+?}[\s\S\n]+?@returns {(\S+)}/g,
    (_, type) => `get elements() {
      const ptr = this.ptr / 4 + 1;
      return new Float32Array(wasm.memory.buffer).slice(ptr, ptr + ${offsets[type]});
    }
    /**
     * @returns {$1}
  }`
  )
  .replace('function init(module) {', 'function initModule(module) {')
  .replace('export default init;', '')
  .replace(/(@param {\S+?} out[\s\S\n]+?@returns {void}\n\s+\*\/)\n\s+static (\S+?\(out[\s\S]+?){\n\s+([\s\S\n]+?)}/g, (_, comm, funh, funb) => {
    const type = /@param {(\S+)} out/.exec(comm)[1];
    
    return `${comm.replace('void', type)}
    static ${funh} {
      ${funb.replace('return ', '')}return out;
    }
    `
  });

fs.writeFileSync(fp, header + content + `
export async function init() {
  return initModule(new Uint8Array([${distBuffer.join(',')}]));
}
  `
);
fs.writeFileSync(
  fp.replace('.js', '.split.js'),
  header + `import * as wasm from './gl_matrix_wasm_bg';` + content.replace('let wasm;', '')
);
fp = path.resolve(__dirname, './pkg/gl_matrix_wasm_bg.d.ts');
fs.writeFileSync(fp, header + fs.readFileSync(fp, {encoding: 'utf8'}));
