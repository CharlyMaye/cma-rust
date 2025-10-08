FROM ubuntu:24.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential curl git pkg-config ca-certificates \
    libssl-dev libclang-dev clang cmake \
    libwayland-dev wayland-protocols libxkbcommon-dev libudev-dev libdbus-1-dev \
    vim less unzip zip tzdata && \
    rm -rf /var/lib/apt/lists/*
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /work
