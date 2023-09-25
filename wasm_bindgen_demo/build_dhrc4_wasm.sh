#!/bin/bash

wasm-opt --target web dhrc.js -o dhrc.wasm -s WASM=1 -s EXPORTED_FUNCTIONS='["_gen_keys", "_secret", "_rc4encryption", "_rc4decryption"]'  
