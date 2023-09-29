# Lists avalable options
default:
    @just -l

# Counts the number lines of code
loc:
    find src/ -name "*.rs" | xargs cat | wc -l

# Builds the project
build:
    cargo build --release

# Formats, lints, and tests the project
publish:
    cargo fmt
    cargo clippy -q -- -D warnings
    ! rg 'blocking_read' src/
    cargo test -q

# Runs the code and then shows resulting logs
debug: run log

# Runs the code with example data
run:
    cargo run -- @INDEX.json

# Installs into $HOME/.local/bin
install: build
    cp './target/release/luma' $HOME/.local/bin/

# Prints the logs
log:
    bat $HOME/.cache/luma.log
