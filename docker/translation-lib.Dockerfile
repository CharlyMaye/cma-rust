# docker/translation-lib.Dockerfile
FROM ubuntu:24.04 AS builder
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential curl git pkg-config ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

RUN curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /src
COPY Cargo.toml Cargo.toml
COPY translation-lib/Cargo.toml translation-lib/Cargo.toml
RUN mkdir -p translation-lib/src && echo "pub fn hello()->&'static str{\"hello\"}" > translation-lib/src/lib.rs && \
    cargo test -p translation-lib || true

COPY . .
RUN cargo test -p translation-lib && cargo build -p translation-lib --release

FROM ubuntu:24.04
WORKDIR /app

CMD ["bash","-lc","echo 'translation-lib built' && ls -la /app || true"]
