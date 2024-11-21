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

IMAGE="homie-api"
BACKEND_DIR="$(git rev-parse --show-toplevel)/homie-backend"

VERSION=$(grep -E '^version\s*=' "$BACKEND_DIR/$IMAGE/Cargo.toml" | awk -F'"' '{print $2}')
TAG="$IMAGE:$VERSION"

CONTAINER_REGISTRY="ghcr.io/Bui-Christopher/"

"$CONTAINER_TOOL" push "$CONTAINER_REGISTRY$IMAGE:$TAG"
