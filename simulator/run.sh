#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

mkdir -p output

echo "Building whiskey-simulator Docker image..."
docker build -t whiskey-sim .

CONFIG="${1:-example-configs/barrel-traditional.json}"
CONFIG_BASENAME="$(basename "$CONFIG")"

# Copy config into output so the container can read it
cp "$CONFIG" "output/$CONFIG_BASENAME"

echo "Running simulation with config: $CONFIG"
docker run --rm \
  -v "$(pwd)/output:/output" \
  whiskey-sim "/output/$CONFIG_BASENAME"

echo ""
echo "Done. Results in output/"
ls -lh output/*.json output/*.svg 2>/dev/null || true
