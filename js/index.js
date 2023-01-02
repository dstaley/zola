"use strict";
const { readFile } = require("node:fs/promises");
const { WASI } = require("wasi");
const { env } = require("node:process");
const { join } = require("node:path");

/**
 * 
 * @param {string} siteDir Path to Zola site, relative to the current working directory
 * @param {string} [baseUrl]
 */
module.exports = async function build(siteDir = '.', baseUrl) {
  let args = ["zola", "build"];
  if (baseUrl) {
    args = [...args, "--base-url", baseUrl];
  }
  const wasi = new WASI({
    args,
    env,
    preopens: {
      "/": join(process.cwd(), siteDir),
    },
  });
  const importObject = { wasi_snapshot_preview1: wasi.wasiImport };
  const wasm = await WebAssembly.compile(
    await readFile(join(__dirname, "zola.wasm"))
  );
  const instance = await WebAssembly.instantiate(wasm, importObject);

  wasi.start(instance);
};
