.PHONY: help build test fmt clippy clean docker-build docker-test docker-run-loggerd docker-run-waydash ci-local

help: ## Display this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# ==============================================================================
# Local Rust Commands
# ==============================================================================

build: ## Compile all projects in debug mode
	cargo build --all

release: ## Compile all projects in release mode
	cargo build --release --all

test: ## Execute all tests
	cargo test --all --verbose

fmt: ## Check code formatting
	cargo fmt --all -- --check

fmt-fix: ## Automatically fix code formatting
	cargo fmt --all

clippy: ## Execute Clippy linter
	cargo clippy --all-targets --all-features -- -D warnings

clean: ## Clean build artifacts
	cargo clean

check: fmt clippy test ## Execute all checks (fmt, clippy, test)

# ==============================================================================
# Docker Commands
# ==============================================================================

docker-build: ## Build all Docker CI images (builder + runtime)
	@echo "🐳 Building all Docker CI stages..."
	docker build -f docker/ci.Dockerfile --target builder -t cma-rust-builder:latest .
	docker build -f docker/ci.Dockerfile --target loggerd-runtime -t cma-rust-loggerd:latest .
	docker build -f docker/ci.Dockerfile --target waydash-runtime -t cma-rust-waydash:latest .
	@echo "✅ All images built successfully"
	@docker images | grep cma-rust

docker-test: ## Execute tests in Docker (like ci-docker-only.yml)
	@echo "🐳 Building CI Docker image (all stages)..."
	docker build -f docker/ci.Dockerfile --target builder -t cma-rust-builder .
	@echo ""
	@echo "🧪 Running tests in Docker container..."
	docker build -f docker/ci.Dockerfile --target test -t cma-rust-test .
	docker run --rm cma-rust-test
	@echo ""
	@echo "✅ Tests passed in Docker environment"

docker-build-loggerd: ## Build only the loggerd runtime image
	@echo "🐳 Building loggerd runtime image..."
	docker build -f docker/ci.Dockerfile --target loggerd-runtime -t cma-rust-loggerd:latest .
	@echo "✅ loggerd image ready"

docker-build-waydash: ## Build only the waydash runtime image
	@echo "🐳 Building waydash runtime image..."
	docker build -f docker/ci.Dockerfile --target waydash-runtime -t cma-rust-waydash:latest .
	@echo "✅ waydash image ready"

docker-run-loggerd: docker-build-loggerd ## Execute loggerd in Docker
	docker run --rm -p 8080:8080 cma-rust-loggerd:latest

docker-run-waydash: docker-build-waydash ## Execute waydash in Docker (requires Wayland)
	docker run --rm -it \
		-e WAYLAND_DISPLAY=$$WAYLAND_DISPLAY \
		-e XDG_RUNTIME_DIR=$$XDG_RUNTIME_DIR \
		-v $$XDG_RUNTIME_DIR/$$WAYLAND_DISPLAY:$$XDG_RUNTIME_DIR/$$WAYLAND_DISPLAY \
		-v $$XDG_RUNTIME_DIR:$$XDG_RUNTIME_DIR \
		--net=host \
		cma-rust-waydash:latest

# ==============================================================================
# Local CI
# ==============================================================================

ci-local: ## Simulate hybrid CI pipeline locally (fast)
	@echo "🔄 Local CI Pipeline (Hybrid - like ci.yml)"
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@echo ""
	@echo "🔍 Checking formatting..."
	@cargo fmt --all -- --check
	@echo "✅ Formatting OK"
	@echo ""
	
	@echo "🔍 Running Clippy..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "✅ Clippy OK"
	@echo ""
	
	@echo "🔍 Running tests..."
	@cargo test --all --verbose
	@echo "✅ Tests OK"
	@echo ""
	
	@echo "🔍 Release build..."
	@cargo build --release --all
	@echo "✅ Build OK"
	@echo ""
	
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@echo "🎉 Local CI pipeline successful!"
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

ci-docker: docker-test ## Simulate Docker-Only CI pipeline (reproducible)
	@echo ""
	@echo "🐳 Docker-Only CI pipeline simulated successfully"
	@echo "Equivalent to workflow: ci-docker-only.yml"

# ==============================================================================
# Binary Management
# ==============================================================================

install: release ## Install binaries in ~/.cargo/bin
	cargo install --path loggerd
	cargo install --path waydash

run-loggerd: ## Execute loggerd in debug mode
	cargo run --package loggerd

run-waydash: ## Execute waydash in debug mode
	cargo run --bin waydash

test-loggerd: ## Execute automatic test script for loggerd
	@./loggerd/test.sh

health-check: ## Quick test of /health endpoint
	@curl -s http://localhost:8080/health && echo ""

metrics: ## Display current metrics
	@curl -s http://localhost:8080/metrics | python3 -m json.tool 2>/dev/null || curl -s http://localhost:8080/metrics

stop-loggerd: ## Stop loggerd gracefully with SIGTERM
	@pkill -TERM -f "target/debug/loggerd" && echo "✅ SIGTERM sent" || echo "ℹ️  No loggerd process found"

# ==============================================================================
# Development Tools
# ==============================================================================

watch-loggerd: ## Execute loggerd with cargo-watch (auto-reload)
	cargo watch -x 'run --bin loggerd'

watch-waydash: ## Execute waydash with cargo-watch (auto-reload)
	cargo watch -x 'run --bin waydash'

doc: ## Generate and open documentation
	cargo doc --all --no-deps --open

tree: ## Display dependency tree
	cargo tree --all

outdated: ## Check outdated dependencies (requires cargo-outdated)
	cargo outdated

audit: ## Check vulnerabilities (requires cargo-audit)
	cargo audit
