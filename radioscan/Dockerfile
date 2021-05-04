# 1: Build the exe
FROM rustlang/rust:nightly AS builder
WORKDIR /usr/src

# 1a: Prepare for static linking
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

# 1b: Download and compile Rust dependencies (and store as a separate Docker layer)
RUN USER=root cargo new basedcast
WORKDIR /usr/src/basedcast
COPY Cargo.toml radioscan/
COPY ../Cargo.toml ../Cargo.lock ./
COPY ../core/Cargo.toml core/
COPY ../api/Cargo.toml api/
RUN cargo install --target x86_64-unknown-linux-musl --path . --bin radioscan

# 1c: Build the exe using the actual source code
COPY ../core/src core/src/
COPY ../api/src api/src/
COPY src radioscan/
COPY radioscan/src radioscan/src/
RUN cargo install --target x86_64-unknown-linux-musl --path . --bin radioscan

# 2: Copy the exe and extra files ("static") to an empty Docker image
FROM scratch
COPY --from=builder /usr/local/cargo/bin/radioscan .
COPY static .
USER 1000
CMD ["./radioscan"]