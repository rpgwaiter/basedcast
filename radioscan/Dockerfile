# FROM lukemathwalker/cargo-chef as planner
# WORKDIR /app
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

# FROM lukemathwalker/cargo-chef as cacher
# WORKDIR /app
# COPY --from=planner /app/recipe.json recipe.json
# RUN cargo chef cook --release --recipe-path recipe.json

# FROM rustlang/rust:nightly as builder
# WORKDIR /app
# COPY . .
# # Copy over the cached dependencies
# COPY --from=cacher /app/target target
# COPY --from=cacher $CARGO_HOME $CARGO_HOME
# RUN rustup default nightly && rustup update && cargo build --release --bin radioscan

# FROM rustlang/rust:nightly-alpine as runtime
# WORKDIR /app
# COPY --from=builder /app/target/release/radioscan /usr/local/bin
# ENTRYPOINT ["/usr/local/bin/radioscan"]

FROM nixpkgs/nix-flakes:nixos-20.09 AS builder
WORKDIR /app

RUN nix-channel --add https://nixos.org/channels/nixpkgs-unstable nixpkgs
RUN nix-channel --update

COPY . .

RUN nix build .#radioscan

FROM rust
WORKDIR /app

COPY --from=builder /app/result/bin/radioscan .
COPY settings.toml .
ENTRYPOINT ["/app/radioscan"]