# =============================================================================
# Container for translation-lib I18n Library Development and Testing
# =============================================================================
# Multi-stage build for translation library:
# - Builder stage: Compiles library and examples
# - Runtime stage: Testing environment with examples
#
# Usage:
#   docker build -f docker/translation-lib.Dockerfile -t translation-lib .
#   docker run -it translation-lib
#
# Purpose: Development and testing environment for i18n library
# =============================================================================

# Builder stage: Compile translation library and examples
FROM ubuntu:24.04 AS builder
ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies for Rust and SSL support
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential curl git pkg-config ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Install Rust toolchain
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /src

# Copy workspace configuration for dependency management
COPY Cargo.toml Cargo.toml
COPY translation-lib/Cargo.toml translation-lib/Cargo.toml

# Pre-compile dependencies with dummy source (Docker layer caching)
RUN mkdir -p translation-lib/src && echo "pub fn hello()->&'static str{\"hello\"}" > translation-lib/src/lib.rs && \
    cargo test -p translation-lib || true

# Copy source code and run comprehensive tests and build
COPY . .
RUN cargo test -p translation-lib && cargo build -p translation-lib --release

# Runtime stage: Testing and development environment
FROM ubuntu:24.04

WORKDIR /app

# Copy built library artifacts from builder stage
COPY --from=builder /src/target/release/deps/libtranslation_lib*.rlib /app/ || true

# Default command: Display build success and available artifacts
CMD ["bash","-lc","echo 'translation-lib built successfully' && echo 'Available library artifacts:' && ls -la /app/ || true"]
