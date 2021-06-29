FROM rustlang/rust:nightly AS builder
WORKDIR /app

COPY . .

RUN cargo build --bin basedcast_api --release

FROM rust
WORKDIR /app

COPY --from=builder /app/target/release/basedcast_api .
COPY settings.toml .
ENTRYPOINT ["/app/basedcast_api"]