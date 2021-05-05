# 1: Build the exe
FROM rustlang/rust:nightly as builder
WORKDIR /usr/src

# 1a: Prepare for static linking
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

# 1b: Build Radioscan
RUN USER=root cargo new basedcast
WORKDIR /usr/src/basedcast
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --bin radioscan --release

# 2: Copy the bin to an empty Docker image
FROM scratch
COPY --from=builder /usr/src/basedcast/target/release/radioscan .
USER 1000
CMD ["./radioscan"]