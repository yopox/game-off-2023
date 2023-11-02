#!/usr/bin/env sh


# compress all wasm files in dist
find ./dist -name "*.wasm" -exec gzip -f -9 {} \;

sed -i "s@'/@'./@g; s@\"/@\"./@g" ./dist/index.html

# replace '.wasm' with '.wasm.gz'
sed -i "s@_bg.wasm@_bg.wasm.gz@g" ./dist/index.html

# replace 'init' with 'myInit'
sed -i "s@';init('@';myInit(init,'@g" ./dist/index.html