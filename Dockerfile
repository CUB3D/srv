FROM rust:latest

WORKDIR /home/srv

COPY docker-files ./

ADD Cargo.toml ./
ADD ./src/ ./src/

RUN cargo build --release

CMD ["cargo", "run", "--release", "/html"]
