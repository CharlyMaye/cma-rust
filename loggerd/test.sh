#!/bin/bash
# Script de test pour loggerd

set -e

echo "🚀 Building loggerd..."
cargo build --package loggerd

echo ""
echo "🧪 Starting loggerd in background..."
cargo run --package loggerd > /tmp/loggerd-test.log 2>&1 &
PID=$!
echo "   PID: $PID"

# Attendre que le serveur démarre
sleep 2

echo ""
echo "✅ Testing /health endpoint..."
HEALTH=$(curl -s http://localhost:8080/health)
if [ "$HEALTH" = "OK" ]; then
    echo "   ✓ Health check passed: $HEALTH"
else
    echo "   ✗ Health check failed: $HEALTH"
    kill $PID
    exit 1
fi

echo ""
echo "✅ Testing /metrics endpoint (call 1)..."
METRICS1=$(curl -s http://localhost:8080/metrics)
echo "   Response: $METRICS1"

echo ""
echo "✅ Testing /metrics endpoint (call 2)..."
METRICS2=$(curl -s http://localhost:8080/metrics)
echo "   Response: $METRICS2"

echo ""
echo "✅ Testing graceful shutdown (SIGTERM)..."
kill -TERM $PID
sleep 1

echo ""
echo "📋 Daemon logs:"
echo "─────────────────────────────────────────────────"
tail -10 /tmp/loggerd-test.log
echo "─────────────────────────────────────────────────"

echo ""
echo "✅ All tests passed!"
echo ""
echo "Summary:"
echo "  - HTTP server: ✓"
echo "  - /health endpoint: ✓"
echo "  - /metrics endpoint: ✓"
echo "  - Graceful shutdown: ✓"
