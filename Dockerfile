FROM rust:1.65 as builder
WORKDIR /usr/src/pos-rs
COPY Cargo.lock Cargo.toml ./
RUN mkdir src && \
    echo "fn main() {}" >> src/main.rs && \
    cargo build --release && \
    rm -rf src
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt update && apt install -y build-essential
COPY --from=builder /usr/src/pos-rs/target/release/pos-rs /usr/local/bin/pos-rs
COPY ./migrations ./migrations
EXPOSE 80
ENV PORT=80
CMD ["pos-rs"]
