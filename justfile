fmt: 
    cargo +nightly fmt --all

check:
    cargo clippy --all-targets --all-features -- -D warnings

fix: fmt
    git add ./
    cargo clippy --fix --all-targets --all-features --allow-staged
    
test:
    cargo test --all-features

cov:
    cargo llvm-cov --lcov --output-path {{justfile_directory()}}/target/llvm-cov-target/coverage.lcov

# Cross-compilation setup
setup-targets:
    rustup target add x86_64-unknown-linux-gnu
    rustup target add x86_64-pc-windows-gnu

# Build for Linux (glibc)
build-linux:
    cargo build --target x86_64-unknown-linux-gnu --release

# Build for Windows (GNU toolchain)
build-windows-gnu:
    cargo build --target x86_64-pc-windows-gnu --release


# Build for all targets
build-all: build-linux build-windows-gnu

# Cross-compile release builds for distribution
release-all: setup-targets build-all
    @echo "Release builds completed:"
    @echo "Linux (glibc): target/x86_64-unknown-linux-gnu/release/raindrop-mcp-server"
    @echo "Windows (GNU): target/x86_64-pc-windows-gnu/release/raindrop-mcp-server.exe"
