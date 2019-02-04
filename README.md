# Baby's first Rust

## Install

(`rustup self uninstall`)\
`curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly`\
`source $HOME/.cargo/env`\
`rustup component add rust-src`\
(`rustup component add rustfmt`)\
`cargo install cargo-xbuild`\
(`cargo install bindgen`)

## Build
(`bindgen ../../include/rmu_defs.h > ./src/rmu_defs.rs`)\
(`cargo clean`)\
`cargo xbuild --target=aarch64-unknown-none --release` (`--verbose`)

## Run

./run.sh ${MODEL_BINARY}

## TODO

Everything else.