````markdown
# CI/CD Pipeline

## 🔧 Principle

**All CI runs in Docker via `docker/ci.Dockerfile`**

The Dockerfile performs:
1. `cargo fmt --check` → validates formatting
2. `cargo clippy -- -D warnings` → strict linting
3. `cargo test --all` → all tests
4. `cargo build --release` → compilation

If any step fails → build fails → CI red ❌

## 🚀 Triggered on

- Push to `main`, `develop`, `cma/**`
- Pull Requests to `main`, `develop`

## 📦 Results

- Binaries: `loggerd`, `waydash` (downloadable)
- Docker Images: `cma-rust-loggerd`, `cma-rust-waydash`

## 🧪 Test Locally

```bash
# With Docker (identical to CI)
docker build -f docker/ci.Dockerfile .

# Or with Make
make docker-build

# Without Docker (faster)
make ci-local
```

## ⚙️ Performance

- **First time**: ~8-10 min
- **With cache**: ~3-5 min

````
