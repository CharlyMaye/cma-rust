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
    libssl-dev \        # OpenSSL development headers for HTTPS/TLS
    libclang-dev \      # Clang development headers for bindgen
    clang \             # Clang compiler for native dependencies
    cmake \             # CMake build system for C/C++ components
    
    # Wayland/X11 development support for waydash GUI
    libwayland-dev \        # Wayland protocol development headers
    libwayland-client0 \    # Wayland client runtime library
    libwayland-cursor0 \    # Wayland cursor support
    libwayland-egl1 \       # Wayland EGL integration
    wayland-protocols \     # Standard Wayland protocol definitions
    libxkbcommon-dev \      # Keyboard layout development headers
    libxkbcommon0 \         # Keyboard layout runtime library
    libx11-dev \            # X11 development headers (fallback support)
    libxcursor-dev \        # X11 cursor management
    libxrandr-dev \         # X11 display configuration
    libxi-dev \             # X11 input extension
    
    # OpenGL/EGL graphics development for waydash rendering
    libgl1-mesa-dev \       # Mesa OpenGL development headers
    libgl1 \                # Mesa OpenGL runtime library
    libegl1-mesa-dev \      # EGL development headers for Wayland
    libegl1 \               # EGL runtime library
    
    # System integration libraries
    libudev-dev \           # Device management development headers
    libdbus-1-dev \         # D-Bus IPC development headers
    
    # Internationalization and locale support
    locales \               # Locale data for proper text rendering
    
    # Development and debugging tools
    vim \                   # Text editor for container development
    less \                  # Pager for log viewing
    unzip \                 # Archive extraction utility
    zip \                   # Archive creation utility
    tzdata \                # Timezone data for accurate timestamps
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
# Mount workspace source code here: -v $(pwd):/work
WORKDIR /work

# Default command: Interactive shell for development
# Allows developers to run cargo commands, edit code, and test applications
# CMD ["/bin/bash"]
