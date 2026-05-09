#!/bin/bash

# dx --version = 0.7.7

cd app 2>/dev/null
rm -R ../docs 2>/dev/null
dx bundle --out-dir ../docs
mv ../docs/public/* ../docs/
mkdir ../docs/assets
cp assets/favicon.ico ../docs/assets/favicon.ico
cp assets/style.css ../docs/assets/style.css
cp assets/tensorflow.js ../docs/assets/tensorflow.js
cp assets/loader.js ../docs/assets/loader.js
