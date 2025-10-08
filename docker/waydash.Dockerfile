FROM ubuntu:24.04 AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential curl git pkg-config ca-certificates \
    libwayland-dev wayland-protocols libxkbcommon-dev libssl-dev && \
    rm -rf /var/lib/apt/lists/*
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /src
COPY Cargo.toml Cargo.toml
COPY waydash/Cargo.toml waydash/Cargo.toml
RUN mkdir -p waydash/src && echo "fn main(){}" > waydash/src/main.rs && cargo build -p waydash --release || true
COPY . .
RUN cargo build -p waydash --release

FROM ubuntu:24.04
RUN useradd -m appuser
USER appuser
WORKDIR /app
COPY --from=builder /src/target/release/waydash /usr/local/bin/waydash
ENTRYPOINT ["/usr/local/bin/waydash"]
