FROM ubuntu:24.04 AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential curl git pkg-config ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /src
COPY Cargo.toml Cargo.toml
COPY loggerd/Cargo.toml loggerd/Cargo.toml
RUN mkdir -p loggerd/src && echo "fn main(){}" > loggerd/src/main.rs && cargo build -p loggerd --release || true
COPY . .
RUN cargo build -p loggerd --release

FROM ubuntu:24.04
RUN useradd -m appuser
WORKDIR /app
COPY --from=builder /src/target/release/loggerd /usr/local/bin/loggerd
USER appuser
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/loggerd"]
