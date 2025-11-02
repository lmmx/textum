build:
    cargo build --workspace --all-features

check:
    # cargo check --workspace --all-features
    # cargo hack check --workspace --each-feature --no-dev-deps
    cargo hack check --workspace --feature-powerset --no-dev-deps

clippy:
    cargo clippy --workspace --all-targets --all-features --target-dir target/clippy-all-features -- -D warnings

clippy-fix:
    cargo clippy --fix --workspace --all-targets --all-features --target-dir target/clippy-all-features -- -D warnings

doctest *args:
    cargo test --doc {{args}} --all-features

test *args:
    # cargo nextest run {{args}} --all-targets --all-features
    # cargo hack nextest run --workspace --all-targets --each-feature
    cargo hack nextest run --workspace --all-targets --feature-powerset

test-ci *args:
    #!/usr/bin/env -S echo-comment --color bright-green
    # 🏃 Running Rust tests...
    cargo test {{args}}

    # 📚 Running documentation tests...
    cargo test --doc {{args}}
