set dotenv-load := true

default: list

list:
    just --list

reedme:
    cargo reedme --workspace

reedme-check:
    cargo reedme --workspace --check

test *args:
    cargo nextest run --workspace {{ args }} < /dev/null

doc-tests *args:
    cargo test --workspace --doc {{ args }}

clippy:
    cargo clippy --workspace --all-features --all-targets --keep-going -- -D warnings --allow deprecated

docs:
    cargo doc --workspace --all-features --no-deps --document-private-items --keep-going

lockfile:
    cargo update --workspace --locked

msrv:
    cargo hack check --rust-version --workspace --locked --ignore-private --keep-going
    cargo hack check --rust-version --workspace --locked --ignore-private --keep-going --all-features

miri-json:
    #!/usr/bin/env -S bash -euo pipefail
    export RUSTUP_TOOLCHAIN=nightly
    export MIRIFLAGS="-Zmiri-strict-provenance -Zmiri-env-forward=NEXTEST"
    rustup toolchain install "${RUSTUP_TOOLCHAIN}"
    rustup "+${RUSTUP_TOOLCHAIN}" component add miri rust-src llvm-tools-preview
    cargo miri nextest run --target-dir target/miri -p facet-json -E 'test(/scanner::tests|format_suite.*(pointer|proxy|collection::vec_nested|collection::map|struct::nested|enum::untagged|flatten::optional_some)|issue_2020|bstr.*invalid_utf8|format_specific_proxy.*roundtrip|raw_json::(deserialize_raw_json_owned|round_trip_raw_json_complex|skip_peeked_object_value)|list_deferred_processing::(nested_vec|deeply_nested)/)'
