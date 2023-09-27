default:
    @just -l

loc:
    find src/ -name "*.rs" | xargs cat | wc -l

build:
    cargo build -q

publish:
    cargo clippy -q -- -D warnings
    cargo test -q

debug:
    RUST_LOG=debug cargo run -- @INDEX.json
    bat luma.log
