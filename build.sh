#rustc -C link-args=-nostartfiles ./src/main.rs
cargo xbuild --target=aarch64-unknown-none --release --verbose
