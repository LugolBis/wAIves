#!/bin/bash

cat docs/index.html > index.html
rm docs/index.html
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

# Update the 'index.html' withe the new names of the .JS and .WASM files
folder="docs/assets"
target_file="index.html"

wasm_file=$(find "$folder" -maxdepth 1 -type f -name "*.wasm" | head -n 1)
js_file=$(find "$folder" -maxdepth 1 -type f -name "*.js" | head -n 1)

if [ -z "$wasm_file" ] || [ -z "$js_file" ]; then
    echo "Erreur: Fichiers .wasm ou .js introuvables dans '$folder'"
    exit 1
fi

wasm_name=$(basename "$wasm_file")
js_name=$(basename "$js_file")

escape_sed() {
    echo "$1" | sed -e 's/[\/&]/\\&/g'
}

escaped_wasm=$(escape_sed "$wasm_name")
escaped_js=$(escape_sed "$js_name")

sed -i -E \
-e "s/\/assets\/[^\"]+\.wasm/\/assets\/$escaped_wasm/g" \
-e "s/\/assets\/[^\"]+\.js/\/assets\/$escaped_js/g" \
"$target_file"

echo "WASM file detected : $escaped_wasm"
echo "JS file detected : $escaped_js"

rm docs/index.html
cat index.html > docs/index.html
rmdir app/docs/public
rmdir app/docs
