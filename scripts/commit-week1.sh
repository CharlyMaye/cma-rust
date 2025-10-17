#!/bin/bash
set -e

echo "🚀 Commit Week 1 - CI/CD"
echo ""

# Display files
git status --short

echo ""
read -p "Commit? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
fi

# Add files
git add .github/ Makefile README.md docker/ci.Dockerfile docs/CI_CD.md
git add loggerd/ waydash/ traces/ rustlings/

# Commit
git commit -m "feat: Docker CI pipeline (Week 1)

- 100% Docker CI with ci.Dockerfile
- Builder stage: fmt + clippy + test + build
- Extract binaries with outputs=type=local
- Runtime images: loggerd + waydash
- Fix all clippy warnings
- Format all code"

echo "✅ Done"
echo ""
read -p "Push? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git push origin $(git branch --show-current)
    echo "✅ Pushed"
fi
