#!/bin/bash
# Test de rotation des logs

set -e

echo "🧪 Test de rotation des logs loggerd"
echo "======================================"
echo ""

# Arrêter les instances existantes
pkill -9 -f loggerd 2>/dev/null || true
rm -f loggerd.log* 2>/dev/null || true

echo "1️⃣ Démarrage de loggerd avec limite de 500 bytes..."

# Modifier temporairement le fichier pour utiliser une petite taille
cd /workspaces/cma-rust

# Lancer loggerd en arrière-plan
nohup cargo run --package loggerd > /tmp/loggerd-test.log 2>&1 &
LOGGERD_PID=$!
echo "   PID: $LOGGERD_PID"

sleep 3

echo ""
echo "2️⃣ Génération de logs via requêtes HTTP..."
for i in {1..50}; do
    curl -s http://localhost:8080/metrics > /dev/null
    echo -n "."
    sleep 0.1
done
echo " ✓"

echo ""
echo "3️⃣ Vérification des fichiers de log..."
ls -lh loggerd.log* 2>/dev/null || echo "   Aucun fichier de backup créé (taille non atteinte)"

echo ""
echo "4️⃣ Contenu du fichier principal (dernières 5 lignes):"
echo "─────────────────────────────────────────────────────────"
tail -5 loggerd.log 2>/dev/null || echo "Fichier vide"
echo "─────────────────────────────────────────────────────────"

echo ""
echo "5️⃣ Statistiques:"
LOG_SIZE=$(stat -c%s loggerd.log 2>/dev/null || echo "0")
LOG_LINES=$(wc -l < loggerd.log 2>/dev/null || echo "0")
echo "   Taille du log: $LOG_SIZE bytes"
echo "   Nombre de lignes: $LOG_LINES"

echo ""
echo "6️⃣ Métriques finales du daemon:"
curl -s http://localhost:8080/metrics | python3 -m json.tool 2>/dev/null || curl -s http://localhost:8080/metrics

echo ""
echo ""
echo "7️⃣ Arrêt gracieux (SIGTERM)..."
kill -TERM $LOGGERD_PID
sleep 2

echo ""
echo "8️⃣ Logs du daemon:"
echo "─────────────────────────────────────────────────────────"
tail -20 loggerd.log
echo "─────────────────────────────────────────────────────────"

echo ""
echo "✅ Test terminé !"
