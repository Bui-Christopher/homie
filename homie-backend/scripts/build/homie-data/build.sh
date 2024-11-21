#!/usr/bin/env bash
set -e

SCRIPT_DIR="$( cd "$( dirname "$0" )" && pwd )"
cd "$SCRIPT_DIR"

if command -v podman &>/dev/null; then
    CONTAINER_TOOL="podman"
elif command -v docker &>/dev/null; then
    CONTAINER_TOOL="docker"
else
    echo "Neither Podman or Docker found, exiting..."
    exit 1
fi

IMAGE="homie-data"
BACKEND_DIR="$(git rev-parse --show-toplevel)/homie-backend"
VERSION=$(grep -E '^version\s*=' "$BACKEND_DIR/$IMAGE/Cargo.toml" | awk -F'"' '{print $2}')

REGISTRY="ghcr.io/bui-christopher/"
TAG="$REGISTRY$IMAGE:$VERSION"

"$CONTAINER_TOOL" build \
    -f Dockerfile "$BACKEND_DIR" \
    -t "$TAG" \
    --label "org.opencontainers.image.source=https://github.com/Bui-Christopher/homie" \
    --label "org.opencontainers.image.description=homie-data image"

"$CONTAINER_TOOL" tag "$TAG" "$REGISTRY$IMAGE:latest"
