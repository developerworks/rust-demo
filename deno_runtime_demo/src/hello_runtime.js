import lib from './lib.js';

globalThis.rustCallback = function rustCallback(data) {
  console.log("Receive rust message:", data)
};

globalThis.add = lib.add;
globalThis.base64 = lib.base64;