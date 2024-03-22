#!/bin/sh

features(){
    echo ext_wasm
    echo wasm_simd
    echo chunk8
    echo chunk8simd
}

export RUSTFLAGS='-C target_feature=+simd128'
cargo \
    build \
    --target wasm32-unknown-unknown \
    --features $( features | tr '\n' , | sed 's/,$//' ) \
    --profile release-wasm
