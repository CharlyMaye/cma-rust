# =============================================================================
# Production Container for waydash Wayland System Dashboard
# =============================================================================
# Multi-stage build optimized for waydash GUI application:
# - Builder stage: Compiles waydash with Wayland/graphics dependencies
# - Runtime stage: Wayland-enabled container for GUI execution
# 
# Usage:
#   docker build -f docker/waydash.Dockerfile -t waydash .
#   docker run --rm -e WAYLAND_DISPLAY=$WAYLAND_DISPLAY \
#     -v $XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY \
#     waydash
#
# Purpose: Containerized Wayland dashboard for system metrics visualization
# =============================================================================

# Builder stage: Compile waydash with full Wayland/graphics support
FROM ubuntu:24.04 AS builder

# Install comprehensive build dependencies for Wayland GUI development
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \       # Core compilation tools
    curl \                  # For rustup installation
    git \                   # Version control (dependencies may need)
    pkg-config \            # Library configuration detection
    ca-certificates \       # SSL certificate validation
    
    # Wayland development libraries
    libwayland-dev \        # Core Wayland protocol development
    wayland-protocols \     # Standard protocol definitions
    libxkbcommon-dev \      # Keyboard layout support
    
    # SSL and networking
    libssl-dev \            # OpenSSL development headers
    
    # X11 fallback support
    libx11-dev \            # X11 development libraries
    libxcursor-dev \        # Cursor management
    libxrandr-dev \         # Display configuration
    libxi-dev \
    libgl1-mesa-dev \
    libegl1-mesa-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set up workspace
WORKDIR /src
COPY Cargo.toml Cargo.toml
COPY waydash/Cargo.toml waydash/Cargo.toml

# Cache dependencies
RUN mkdir -p waydash/src && \
    echo "fn main(){}" > waydash/src/main.rs && \
    cargo build -p waydash --release || true

# Build actual application
COPY . .
RUN cargo build -p waydash --release

# Runtime stage
FROM ubuntu:24.04

# Install runtime dependencies and configure locales
RUN apt-get update && apt-get install -y --no-install-recommends \
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

# Force software rendering to avoid GPU device requirements
ENV LIBGL_ALWAYS_SOFTWARE=1

# Create non-root user
RUN useradd -m appuser
USER appuser
WORKDIR /app

# Copy binary from builder
COPY --from=builder /src/target/release/waydash /usr/local/bin/waydash

ENTRYPOINT ["/usr/local/bin/waydash"]
