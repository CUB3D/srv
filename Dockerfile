FROM rust:latest

# Add our source code.
ADD Cargo.toml .
ADD ./src/ ./src/

# Build our application.
RUN cargo build --release

CMD ["RUST_LOG=info", "cargo", "run", "--release", "/html"]
