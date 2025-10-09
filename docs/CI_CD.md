# CI/CD Pipeline

## � Principe

**Tout le CI s'exécute dans Docker via `docker/ci.Dockerfile`**

Le Dockerfile fait :
1. `cargo fmt --check` → vérifie formatage
2. `cargo clippy -- -D warnings` → lint strict
3. `cargo test --all` → tous les tests
4. `cargo build --release` → compile

Si une étape échoue → build échoue → CI rouge ❌

## 🚀 Déclenché sur

- Push vers `main`, `develop`, `cma/**`
- Pull Requests vers `main`, `develop`

## 📦 Résultat

- Binaires : `loggerd`, `waydash` (téléchargeables)
- Images Docker : `cma-rust-loggerd`, `cma-rust-waydash`

## � Tester localement

```bash
# Avec Docker (identique au CI)
docker build -f docker/ci.Dockerfile .

# Ou avec Make
make docker-build

# Sans Docker (plus rapide)
make ci-local
```

## ⚙️ Performance

- **Première fois** : ~8-10 min
- **Avec cache** : ~3-5 min
