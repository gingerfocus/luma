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
    cargo clippy -q -- -D warnings
    cargo fmt
    @just safety
    cargo test -q

# Runs the code and then shows resulting logs
debug: && log
    cargo run -- -l ~/dox/luma.json

# Runs the code with example data
run:
    cargo run -- ./luma.json

# Installs into $HOME/.local/bin
install: build
    cp './target/release/luma' $HOME/.local/bin/

# Prints the logs
log:
    bat $HOME/.cache/luma.log

# The functions must return true for safety to be garenteed
safety:
    ! rg " print" src
