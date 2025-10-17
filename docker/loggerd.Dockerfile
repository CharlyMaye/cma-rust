# =============================================================================
# Production Container for loggerd System Logging Daemon
# =============================================================================
# Multi-stage build optimized for loggerd daemon deployment:
# - Builder stage: Compiles loggerd with all dependencies
# - Runtime stage: Minimal container with only required libraries
# 
# Usage:
#   docker build -f docker/loggerd.Dockerfile -t loggerd .
#   docker run -p 8080:8080 -v /var/log:/var/log loggerd
#
# Purpose: Production-ready logging daemon with HTTP API and file rotation
# =============================================================================

# Builder stage: Compile loggerd daemon
FROM ubuntu:24.04 AS builder

# Install build dependencies for Rust compilation and SSL support
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential curl git pkg-config ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Install Rust toolchain
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /src
# Copy workspace and project configuration for dependency caching
COPY Cargo.toml Cargo.toml
COPY loggerd/Cargo.toml loggerd/Cargo.toml

# Pre-compile dependencies (Docker layer caching optimization)
RUN mkdir -p loggerd/src && echo "fn main(){}" > loggerd/src/main.rs && cargo build -p loggerd --release || true

# Copy source code and compile final binary
COPY . .
RUN cargo build -p loggerd --release

# Runtime stage: Minimal production container
FROM ubuntu:24.04

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m appuser

WORKDIR /app

# Copy compiled binary from builder stage
COPY --from=builder /src/target/release/loggerd /usr/local/bin/loggerd

# Switch to non-root user
USER appuser

# Expose HTTP API port (metrics and health endpoints)
EXPOSE 8080

# Start loggerd daemon
ENTRYPOINT ["/usr/local/bin/loggerd"]
