TODO.md — Parcours Rust/Linux/Wayland
🎯 Objectif global

Maîtrise du Rust bas-niveau et du monde Linux (systemd, snap, containers).

Construction d’un démon Rust robuste et packagé (loggerd).

Création d’une UI Wayland Rust (waydash) affichant les métriques du démon.

Publication open source : code, doc, lib, article, contributions.


🧰 Prérequis et environnement

Système hôte : ArchLinux

Conteneurs de build/test : Ubuntu 24.04 (Docker)

Langage : Rust stable + nightly

Éditeur recommandé : VSCode ou Helix + Rust-Analyzer

CI : GitHub Actions

Packaging : Snapcraft

Stack graphique : Wayland (winit, egui, smithay-client-toolkit)

🧭 Vue d’ensemble
Semaine	Thème principal	Livrable clé
1	Setup & CI Rust pro	Environnement Docker + CI stable
2	Daemon système (loggerd)	Service Rust + systemd
3	Packaging Ubuntu	Snap fonctionnel
4	UI Wayland (xdg-shell)	Dashboard visuel basique
5	UI Wayland avancée (layer-shell)	OSD/panel pour wlroots
6	Lib de traduction	Publiée sur crates.io
7	Observabilité & tests	Metrics, perf, CI
8	Open source & article	PR publique + article
9	Projet final “showcase”	Ensemble complet prêt à publier
10	Dossier CV, cover letter, portfolio
🗓️ Semaine 1 — Setup & CI Rust pro

🎯 Objectif : Structurer ton environnement pro Rust/Linux.

✅ Tâches :

Installer toolchains Rust :

rustup default stable
rustup component add rustfmt clippy


Créer un Dockerfile Ubuntu 24.04 multi-stage (build/test).

Ajouter un workflow GitHub Actions :

cargo fmt --check

cargo clippy -- -D warnings

cargo test

build release (multi-arch avec cross).

Créer repo :

/loggerd
/waydash
/translation-lib
/docs
TODO.md


💡 Livrable :
Dockerfile Ubuntu + pipeline CI complet fonctionnel.

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

