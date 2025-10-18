# CMA-Rust

[![CI](https://github.com/CharlyMaye/cma-rust/workflows/CI/badge.svg)](https://github.com/CharlyMaye/cma-rust/actions)

> 🦀 Rust Learning Journey — System, Wayland & Open Source

Complete low-level Rust learning project with construction of a system daemon (`loggerd`) and a Wayland dashboard (`waydash`).

## 📦 Workspace Projects

This workspace contains several Rust crates:

| Crate | Description | Status |
|-------|-------------|--------|
| **loggerd** | System logging daemon with rotation + REST API | ✅ Functional |
| **waydash** | Wayland dashboard to display metrics | 🚧 Planned |
| **translation-lib** | i18n library for Rust | 📦 To publish |
| **traces** | Custom logging library with Rx patterns | 🎓 In progress |
| **rustlings** | Rust exercises (learning) | 🎓 In progress |

## 🚀 Quick Start

### Prerequisites

```bash
# Rust installation
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt clippy

# On Ubuntu/Debian
sudo apt-get install -y \
  pkg-config libssl-dev libwayland-dev wayland-protocols \
  libxkbcommon-dev libx11-dev libxcursor-dev libxrandr-dev \
  libxi-dev libgl1-mesa-dev libegl1-mesa-dev \
  libudev-dev libdbus-1-dev
```

### Build & Run

```bash
# Build all projects
make build

# Run tests
make test

# Complete checks (format + clippy + test)
make check

# Release build
make release
```

### Run applications

```bash
# loggerd daemon (REST API on port 8080)
make run-loggerd

# Test endpoints
make health-check    # GET /health
make metrics         # GET /metrics (JSON)

# Stop loggerd gracefully
make stop-loggerd    # Send SIGTERM (graceful shutdown)

# waydash dashboard (Wayland graphical interface)
make run-waydash     # Coming soon (Week 4)
```

## 🔄 CI/CD

**🐳 CI runs entirely in Docker to ensure reproducibility.**

The GitHub Actions pipeline uses `docker/ci.Dockerfile` and automates:
- ✅ Format checking (`cargo fmt --check`)
- ✅ Linting with Clippy (strict mode: `-D warnings`)
- ✅ Unit and integration tests
- ✅ Optimized release build
- ✅ Runtime Docker images generation

### Test CI locally

```bash
# With Docker (identical to GitHub CI)
make docker-build

# Without Docker (faster)
make ci-local
```

**Execution time**:
- Docker (first time): ~8-10 min
- Docker (with cache): ~3-5 min  
- Local without Docker: ~2 min

## 🐳 Docker

### Build with Docker

```bash
# Build complete CI image (with tests)
make docker-build

# Build loggerd runtime image
make docker-build-loggerd

# Build waydash runtime image
make docker-build-waydash
```

### Run in Docker

```bash
# loggerd (REST API)
make docker-run-loggerd

# waydash (requires Wayland socket access)
make docker-run-waydash
```

## �️ Development Services

### MongoDB & Mongo Express

The development environment includes MongoDB with a web interface:

```bash
# Start all services (MongoDB + Mongo Express + Qdrant)
docker-compose -f docker/docker-compose.yml up -d

# Access Mongo Express (Web UI for MongoDB)
# URL: http://localhost:8081
# Username: admin
# Password: admin123
```

**MongoDB Connection:**
- Host: `localhost:27017`
- Database: `cma_dev`
- Username: `root`
- Password: `example`

### Qdrant Vector Database

Qdrant provides vector search capabilities with a built-in dashboard:

```bash
# Access Qdrant Dashboard
# URL: http://localhost:6333/dashboard

# Qdrant API endpoints
# HTTP API: http://localhost:6333
# gRPC API: localhost:6334
```

**Qdrant Usage:**
```bash
# Health check
curl http://localhost:6333/health

# List collections
curl http://localhost:6333/collections
```

### Stop Services

```bash
# Stop all development services
docker-compose -f docker/docker-compose.yml down

# Stop and remove volumes (clean reset)
docker-compose -f docker/docker-compose.yml down -v
```

## �📚 Documentation

- **[CI/CD](docs/CI_CD.md)**: Complete CI/CD pipeline documentation
- **[TODO](docs/todo.md)**: Project roadmap (10 weeks)

## 🧩 Architecture

```
+-------------------+          HTTP/JSON          +----------------------+
|   waydash (UI)    |  <----------------------->  |     loggerd (daemon) |
|  - egui / winit   |                            |  - Axum REST API      |
|  - xdg-shell       |                            |  - journald logs      |
|  - layer-shell*    |                            |  - prometheus metrics |
+-------------------+                            +----------------------+
         |                                                   ^
         | Wayland sockets                                   |
         v                                                   |
   Compositor (Sway, Hyprland, GNOME, KDE)                   |
         ^                                                   |
         +------------------ Linux system -------------------+
```

## 🛠️ Useful Commands

```bash
# Help (show all available commands)
make help

# Build & test
make build              # Debug build
make release            # Release build
make test               # Tests
make fmt                # Check formatting
make fmt-fix            # Fix formatting
make clippy             # Linter
make clean              # Clean

# Development
make watch-loggerd      # Auto-reload loggerd
make watch-waydash      # Auto-reload waydash
make doc                # Generate docs
make tree               # Dependency tree

# Installation
make install            # Install in ~/.cargo/bin
```

## 🎯 Project Objectives

### Week 1: Setup & CI ✅
- [x] Multi-stage Ubuntu 24.04 Dockerfile
- [x] Complete GitHub Actions pipeline
- [x] Makefile for automation
- [x] CI/CD documentation

### Week 2: Rust Daemon (loggerd) ✅
- [x] loggerd binary with HTTP API (Axum)
- [x] `/health` and `/metrics` endpoints
- [x] Custom trace system (console + file)
- [x] Automatic log rotation (by size + timestamp)
- [x] Unix signal handling (SIGTERM, SIGHUP)
- [x] Graceful shutdown
- [x] Thread-safe log counter (AtomicU64)
- [x] Modular refactoring (SRP, Clean Architecture)
- [x] Complete documentation with doctests
- [x] systemd unit file

**Documentation**:
- [Week 2 Summary](docs/semaine2-recap.md)
- [File Module Architecture](docs/architecture-file-module.md)
- [file_trace_handlers Refactoring](docs/refactoring-file-handler.md)

### Following Weeks
- [ ] Week 3: Ubuntu packaging (Snapcraft)
- [ ] Week 4: Wayland UI (xdg-shell)
- [ ] Week 5: Advanced UI (layer-shell)
- [ ] Week 6: Translation library publication
- [ ] Week 7: Observability & performance
- [ ] Week 8: Open source & article
- [ ] Week 9: Complete showcase project

See [docs/todo.md](docs/todo.md) for complete details.

## 🔧 Tech Stack

- **Language**: Rust (stable)
- **Build**: Cargo workspace
- **CI/CD**: GitHub Actions
- **Containers**: Docker multi-stage
- **Web**: Axum (REST API)
- **GUI**: egui + winit (Wayland)
- **Packaging**: Snap (Ubuntu)
- **System**: systemd, journald

## 📊 Métriques

![CI Status](https://img.shields.io/github/actions/workflow/status/CharlyMaye/cma-rust/ci.yml?branch=main)
![Rust Version](https://img.shields.io/badge/rust-stable-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## 🤝 Contributing

Contributions are welcome! This project is part of a learning journey, feel free to:
- Propose improvements
- Report bugs
- Share ideas

## 📝 License

MIT License - see [LICENSE](LICENSE) for more details.

## 👤 Author

**CharlyMaye**

- GitHub: [@CharlyMaye](https://github.com/CharlyMaye)
- Project: Rust/Linux/Wayland Journey

---

**Note**: This project is currently under active development as part of a 10-week learning journey on system Rust and Linux.
