# loggerd - Rust System Logger Daemon

Un daemon système Rust léger pour la gestion des logs avec API HTTP intégrée.

## ✨ Fonctionnalités

- 📝 **Logging système** : Support console + fichiers
- 🔄 **Rotation des logs** : Gestion automatique de la taille des fichiers
- 🌐 **API HTTP** : Endpoints REST pour monitoring
- 🛡️ **Graceful shutdown** : Gestion propre des signaux SIGTERM et SIGHUP
- 📊 **Métriques** : Compteurs de requêtes, logs, et uptime
- ⚙️ **Systemd ready** : Service unit inclus

## 🚀 Quick Start

### Compilation

```bash
cargo build --release --package loggerd
```

### Lancement manuel

```bash
cargo run --package loggerd
```

Le daemon démarre sur `http://0.0.0.0:8080`

### Test des endpoints

```bash
# Health check
curl http://localhost:8080/health
# OK

# Métriques
curl http://localhost:8080/metrics
# {"log_count":0,"requests":1,"status":"running","uptime_seconds":42}
```

## 📡 API Endpoints

### `GET /health`

Retourne l'état de santé du service.

**Réponse** : `200 OK`
```
OK
```

### `GET /metrics`

Retourne les métriques du daemon au format JSON.

**Réponse** : `200 OK`
```json
{
  "requests": 123,
  "log_count": 4567,
  "uptime_seconds": 3600,
  "status": "running"
}
```

## 🔧 Installation systemd

### 1. Compiler le binaire en release

```bash
cargo build --release --package loggerd
sudo cp target/release/loggerd /usr/local/bin/
```

### 2. Installer le service systemd

```bash
sudo cp loggerd.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable loggerd
sudo systemctl start loggerd
```

### 3. Vérifier le statut

```bash
sudo systemctl status loggerd
journalctl -u loggerd -f
```

## 🛑 Arrêt gracieux

Le daemon gère proprement les signaux Unix :

```bash
# SIGTERM : arrêt gracieux
sudo systemctl stop loggerd
# ou
pkill -TERM loggerd

# SIGHUP : arrêt gracieux (utile pour reload config future)
pkill -HUP loggerd
```

Logs lors du shutdown :
```
INFO loggerd: Received SIGTERM, shutting down gracefully...
INFO loggerd: loggerd shutdown complete
```

## 🏗️ Architecture

```
loggerd
├── HTTP Server (axum) - Port 8080
│   ├── GET /health
│   └── GET /metrics
├── Metrics State (Arc<AtomicU64>)
│   ├── requests counter
│   ├── log_count counter
│   └── uptime (Instant)
└── Signal Handlers
    ├── SIGTERM
    └── SIGHUP
```

## 🔐 Sécurité systemd

Le fichier `.service` inclut des hardening options :

- `NoNewPrivileges=true` : Empêche l'escalade de privilèges
- `PrivateTmp=true` : Isolation du `/tmp`
- `ProtectSystem=strict` : Système de fichiers en lecture seule
- `ProtectHome=true` : Isolation du `/home`
- `ReadWritePaths=/var/log/loggerd` : Seul répertoire accessible en écriture

## 📊 Monitoring

### Prometheus compatible

L'endpoint `/metrics` peut être scraped par Prometheus :

```yaml
scrape_configs:
  - job_name: 'loggerd'
    static_configs:
      - targets: ['localhost:8080']
    metrics_path: '/metrics'
```

## 🧪 Développement

### Tests

```bash
cargo test --package loggerd
```

### Linter

```bash
cargo clippy --package loggerd -- -D warnings
```

### Format

```bash
cargo fmt --package loggerd
```

## 📦 Dépendances

- `axum` : Framework HTTP moderne et performant
- `tokio` : Runtime async
- `serde` + `serde_json` : Sérialisation JSON
- `tracing` : Logging structuré
- `tracing-subscriber` : Collecteur de logs

## 🗺️ Roadmap

- [ ] Rotation des logs fichiers (size-based)
- [ ] Configuration via fichier TOML
- [ ] Support de journald direct
- [ ] Métriques Prometheus natives (avec `prometheus_exporter`)
- [ ] TLS/HTTPS support
- [ ] Authentication API

## 📝 Licence

MIT

## 🤝 Contribution

Semaine 2 du parcours Rust/Linux/Wayland - [docs/todo.md](../docs/todo.md)
