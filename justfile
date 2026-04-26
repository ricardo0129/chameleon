build:
    cargo fmt --check
    cargo clippy --all-features -- --deny=warnings
    cargo build
    cargo test
