#!/bin/sh

# cleans all wasm targets

cargo install drt-sc-meta

sc-meta all clean --path ./contracts
