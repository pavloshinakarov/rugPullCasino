cargo clean
cargo build-sbf
solana program dump HRnqTwBTdfe96XRXQmnAMWRUQLrYLGGPWRFeUP7xMiZv dump.so
sha256sum dump.so ./target/deploy/daily_contract.so
