#!/bin/sh

# builds all wasm targets

cargo install drt-sc-meta

TARGET_DIR=$PWD/target

sc-meta all build --target-dir-all $TARGET_DIR --path ./contracts
