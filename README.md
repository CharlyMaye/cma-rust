# CMA-Rust

[![CI](https://github.com/CharlyMaye/cma-rust/workflows/CI/badge.svg)](https://github.com/CharlyMaye/cma-rust/actions)

> 🦀 Rust Learning Journey — Système, Wayland & Open Source

Projet complet d'apprentissage Rust bas-niveau avec construction d'un démon système (`loggerd`) et d'un dashboard Wayland (`waydash`).

## 📦 Projets du Workspace

Ce workspace contient plusieurs crates Rust :

| Crate | Description | Statut |
|-------|-------------|--------|
| **loggerd** | Démon système de logging avec rotation + API REST | ✅ Fonctionnel |
| **waydash** | Dashboard Wayland pour afficher les métriques | 🚧 Planifié |
| **translation-lib** | Bibliothèque i18n pour Rust | 📦 À publier |
| **traces** | Bibliothèque de logging personnalisée avec Rx patterns | 🎓 En cours |
| **rustlings** | Exercices Rust (apprentissage) | 🎓 En cours |

## 🚀 Quick Start

### Prérequis

```bash
# Installation de Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt clippy

# Sur Ubuntu/Debian
sudo apt-get install -y \
  pkg-config libssl-dev libwayland-dev wayland-protocols \
  libxkbcommon-dev libx11-dev libxcursor-dev libxrandr-dev \
  libxi-dev libgl1-mesa-dev libegl1-mesa-dev \
  libudev-dev libdbus-1-dev
```

### Build & Run

```bash
# Build tous les projets
make build

# Lancer les tests
make test

# Vérifications complètes (format + clippy + test)
make check

# Build release
make release
```

### Lancer les applications

```bash
# Démon loggerd (API REST sur port 8080)
make run-loggerd

# Tester les endpoints
make health-check    # GET /health
make metrics         # GET /metrics (JSON)

# Arrêter proprement loggerd
make stop-loggerd    # Envoie SIGTERM (graceful shutdown)

# Dashboard waydash (interface graphique Wayland)
make run-waydash     # À venir (Semaine 4)
```

## 🔄 CI/CD

**🐳 Le CI s'exécute entièrement dans Docker pour garantir la reproductibilité.**

Le pipeline GitHub Actions utilise `docker/ci.Dockerfile` et automatise :
- ✅ Vérification du formatage (`cargo fmt --check`)
- ✅ Linting avec Clippy (mode strict : `-D warnings`)
- ✅ Tests unitaires et d'intégration
- ✅ Build release optimisé
- ✅ Génération des images Docker runtime

### Tester le CI localement

```bash
# Avec Docker (identique au CI GitHub)
make docker-build

# Sans Docker (plus rapide)
make ci-local
```

**Temps d'exécution** :
- Docker (première fois) : ~8-10 min
- Docker (avec cache) : ~3-5 min  
- Local sans Docker : ~2 min

## 🐳 Docker

### Build avec Docker

```bash
# Build l'image CI complète (avec tests)
make docker-build

# Build image runtime loggerd
make docker-build-loggerd

# Build image runtime waydash
make docker-build-waydash
```

### Lancer dans Docker

```bash
# Loggerd (API REST)
make docker-run-loggerd

# Waydash (nécessite accès au socket Wayland)
make docker-run-waydash
```

## 📚 Documentation

- **[CI/CD](docs/CI_CD.md)** : Documentation complète du pipeline CI/CD
- **[TODO](docs/todo.md)** : Feuille de route du projet (10 semaines)

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

## 🛠️ Commandes utiles

```bash
# Aide (affiche toutes les commandes disponibles)
make help

# Build & test
make build              # Build debug
make release            # Build release
make test               # Tests
make fmt                # Vérifier formatage
make fmt-fix            # Corriger formatage
make clippy             # Linter
make clean              # Nettoyer

# Développement
make watch-loggerd      # Auto-reload loggerd
make watch-waydash      # Auto-reload waydash
make doc                # Générer docs
make tree               # Arbre dépendances

# Installation
make install            # Installer dans ~/.cargo/bin
```

## 🎯 Objectifs du projet

### Semaine 1 : Setup & CI ✅
- [x] Dockerfile Ubuntu 24.04 multi-stage
- [x] Pipeline GitHub Actions complet
- [x] Makefile pour automatisation
- [x] Documentation CI/CD

### Semaine 2 : Daemon Rust (loggerd) ✅
- [x] Binaire loggerd avec API HTTP (Axum)
- [x] Endpoints `/health` et `/metrics`
- [x] Système de traces personnalisé (console + fichier)
- [x] Rotation automatique des logs (par taille + timestamp)
- [x] Gestion des signaux Unix (SIGTERM, SIGHUP)
- [x] Graceful shutdown
- [x] Compteur de logs thread-safe (AtomicU64)
- [x] Refactoring modulaire (SRP, Clean Architecture)
- [x] Documentation complète avec doctests
- [x] Fichier systemd unit

**Documentation** :
- [Récapitulatif Semaine 2](docs/semaine2-recap.md)
- [Architecture du module file](docs/architecture-file-module.md)
- [Refactoring file_trace_handlers](docs/refactoring-file-handler.md)

### Semaines suivantes
- [ ] Semaine 3 : Packaging Ubuntu (Snapcraft)
- [ ] Semaine 4 : UI Wayland (xdg-shell)
- [ ] Semaine 5 : UI avancée (layer-shell)
- [ ] Semaine 6 : Publication lib de traduction
- [ ] Semaine 7 : Observabilité & performance
- [ ] Semaine 8 : Open source & article
- [ ] Semaine 9 : Projet showcase complet

Voir [docs/todo.md](docs/todo.md) pour le détail complet.

## 🔧 Stack technique

- **Langage** : Rust (stable)
- **Build** : Cargo workspace
- **CI/CD** : GitHub Actions
- **Containers** : Docker multi-stage
- **Web** : Axum (API REST)
- **GUI** : egui + winit (Wayland)
- **Packaging** : Snap (Ubuntu)
- **Système** : systemd, journald

## 📊 Métriques

![CI Status](https://img.shields.io/github/actions/workflow/status/CharlyMaye/cma-rust/ci.yml?branch=main)
![Rust Version](https://img.shields.io/badge/rust-stable-blue)
![License](https://img.shields.io/badge/license-MIT-green)

## 🤝 Contribuer

Les contributions sont les bienvenues ! Ce projet fait partie d'un parcours d'apprentissage, n'hésitez pas à :
- Proposer des améliorations
- Signaler des bugs
- Partager des idées

## 📝 License

MIT License - voir [LICENSE](LICENSE) pour plus de détails.

## 👤 Auteur

**CharlyMaye**

- GitHub: [@CharlyMaye](https://github.com/CharlyMaye)
- Projet : Parcours Rust/Linux/Wayland

---

**Note** : Ce projet est actuellement en développement actif dans le cadre d'un parcours d'apprentissage de 10 semaines sur Rust système et Linux.
