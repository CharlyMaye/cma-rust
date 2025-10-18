# =============================================================================
# Development Environment Container for cma-rust Workspace
# =============================================================================
# Comprehensive development container with all tools needed for Rust development:
# - Latest stable Rust toolchain with essential components  
# - Development tools: LSP, debugger, formatters, linters
# - System dependencies for all workspace projects
# - Wayland support for waydash GUI development
# 
# Usage:
#   docker build -f docker/dev.Dockerfile -t cma-dev .
#   docker run -it --rm -v $(pwd):/workspace cma-dev
#
# Purpose: Consistent development environment across different systems
# Base: Ubuntu 24.04 LTS for stability and package availability
# =============================================================================

FROM ubuntu:24.04
ENV DEBIAN_FRONTEND=noninteractive

# Install comprehensive development dependencies for all workspace projects:
# - loggerd: System logging daemon (Rust + SSL/HTTP)
# - translation-lib: I18n library (Rust + SSL for remote resources)
# - waydash: Wayland dashboard UI (Rust + Wayland/X11 + OpenGL/EGL)
# - libs-cma: Core library with tracing and reactive patterns
RUN apt-get update && apt-get install -y --no-install-recommends \
    # Essential build tools
    build-essential \
    curl \
    git \
    pkg-config \
    ca-certificates \
    # Core development libraries
    # OpenSSL development headers for HTTPS/TLS
    libssl-dev \
    # Clang development headers for bindgen
    libclang-dev \
    # Clang compiler for native dependencies
    clang \
    # CMake build system for C/C++ components
    cmake \

    # Wayland/X11 development support for waydash GUI
    # Wayland protocol development headers
    libwayland-dev \
    # Wayland client runtime library
    libwayland-client0 \
    # Wayland cursor support
    libwayland-cursor0 \
    # Wayland EGL integration
    libwayland-egl1 \
    # Standard Wayland protocol definitions
    wayland-protocols \
    # Keyboard layout development headers
    libxkbcommon-dev \
    # Keyboard layout runtime library
    libxkbcommon0 \
    # X11 development headers (fallback support)
    libx11-dev \
    # X11 cursor management
    libxcursor-dev \
    # X11 display configuration
    libxrandr-dev \
    # X11 input extension
    libxi-dev \

    # OpenGL/EGL graphics development for waydash rendering
    # Mesa OpenGL development headers
    libgl1-mesa-dev \
    # Mesa OpenGL runtime library
    libgl1 \
    # EGL development headers for Wayland
    libegl1-mesa-dev \
    # EGL runtime library
    libegl1 \
    
    # System integration libraries
    # Device management development headers
    libudev-dev \
    # D-Bus IPC development headers
    libdbus-1-dev \
    
    # Internationalization and locale support
    # Locale data for proper text rendering
    locales \
    
    # Development and debugging tools
    # Text editor for container development
    vim \
    # Pager for log viewing
    less \
    # Archive extraction utility
    unzip \
    # Archive creation utility
    zip \
    # Timezone data for accurate timestamps
    tzdata \
    && rm -rf /var/lib/apt/lists/*

# Configure UTF-8 locale for proper text rendering and xkbcommon compatibility
# Prevents keyboard layout warnings in Wayland applications
RUN locale-gen en_US.UTF-8
ENV LANG=en_US.UTF-8 \
    LANGUAGE=en_US:en \
    LC_ALL=en_US.UTF-8

# Optional fallback: Force software rendering if GPU acceleration unavailable
# Uncomment for headless environments or systems without proper GPU drivers
# ENV LIBGL_ALWAYS_SOFTWARE=1

# Install Node.js (latest LTS version) using NodeSource repository
# Provides Node.js and npm for JavaScript/TypeScript development
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - \
    && apt-get install -y nodejs

# Install Rust toolchain using official rustup installer
# Provides latest stable Rust with package manager (cargo)
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install essential Rust development components
# rustfmt: Code formatter for consistent style
# clippy: Advanced linter for code quality
# rust-src: Source code for better IDE integration
RUN rustup component add rustfmt clippy rust-src

# Set development working directory
# Mount workspace source code here: -v $(pwd):/workspace
WORKDIR /workspace

# Default command: Interactive shell for development
# Allows developers to run cargo commands, edit code, and test applications
# CMD ["/bin/bash"]
