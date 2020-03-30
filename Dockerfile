FROM rust:latest

# Add our source code.
ADD Cargo.toml .
ADD ./src/ ./src/

# Build our application.
RUN cargo build --release

CMD ["cargo", "run", "--release", "/html"]
