default:
    @just -l

loc:
    find src/ -name "*.rs" | xargs cat | wc -l

build:
    cargo build

publish: build
    cargo clippy -- -D warnings
    cargo test
