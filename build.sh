#rustc -C link-args=-nostartfiles ./src/main.rs
cargo xbuild --target=aarch64-unknown-none --release --verbose

#aarch64-linux-gnu-objcopy -O binary ./target/aarch64-unknown-none/release/hwrs
