# ─── backend ────────────────────────────────────────────────────────────────

check:
    cargo check

fmt:
    cargo fmt

lint:
    cargo clippy -- -D warnings

test:
    cargo test

dev:
    cargo run
