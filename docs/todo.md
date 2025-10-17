TODO.md — Rust/Linux/Wayland Journey
🎯 Global Objective

Master low-level Rust and the Linux ecosystem (systemd, snap, containers).

Build a robust and packaged Rust daemon (loggerd).

Create a Wayland Rust UI (waydash) displaying daemon metrics.

Open source publication: code, documentation, library, article, contributions.


🧰 Prerequisites and Environment

Host system: ArchLinux

Build/test containers: Ubuntu 24.04 (Docker)

Language: Rust stable + nightly

Recommended editor: VSCode or Helix + Rust-Analyzer

CI: GitHub Actions

Packaging: Snapcraft

Graphics stack: Wayland (winit, egui, smithay-client-toolkit)

🧭 Overview
Week	Main Theme	Key Deliverable
1	Pro Rust Setup & CI	Docker environment + stable CI
2	System daemon (loggerd)	Rust service + systemd
3	Ubuntu packaging	Functional Snap
4	Wayland UI (xdg-shell)	Basic visual dashboard
5	Advanced Wayland UI (layer-shell)	OSD/panel for wlroots
6	Translation library	Published on crates.io
7	Observability & tests	Metrics, perf, CI
8	Open source & article	Public PR + article
9	Final "showcase" project	Complete set ready to publish
10	Resume folder, cover letter, portfolio
🗓️ Week 1 — Pro Rust Setup & CI

🎯 Objective: Structure your professional Rust/Linux environment.

✅ Tasks:

Install Rust toolchains:

rustup default stable
rustup component add rustfmt clippy


Create a multi-stage Ubuntu 24.04 Dockerfile (build/test).

Add a GitHub Actions workflow:

cargo fmt --check

cargo clippy -- -D warnings

cargo test

release build (multi-arch with cross).

Create repository:

/loggerd
/waydash
/translation-lib
/docs
TODO.md


💡 Deliverable:
Ubuntu Dockerfile + complete functional CI pipeline.

🗓️ Semaine 2 — Daemon Rust : loggerd

🎯 Objectif : Transformer ta lib de logs en service système Rust.

✅ Tâches :

Créer un binaire loggerd :

Base sur ta lib de logs existante.

Support fichiers + console.

Rotation des logs.

Gestion signaux (SIGTERM, SIGHUP).

Ajouter une API HTTP avec axum :

/metrics → JSON (nb logs, uptime…)

/health → 200 OK

Créer un fichier loggerd.service pour systemd.

💡 Livrable :
Daemon Rust exécutable, fonctionnant via systemd.

🗓️ Semaine 3 — Packaging Ubuntu (Snapcraft)

🎯 Objectif : Préparer loggerd.

✅ Tâches :

Conteneur Ubuntu 24.04 :

FROM ubuntu:24.04
RUN apt update && apt install -y build-essential curl snapcraft


Snapcraft minimal :

name: loggerd
base: core24
version: '0.1'
summary: Rust system logger daemon
description: Lightweight system daemon for logging
parts:
  loggerd:
    plugin: rust
    source: .
apps:
  loggerd:
    command: bin/loggerd


Test : snapcraft build + exécution sous Ubuntu.

💡 Livrable :
Fichier .snap fonctionnel + instructions d’installation.

🗓️ Semaine 4 — UI Wayland (xdg-shell, waydash)

🎯 Objectif : Créer un client visuel Rust natif Wayland.

✅ Tâches :

Nouveau binaire waydash :

Fenêtre via winit / eframe.

UI via egui.

Requête HTTP vers loggerd (reqwest).

Affichage CPU/RAM/logs sous forme de jauges et graphiques.

Test sur Wayland (GNOME, Sway, Hyprland).

📦 Dépendances :

sudo pacman -S pkgconf wayland wayland-protocols libxkbcommon
cargo add eframe egui reqwest tokio


💡 Livrable :
UI Rust Wayland (waydash) affichant les métriques système.

🗓️ Semaine 5 — UI avancée (layer-shell, OSD)

🎯 Objectif : Faire fonctionner waydash en barre/OSD.

✅ Tâches :

Ajouter un mode --layer-shell :

smithay-client-toolkit pour protocole layer-shell.

Affichage non flottant (barre haut/bas).

Test sous wlroots (Sway, Hyprland).

Fallback en xdg-shell pour GNOME/KDE.

Thème clair/sombre, scaling DPI.

💡 Livrable :
waydash --layer-shell fonctionne comme un panneau sur Wayland wlroots.

🗓️ Semaine 6 — Lib de traduction (crates.io)

🎯 Objectif : Finaliser et publier ta lib de traduction Rust.

✅ Tâches :

Tests unitaires + intégration.

Formats : JSON/YAML, fallback en-US.

Exemples (examples/demo.rs).

Publier sur crates.io.

Ajouter badges (crates.io, docs.rs).

💡 Livrable :
Lib de traduction publiée et documentée.

🗓️ Semaine 7 — Observabilité & performance

🎯 Objectif : Mesurer et fiabiliser.

✅ Tâches :

Ajouter métriques :

prometheus_exporter dans loggerd.

/metrics compatible Prometheus.

Tests de charge (ab, hey, wrk).

Profilage (cargo flamegraph, perf).

Log structuré JSON.

💡 Livrable :
Services instrumentés, stables, mesurables.

🗓️ Semaine 8 — Open source & communication

🎯 Objectif : Montrer ta participation à la communauté.

✅ Tâches :

Contribuer à un projet Rust/Linux :

nix, systemd-rs, smithay-client-toolkit, winit.

Petite PR (doc, test, bugfix).

Rédiger un article :

“Rust on Wayland: Building a system dashboard and daemon from scratch”

Pourquoi Rust + Linux

Architecture loggerd/waydash

Snap packaging

Lessons learned

💡 Livrable :
Article public + PR open source visibles sur GitHub.

🗓️ Semaine 9 — Projet “showcase” complet

🎯 Objectif : Fusionner tous les éléments dans un seul projet.

✅ Tâches :

loggerd + waydash intégrés.

API REST + UI temps réel.

Build CI complet (tests, build, snap).

Docs + schéma d’architecture (docs/ARCHITECTURE.md).

GIFs/screenshots.

💡 Livrable :
Projet complet, démonstratif et présentable.

🧩 Architecture globale
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

📦 Commandes utiles
Lancer un client Wayland dans Docker
docker run --rm -it \
  -e WAYLAND_DISPLAY=$WAYLAND_DISPLAY \
  -e XDG_RUNTIME_DIR=$XDG_RUNTIME_DIR \
  -v $XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:$XDG_RUNTIME_DIR/$WAYLAND_DISPLAY \
  -v $XDG_RUNTIME_DIR:$XDG_RUNTIME_DIR \
  --net=host ubuntu:24.04 bash

Snap minimal pour waydash
name: waydash
base: core24
version: '0.1'
summary: Rust Wayland system dashboard
description: Wayland dashboard (xdg-shell/layer-shell) consuming loggerd metrics.
grade: devel
confinement: classic

parts:
  waydash:
    plugin: rust
    source: .
    build-packages:
      - pkg-config
      - libxkbcommon-dev
      - wayland-protocols
      - libwayland-dev
apps:
  waydash:
    command: bin/waydash

🏁 Fin du parcours

Objectif final atteint quand :

✅ loggerd tourne sous Ubuntu via Snap & systemd.

✅ waydash affiche les métriques via Wayland.

✅ Lib de traduction publiée.

✅ CI/CD complet.

✅ Article + PR open source en ligne.

