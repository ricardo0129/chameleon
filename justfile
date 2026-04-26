build:
    cargo clippy --all-features -- --deny=warnings
    cargo build
    cargo test
