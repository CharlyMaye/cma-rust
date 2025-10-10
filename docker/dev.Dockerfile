FROM ubuntu:24.04
ENV DEBIAN_FRONTEND=noninteractive

# Install all development dependencies for:
# - loggerd (basic Rust + SSL)
# - translation-lib (basic Rust + SSL)
# - waydash (Wayland/X11 + OpenGL/EGL)
# - traces (basic Rust)
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    curl \
    git \
    pkg-config \
    ca-certificates \
    # Core libraries
    libssl-dev \
    libclang-dev \
    clang \
    cmake \
    # Wayland/X11 support for waydash
    libwayland-dev \
    libwayland-client0 \
    libwayland-cursor0 \
    libwayland-egl1 \
    wayland-protocols \
    libxkbcommon-dev \
    libxkbcommon0 \
    libx11-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    # OpenGL/EGL for graphics
    libgl1-mesa-dev \
    libgl1 \
    libegl1-mesa-dev \
    libegl1 \
    # System integration
    libudev-dev \
    libdbus-1-dev \
    # Locale support
    locales \
    # Development tools
    vim \
    less \
    unzip \
    zip \
    tzdata \
    && rm -rf /var/lib/apt/lists/*

# Configure locale (fixes xkbcommon warnings)
RUN locale-gen en_US.UTF-8
ENV LANG=en_US.UTF-8 \
    LANGUAGE=en_US:en \
    LC_ALL=en_US.UTF-8

# Optional: Force software rendering if GPU not available
# ENV LIBGL_ALWAYS_SOFTWARE=1

# Install Rust
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /work
