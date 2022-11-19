FROM rust AS chef
WORKDIR /pos-rs
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /pos-rs/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
COPY --from=builder /pos-rs/target/release/pos-rs /usr/local/bin/pos-rs
COPY ./migrations ./migrations
EXPOSE 80
ENV PORT=80
CMD ["pos-rs"]
