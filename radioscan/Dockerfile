FROM rustlang/rust:nightly AS builder
WORKDIR /app

COPY . .

RUN cargo build --bin radioscan --release

FROM rust
WORKDIR /app

COPY --from=builder /app/target/release/radioscan .
COPY settings.toml .
ENTRYPOINT ["/app/radioscan"]