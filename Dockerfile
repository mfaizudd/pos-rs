FROM rust:1.63 as builder
WORKDIR /usr/src/pos-rs
COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir src
RUN echo "fn main() {}" >> src/main.rs
RUN cargo build --release
RUN rm -rf src
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update
RUN apt-get install -y build-essential
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/pos-rs/target/release/pos-rs /usr/local/bin/pos-rs
COPY ./migrations ./migrations
EXPOSE 80
CMD ["pos-rs"]
