FROM nixpkgs/nix-flakes:nixos-20.09 AS builder
WORKDIR /app

RUN nix-channel --add https://nixos.org/channels/nixpkgs-unstable nixpkgs
RUN nix-channel --update

COPY . .

RUN nix build .#basedcast_api

FROM rust
WORKDIR /app

COPY --from=builder /app/result/bin/basedcast_api .
COPY settings.toml .
ENTRYPOINT ["/app/basedcast_api"]