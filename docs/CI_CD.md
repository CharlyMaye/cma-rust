# CI/CD Pipeline Documentation

## 📋 Vue d'ensemble

Ce projet utilise GitHub Actions pour automatiser les vérifications de qualité de code, les tests et les builds.

## 🔄 Workflow CI

Le pipeline CI s'exécute automatiquement sur :
- **Push** vers `main`, `develop`, ou toute branche `cma/**`
- **Pull Requests** vers `main` ou `develop`

### Jobs du pipeline

1. **Format Check** 
   - Vérifie que le code est correctement formaté avec `cargo fmt`
   - Échoue si le formatage n'est pas conforme

2. **Clippy**
   - Execute le linter Rust avec des règles strictes (`-D warnings`)
   - Vérifie tous les targets et toutes les features

3. **Tests**
   - Execute tous les tests unitaires et d'intégration
   - Utilise le workspace complet

4. **Build Release**
   - Compile tous les binaires en mode `--release`
   - Génère des artifacts pour `x86_64-unknown-linux-gnu`
   - Upload les binaires `loggerd` et `waydash`

5. **Docker Build** (optionnel)
   - Build l'image Docker de développement
   - Utilise le cache GitHub Actions pour accélérer les builds

## 🚀 Utilisation locale

### Prérequis

```bash
# Installer les composants Rust
rustup component add rustfmt clippy

# Sur Ubuntu/Debian, installer les dépendances système
sudo apt-get install -y \
  pkg-config libssl-dev libwayland-dev wayland-protocols \
  libxkbcommon-dev libx11-dev libxcursor-dev libxrandr-dev \
  libxi-dev libgl1-mesa-dev libegl1-mesa-dev \
  libudev-dev libdbus-1-dev
```

### Commandes rapides

```bash
# Simuler le pipeline CI complet en local
make ci-local

# Vérifier uniquement le formatage
make fmt

# Corriger le formatage automatiquement
make fmt-fix

# Lancer Clippy
make clippy

# Lancer les tests
make test

# Build release
make release

# Toutes les vérifications (fmt + clippy + test)
make check
```

### Avec Docker

```bash
# Build l'image CI complète
make docker-build

# Tester dans Docker
make docker-test

# Build et run loggerd
make docker-run-loggerd

# Build et run waydash (nécessite Wayland)
make docker-run-waydash
```

## 📦 Artifacts

Les binaries compilés sont disponibles en artifacts sur chaque run réussi :
- `loggerd` : daemon de logs
- `waydash` : dashboard Wayland

Pour les télécharger :
1. Aller sur l'onglet "Actions" du repo GitHub
2. Sélectionner un workflow run réussi
3. Télécharger les artifacts dans la section "Artifacts"

## 🔧 Configuration

### Variables d'environnement

Le workflow utilise ces variables :
- `CARGO_TERM_COLOR=always` : coloration de la sortie
- `RUST_BACKTRACE=1` : backtraces détaillées en cas d'erreur

### Cache

Le pipeline utilise le cache GitHub Actions pour :
- `~/.cargo/registry` : registry cargo
- `~/.cargo/git` : dépendances git
- `target/` : artefacts de compilation

Le cache est invalidé quand `Cargo.lock` change.

## 🐛 Résolution de problèmes

### Le formatage échoue

```bash
# Corriger localement
cargo fmt --all
git add .
git commit --amend
```

### Clippy échoue

```bash
# Voir les warnings localement
cargo clippy --all-targets --all-features

# Corriger et tester
# ... faire les corrections ...
cargo clippy --all-targets --all-features -- -D warnings
```

### Les tests échouent

```bash
# Lancer les tests avec plus de détails
cargo test --all --verbose -- --nocapture

# Lancer un test spécifique
cargo test nom_du_test -- --nocapture
```

### Build Docker échoue

```bash
# Build en local pour voir les erreurs détaillées
docker build -f docker/ci.Dockerfile .

# Build seulement le stage test
docker build -f docker/ci.Dockerfile --target test -t test .
docker run --rm test
```

## 📊 Badges de statut

Ajoutez ces badges à votre README.md :

```markdown
![CI](https://github.com/CharlyMaye/cma-rust/workflows/CI/badge.svg)
```

## 🔄 Multi-architecture (futur)

Pour ajouter le support multi-architecture :

```yaml
strategy:
  matrix:
    target:
      - x86_64-unknown-linux-gnu
      - aarch64-unknown-linux-gnu
      - x86_64-pc-windows-gnu
```

Nécessite l'installation de `cross` :
```bash
cargo install cross
cross build --target aarch64-unknown-linux-gnu --release
```

## 📝 Bonnes pratiques

1. **Avant de push** : toujours lancer `make ci-local`
2. **Format** : utiliser `make fmt-fix` régulièrement
3. **Tests** : écrire des tests pour les nouvelles fonctionnalités
4. **Clippy** : corriger tous les warnings avant de commit
5. **Commits** : faire des commits atomiques et bien nommés

## 🔗 Liens utiles

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust CI with GitHub Actions](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)
- [actions-rs (deprecated but useful)](https://github.com/actions-rs)
