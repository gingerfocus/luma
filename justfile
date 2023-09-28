default: # List avalable options
    @just -l

loc: # Count the lines of code in this project
    find src/ -name "*.rs" | xargs cat | wc -l

build: # build the project
    cargo build

publish: # format lint and test the project
    cargo fmt
    cargo clippy -q -- -D warnings
    cargo test -q

debug:
    RUST_LOG=debug cargo run -- @INDEX.json
    bat luma.log
