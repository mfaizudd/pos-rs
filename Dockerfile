FROM rust:1.63 as builder
WORKDIR /usr/src/pos-rs
COPY Cargo.toml .
RUN mkdir src
RUN touch src/lib.rs
RUN echo "fn main() {}" >> dummy.rs
RUN sed -i "s#src/main.rs#dummy.rs#" Cargo.toml
RUN cargo build --release
RUN sed -i "s#dummy.rs#src/main.rs#" Cargo.toml
RUN rm dummy.rs src/lib.rs
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/pos-rs /usr/local/bin/pos-rs
EXPOSE 80
CMD ["pos-rs"]
