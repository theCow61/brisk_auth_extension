#!/bin/sh

rm -rf pkg/
wasm-pack build . --dev --no-typescript --target web

cp manifest.json pkg/
cp index.html pkg/
cp popup.js pkg/
cp run_wasm.js pkg/
cp stores.js pkg/
# cp stores.js pkg/snippets/brisk_auth_extension-0d28f1cc4a65d82c/


