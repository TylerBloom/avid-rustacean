#!/usr/bin/env bash

# Assembles all of the assets for the web TUI and puts them in the static directory for zola

pushd crates

# Compile the web TUI frontend
pushd frontend
trunk build --dist ../assets --filehash=false --release=true --no-sri=false index.html
popd

# Compress the WASM binary and move it into statics
pushd assets

tar -czf avid-rustacean-frontend_bg.wasm.gz avid-rustacean-frontend_bg.wasm

cp index.html ../../static/tui
cp avid-rustacean-frontend.js ../../static
cp avid-rustacean-frontend_bg.wasm ../../static
cp avid-rustacean-frontend_bg.wasm.gz ../../static

popd

# Copy data from content into asserts to be parsed into JSON
popd
pushd content

cp ./*.md ../crates/assets
cp pages/projects.md ../crates/assets
cp pages/about/index.md ../crates/assets/home.md

popd
pushd crates

# Compile and run then builder to generate the JSON
pushd builder
cargo run
popd

# Move generated JSON into statics
pushd assets

mv badge.json ../../static
mv home.json ../../static/tui

mv projects.json ../../static/tui
cp index.html ../../static/tui/projects

mv posts.json ../../static/tui
cp index.html ../../static/tui/blog

for file in $(ls ./*.json)
do
				mv $file ../../static/tui/posts
				DIR_NAME=$(basename -- "$file" .json)
				mkdir ../../static/tui/blog/$DIR_NAME
				cp index.html ../../static/tui/blog/$DIR_NAME
done

popd
