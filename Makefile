.PHONY: help build test fmt clippy clean docker-build docker-test docker-run-loggerd docker-run-waydash ci-local

help: ## Affiche cette aide
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# ==============================================================================
# Commandes Rust locales
# ==============================================================================

build: ## Compile tous les projets en mode debug
	cargo build --all

release: ## Compile tous les projets en mode release
	cargo build --release --all

test: ## Execute tous les tests
	cargo test --all --verbose

fmt: ## Vérifie le formatage du code
	cargo fmt --all -- --check

fmt-fix: ## Corrige automatiquement le formatage
	cargo fmt --all

clippy: ## Execute le linter Clippy
	cargo clippy --all-targets --all-features -- -D warnings

clean: ## Nettoie les artefacts de build
	cargo clean

check: fmt clippy test ## Execute toutes les vérifications (fmt, clippy, test)

# ==============================================================================
# Commandes Docker
# ==============================================================================

docker-build: ## Build toutes les images Docker CI (builder + runtime)
	@echo "🐳 Building all Docker CI stages..."
	docker build -f docker/ci.Dockerfile --target builder -t cma-rust-builder:latest .
	docker build -f docker/ci.Dockerfile --target loggerd-runtime -t cma-rust-loggerd:latest .
	docker build -f docker/ci.Dockerfile --target waydash-runtime -t cma-rust-waydash:latest .
	@echo "✅ All images built successfully"
	@docker images | grep cma-rust

docker-test: ## Execute les tests dans Docker (comme ci-docker-only.yml)
	@echo "🐳 Building CI Docker image (all stages)..."
	docker build -f docker/ci.Dockerfile --target builder -t cma-rust-builder .
	@echo ""
	@echo "🧪 Running tests in Docker container..."
	docker build -f docker/ci.Dockerfile --target test -t cma-rust-test .
	docker run --rm cma-rust-test
	@echo ""
	@echo "✅ Tests passed in Docker environment"

docker-build-loggerd: ## Build seulement l'image runtime loggerd
	@echo "🐳 Building loggerd runtime image..."
	docker build -f docker/ci.Dockerfile --target loggerd-runtime -t cma-rust-loggerd:latest .
	@echo "✅ loggerd image ready"

docker-build-waydash: ## Build seulement l'image runtime waydash
	@echo "🐳 Building waydash runtime image..."
	docker build -f docker/ci.Dockerfile --target waydash-runtime -t cma-rust-waydash:latest .
	@echo "✅ waydash image ready"

docker-run-loggerd: docker-build-loggerd ## Execute loggerd dans Docker
	docker run --rm -p 8080:8080 cma-rust-loggerd:latest

docker-run-waydash: docker-build-waydash ## Execute waydash dans Docker (nécessite Wayland)
	docker run --rm -it \
		-e WAYLAND_DISPLAY=$$WAYLAND_DISPLAY \
		-e XDG_RUNTIME_DIR=$$XDG_RUNTIME_DIR \
		-v $$XDG_RUNTIME_DIR/$$WAYLAND_DISPLAY:$$XDG_RUNTIME_DIR/$$WAYLAND_DISPLAY \
		-v $$XDG_RUNTIME_DIR:$$XDG_RUNTIME_DIR \
		--net=host \
		cma-rust-waydash:latest

# ==============================================================================
# CI local
# ==============================================================================

ci-local: ## Simule le pipeline CI hybride en local (rapide)
	@echo "� Pipeline CI Local (Hybride - comme ci.yml)"
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@echo ""
	@echo "�🔍 Vérification du formatage..."
	@cargo fmt --all -- --check
	@echo "✅ Formatage OK"
	@echo ""
	
	@echo "🔍 Exécution de Clippy..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "✅ Clippy OK"
	@echo ""
	
	@echo "🔍 Exécution des tests..."
	@cargo test --all --verbose
	@echo "✅ Tests OK"
	@echo ""
	
	@echo "🔍 Build release..."
	@cargo build --release --all
	@echo "✅ Build OK"
	@echo ""
	
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@echo "🎉 Pipeline CI local réussi !"
	@echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

ci-docker: docker-test ## Simule le pipeline CI Docker-Only (reproductible)
	@echo ""
	@echo "🐳 Pipeline CI Docker-Only simulé avec succès"
	@echo "Équivalent au workflow: ci-docker-only.yml"

# ==============================================================================
# Gestion des binaires
# ==============================================================================

install: release ## Installe les binaires dans ~/.cargo/bin
	cargo install --path loggerd
	cargo install --path waydash

run-loggerd: ## Execute loggerd en mode debug
	cargo run --bin loggerd

run-waydash: ## Execute waydash en mode debug
	cargo run --bin waydash

# ==============================================================================
# Outils de développement
# ==============================================================================

watch-loggerd: ## Execute loggerd avec cargo-watch (auto-reload)
	cargo watch -x 'run --bin loggerd'

watch-waydash: ## Execute waydash avec cargo-watch (auto-reload)
	cargo watch -x 'run --bin waydash'

doc: ## Génère et ouvre la documentation
	cargo doc --all --no-deps --open

tree: ## Affiche l'arbre des dépendances
	cargo tree --all

outdated: ## Vérifie les dépendances obsolètes (nécessite cargo-outdated)
	cargo outdated

audit: ## Vérifie les vulnérabilités (nécessite cargo-audit)
	cargo audit
