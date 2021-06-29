FROM lukemathwalker/cargo-chef as planner
WORKDIR /app
RUN rustup update nightly && rustup default nightly && rm -rf $CARGO_HOME/registry/
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR /app
RUN rustup update nightly && rustup default nightly && rm -rf $CARGO_HOME/registry/
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as builder
WORKDIR /app
RUN rustup update nightly && rustup default nightly && rm -rf $CARGO_HOME/registry/
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin radioscan

FROM rust as runtime
RUN rustup update nightly && rustup default nightly && rm -rf $CARGO_HOME/registry/
WORKDIR /app
COPY --from=builder /app/target/release/radioscan /usr/local/bin
COPY settings.toml .
ENTRYPOINT ["./usr/local/bin/radioscan"]