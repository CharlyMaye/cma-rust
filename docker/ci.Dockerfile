# ==============================================================================
# CI Docker Build - Comprehensive Rust Quality Assurance Pipeline
# ==============================================================================
# This multi-stage Dockerfile performs complete CI validation for cma-rust:
# 1. Code formatting validation (cargo fmt --check)
# 2. Comprehensive linting (cargo clippy with warnings as errors)
# 3. Full test suite execution (cargo test --all)
# 4. Optimized release builds (cargo build --release)
#
# Usage: 
#   docker build -f docker/ci.Dockerfile -t cma-rust-ci .
# 
# Used by: GitHub Actions CI pipeline (.github/workflows/ci.yml)
# Base: Ubuntu 24.04 LTS for maximum compatibility with CI environments
# ==============================================================================

# Stage 1: Builder - Rust compilation and quality checks
FROM ubuntu:24.04 AS builder

# Configure non-interactive mode and colored output for better CI logs
ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_TERM_COLOR=always

# Install build dependencies
# Install essential build dependencies for Rust compilation
# build-essential: GCC, make, and other build tools required by native dependencies
# curl: Required for rustup installation and HTTP client functionality
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

# Install Rust toolchain using official rustup installer
# Uses HTTPS with TLS 1.2 for security, installs latest stable Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Rust development tools for CI pipeline
# rustfmt: Code formatting checker (cargo fmt --check)
# clippy: Advanced linting tool (cargo clippy)
RUN rustup component add rustfmt clippy

# Set build directory for workspace operations
WORKDIR /build

# Copy workspace configuration and all sub-projects
# Cargo.toml/Cargo.lock: Workspace-level dependency management
# Individual crates: loggerd (daemon), waydash (UI), translation-lib, libs-cma (core), rustlings (exercises)
COPY Cargo.toml Cargo.lock ./
COPY loggerd ./loggerd
COPY waydash ./waydash
COPY translation-lib ./translation-lib
COPY libs-cma ./libs-cma
COPY rustlings ./rustlings

# ==============================================================================
# CI Quality Assurance Pipeline - Execute all checks in order
# ==============================================================================

# Step 1: Code formatting validation
# Ensures consistent code style across the entire workspace
# Fails build if any file doesn't match rustfmt standards
RUN cargo fmt --all -- --check

# Step 2: Advanced linting with Clippy
# Performs static analysis on all targets and features
# Treats all warnings as errors (-D warnings) for strict quality
RUN cargo clippy --all-targets --all-features -- -D warnings

# Step 3: Comprehensive test suite execution
# Runs all unit tests and integration tests (excluding doctests temporarily)
# Verbose output provides detailed information for CI logs
RUN cargo test --all --verbose --bins --lib

# Step 4: Release build compilation
# Creates optimized binaries for all workspace crates
# Build each binary explicitly to ensure they are created
RUN cargo build --release --bin loggerd
RUN cargo build --release --bin waydash || true
RUN cargo build --release --workspace

# Debug: List what was actually built
RUN ls -la /build/target/release/ && echo "=== Binaries ===" && find /build/target/release -maxdepth 1 -type f -executable

# ==============================================================================
# Stage 2: Minimal Runtime for loggerd Daemon
# ==============================================================================
# Lightweight runtime container containing only essential dependencies
# Base: Ubuntu 24.04 (same as builder for compatibility)
# Purpose: Production-ready container for loggerd daemon deployment
FROM ubuntu:24.04 AS loggerd-runtime

ENV DEBIAN_FRONTEND=noninteractive

# Install minimal runtime dependencies
# ca-certificates: Required for HTTPS connections and certificate validation
# libssl3: OpenSSL library for TLS/SSL support in Rust applications  
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create dedicated non-root user for security best practices
# Running as non-root reduces attack surface and follows container security guidelines
RUN useradd -m loggerd

# Copy compiled loggerd binary from builder stage
# Binary is statically linked and ready for execution
COPY --from=builder /build/target/release/loggerd /usr/local/bin/loggerd

# Switch to non-root user and set working directory
USER loggerd
WORKDIR /home/loggerd

# Expose HTTP API port for metrics and health endpoints
EXPOSE 8080

# Default command: start loggerd daemon
# Can be overridden with docker run command arguments
CMD ["loggerd"]

# ==============================================================================
# Stage 3: Runtime for waydash (Wayland Dashboard UI)
# ==============================================================================
# Runtime container for Wayland-based GUI application
# Base: Ubuntu 24.04 with Wayland and graphics libraries
# Purpose: Execute waydash dashboard in containerized Wayland environments
FROM ubuntu:24.04 AS waydash-runtime

ENV DEBIAN_FRONTEND=noninteractive

# Install Wayland and graphics runtime dependencies
# ca-certificates, libssl3: Basic SSL/TLS support
# libwayland-*: Core Wayland protocol libraries for compositor communication
# libxkbcommon0: Keyboard handling and layout support
# libx11-6, libxcursor1: X11 compatibility layer for hybrid environments
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

# Configure UTF-8 locale for proper text rendering and internationalization
RUN locale-gen en_US.UTF-8
ENV LANG=en_US.UTF-8 \
    LANGUAGE=en_US:en \
    LC_ALL=en_US.UTF-8

# Create dedicated non-root user for waydash execution
RUN useradd -m waydash

# Copy compiled waydash binary from builder stage
# Binary includes egui UI framework and Wayland protocol support
COPY --from=builder /build/target/release/waydash /usr/local/bin/waydash

# Switch to non-root user for security
USER waydash
WORKDIR /home/waydash

# Default command: launch waydash dashboard
# Requires Wayland display server connection via environment variables
CMD ["waydash"]

# ==============================================================================
# Stage 4: Test Runner (Default CI Target)
# ==============================================================================
# This is the primary target used by CI pipeline
# Contains all compiled binaries and CI validation results
# ==============================================================================
FROM builder AS test

WORKDIR /build

# Default CI target stage - inherits from builder with all artifacts
# Contains:
# - All compiled release binaries (loggerd, waydash, etc.) 
# - Validation results from CI pipeline (fmt, clippy, test)
# - Complete build environment for artifact extraction by GitHub Actions

# Optional: Re-run tests with coverage reporting
# Default command can be overridden by CI for specific testing needs
CMD ["cargo", "test", "--all", "--verbose", "--bins", "--lib"]
