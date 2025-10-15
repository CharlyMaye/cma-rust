# ==============================================================================
# Stage 1: Builder - Compile all Rust projects
# ==============================================================================
FROM ubuntu:24.04 AS builder

ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_TERM_COLOR=always

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    git \
    pkg-config \
    ca-certificates \
    libssl-dev \
    libclang-dev \
    clang \
    cmake \
    libwayland-dev \
    wayland-protocols \
    libxkbcommon-dev \
    libx11-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    libgl1-mesa-dev \
    libegl1-mesa-dev \
    libudev-dev \
    libdbus-1-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

# Add rustfmt and clippy
RUN rustup component add rustfmt clippy

WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY loggerd ./loggerd
COPY waydash ./waydash
COPY translation-lib ./translation-lib
COPY libs-cma ./libs-cma
COPY rustlings ./rustlings

# Format check
RUN cargo fmt --all -- --check

# Clippy check
RUN cargo clippy --all-targets --all-features -- -D warnings

# Run tests
RUN cargo test --all --verbose

# Build release binaries
RUN cargo build --release --all

# ==============================================================================
# Stage 2: Runtime for loggerd (minimal)
# ==============================================================================
FROM ubuntu:24.04 AS loggerd-runtime

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m loggerd

COPY --from=builder /build/target/release/loggerd /usr/local/bin/loggerd

USER loggerd
WORKDIR /home/loggerd

EXPOSE 8080

CMD ["loggerd"]

# ==============================================================================
# Stage 3: Runtime for waydash (with Wayland support)
# ==============================================================================
FROM ubuntu:24.04 AS waydash-runtime

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libwayland-client0 \
    libwayland-cursor0 \
    libwayland-egl1 \
    libxkbcommon0 \
    libx11-6 \
    libxcursor1 \
    libxrandr2 \
    libxi6 \
    libgl1 \
    libegl1 \
    locales \
    && rm -rf /var/lib/apt/lists/*

# Configure locale
RUN locale-gen en_US.UTF-8
ENV LANG=en_US.UTF-8 \
    LANGUAGE=en_US:en \
    LC_ALL=en_US.UTF-8

# Create non-root user
RUN useradd -m waydash

COPY --from=builder /build/target/release/waydash /usr/local/bin/waydash

USER waydash
WORKDIR /home/waydash

CMD ["waydash"]

# ==============================================================================
# Stage 4: Test runner (for CI)
# ==============================================================================
FROM builder AS test

WORKDIR /build

# Re-run tests with coverage (optional)
CMD ["cargo", "test", "--all", "--verbose"]
