#!/bin/bash

mv docs/index.html index.html
rm -r docs
cd app
dx bundle --out-dir docs
cd ..
mkdir docs
mv app/docs/public/* docs
cp docs/index.html docs/404.html
cp app/src/tensorflow.js docs/tensorflow.js
cp app/src/style.css docs/style.css
cp app/assets/favicon.ico docs/favicon.ico
rm docs/index.html
mv index.html docs/index.html
rmdir app/docs/public
rmdir app/docs
